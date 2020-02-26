type ObjResult<T> = std::result::Result<T, ObjError>;

macro_rules! str_args_to_f32 {
    ($args:expr) => (
        &{
            let mut ret = Vec::<f32>::new();
            ret.reserve($args.len());
            for arg in $args {
                ret.push(arg.parse()?);
            }
            ret
        }[..]
    )
}

#[derive(Debug)]
pub enum ObjError {
    Io(std::io::Error),
    ParseFloat(std::num::ParseFloatError),
    ParseInt(std::num::ParseIntError),
    WrongNumberOfArguments,
}

impl std::error::Error for ObjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ObjError::Io(ref e) => Some(e),
            ObjError::ParseFloat(ref e) => Some(e),
            ObjError::ParseInt(ref e) => Some(e),
            ObjError::WrongNumberOfArguments => None,
        }
    }
}

impl std::fmt::Display for ObjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ObjError::Io(ref e) => e.fmt(f),
            ObjError::ParseFloat(ref e) => e.fmt(f),
            ObjError::ParseInt(ref e) => e.fmt(f),
            ObjError::WrongNumberOfArguments => write!(f, "wrong number of arguments"),

        }
    }
}

impl From<std::io::Error> for ObjError {
    fn from(e: std::io::Error) -> ObjError {
            ObjError::Io(e)
    }
}

impl From<std::num::ParseFloatError> for ObjError {
    fn from(e: std::num::ParseFloatError) -> ObjError {
        ObjError::ParseFloat(e)
    }
}

impl From<std::num::ParseIntError> for ObjError {
    fn from(e: std::num::ParseIntError) -> ObjError {
        ObjError::ParseInt(e)
    }
}

use crate::vec::{Vec2, Vec3};

pub fn parse<R, C>(reader: R, mut converter: C) -> ObjResult<()>
    where R: std::io::BufRead, C: FnMut(&str, &Vec<&str>) -> ObjResult<()> {
    let mut line = String::new();
    for current_line in reader.lines() {
        let current_line = current_line?;
        let current_line = current_line.split('#').next().unwrap();
        if current_line.ends_with("\\") {
            line.push_str(&current_line[0..current_line.len() - 1]);
            line.push(' ');
            continue;
        }
        line.push_str(current_line);
        let mut splitted = line.split_whitespace();
        if let Some(obj_type) = splitted.next() {
            let mut args : Vec<&str> = Vec::new();
            for arg in splitted {
                args.push(&arg);
            }
            converter(obj_type, &args)?;
        }
        line.clear();
    }
    Ok(())
}

fn parse_face(face: &str) -> ObjResult<(i32, i32, i32)> {
    let mut indices = face.split("/");
    let first = indices.next().unwrap_or("");
    let second = indices.next().unwrap_or("");
    let third = indices.next().unwrap_or("");

    let first = first.parse()?;
    let second = if second == "" {
        0
    } else {
        second.parse()?
    };
    let third = if third == "" {
        0
    } else {
        third.parse()?
    };
    Ok((first, second, third))
}

pub fn parse_obj<R>(reader: R) -> ObjResult<Vec<(Vec3<f32>, Vec3<f32>, Vec3<f32>)>>
    where R: std::io::BufRead {
    let mut vertices = Vec::new();
    vertices.push((Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec2::new(0.0, 0.0)));
    let mut positions = Vec::new();
    let mut tex_coords = Vec::new();
    let mut normals = Vec::new();
    let mut faces = Vec::new();
    parse(reader, |obj_type, args| {
        match obj_type {
            "v" => {
                let args = str_args_to_f32!(args);
                positions.push(match args.len() {
                    3 => (args[0], args[1], args[2], 1.0),
                    4 => (args[0], args[1], args[2], args[3]),
                    _ => return Err(ObjError::WrongNumberOfArguments),
                });
            }
            "vt" => {
                let args = str_args_to_f32!(args);
                tex_coords.push(match args.len() {
                    1 => (args[0], 0.0, 0.0),
                    2 => (args[0], args[1], 0.0),
                    3 => (args[0], args[1], args[2]),
                    _ => return Err(ObjError::WrongNumberOfArguments),
                });
            }
            "vn" => {
                let args = str_args_to_f32!(args);
                normals.push(match args.len() {
                    3 => (args[0], args[1], args[2]),
                    _ => return Err(ObjError::WrongNumberOfArguments),
                });
            }
            "f" => {
                if args.len() < 3 {
                    return Err(ObjError::WrongNumberOfArguments);
                }
                let facev0 = parse_face(args[0])?;
                let facev1 = parse_face(args[1])?;
                let facev2 = parse_face(args[1])?;
                faces.push((facev0, facev1, facev2));
            }
            _ => ()
        }
        Ok(())
    })?;
    let mut vertices: Vec<(Vec3<f32>, Vec3<f32>, Vec3<f32>)> = Vec::new();
    for face in faces {
        vertices.push((Vec3::new(positions[(face.0).0 as usize], positions[(face.1).0 as usize], positions[(face.2).0 as usize]),
                      Vec3::new(tex_coords[(face.0).1 as usize], tex_coords[(face.1).1 as usize], tex_coords[(face.2).1 as usize]),
                      Vec3::new(normals[(face.0).2 as usize], normals[(face.1).2 as usize], normals[(face.2).2 as usize])));
    }
    Ok(vertices)
}
