use nalgebra_glm::*;

pub type ObjResult<T> = std::result::Result<T, ObjError>;

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

struct Face {
    v_index: usize,
    t_index: usize,
    n_index: usize,
}

fn parse_face(face: &str) -> ObjResult<Face> {
    let mut indices = face.split("/");
    let first = indices.next().unwrap_or("");
    let second = indices.next().unwrap_or("");
    let third = indices.next().unwrap_or("");

    let first: i32 = first.parse()?;
    let second: i32 = if second == "" {
        1
    } else {
        second.parse()?
    };
    let third: i32 = if third == "" {
        1
    } else {
        third.parse()?
    };
    Ok(Face { v_index: (first - 1) as usize, t_index: (second - 1) as usize, n_index: (third - 1) as usize })
}

pub fn parse_obj<R>(reader: R) -> ObjResult<Vec<[(Vec3, (Vec3, Vec2)); 3]>>
    where R: std::io::BufRead {
    let mut positions = Vec::new();
    let mut tex_coords = Vec::new();
    let mut normals = Vec::new();
    let mut faces = Vec::new();
    parse(reader, |obj_type, args| {
        match obj_type {
            "v" => {
                let args = str_args_to_f32!(args);
                positions.push(match args.len() {
                    3 => Vec3::new(args[0], args[1], args[2]),
                    4 => Vec3::new(args[0] / args[3], args[1] / args[3], args[2] / args[3]),
                    _ => return Err(ObjError::WrongNumberOfArguments),
                });
            }
            "vt" => {
                let args = str_args_to_f32!(args);
                tex_coords.push(match args.len() {
                    1 => Vec2::new(args[0], 0.0),
                    2 => Vec2::new(args[0], args[1]),
                    3 => Vec2::new(args[0], args[1]),
                    _ => return Err(ObjError::WrongNumberOfArguments),
                });
            }
            "vn" => {
                let args = str_args_to_f32!(args);
                normals.push(match args.len() {
                    3 => Vec3::new(args[0], args[1], args[2]),
                    _ => return Err(ObjError::WrongNumberOfArguments),
                });
            }
            "f" => {
                if args.len() < 3 {
                    return Err(ObjError::WrongNumberOfArguments);
                }
                let facev0 = parse_face(args[0])?;
                let facev1 = parse_face(args[1])?;
                let facev2 = parse_face(args[2])?;
                faces.push((facev0, facev1, facev2));
            }
            _ => ()
        }
        Ok(())
    })?;
    let mut vertices = Vec::new();
    for face in faces {
        let vertex0 = (positions[face.0.v_index],
                                        (normals[face.0.n_index], tex_coords[face.0.t_index]));
        let vertex1 = (positions[face.1.v_index],
                                        (normals[face.1.n_index], tex_coords[face.1.t_index]));

        let vertex2 = (positions[face.2.v_index],
                                        (normals[face.2.n_index], tex_coords[face.2.t_index]));

        vertices.push([vertex0, vertex1, vertex2] )
    }
    Ok(vertices)
}
