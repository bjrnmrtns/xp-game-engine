type Result<T> = std::result::Result<T, ObjError>;

#[derive(Debug)]
pub enum ObjError {
    Io(std::io::Error),
    ParseFloat(std::num::ParseFloatError),
    WrongNumberOfArguments,
}

impl std::error::Error for ObjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ObjError::Io(ref e) => Some(e),
            ObjError::ParseFloat(ref e) => Some(e),
            ObjError::WrongNumberOfArguments => None,
        }
    }
}

impl std::fmt::Display for ObjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ObjError::Io(ref e) => e.fmt(f),
            ObjError::ParseFloat(ref e) => e.fmt(f),
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

use crate::vec::{Vec2, Vec3};

pub fn parse<R, C>(reader: R, mut converter: C) -> Result<()>
    where R: std::io::BufRead, C: FnMut(&str, &Vec<&str>) -> Result<()> {
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

pub fn parse_obj<R>(reader: R) -> Result<Vec<(Vec3<f32>, Vec3<f32>, Vec2<f32>)>>
    where R: std::io::BufRead {
    let mut vertices = Vec::new();
    vertices.push((Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec2::new(0.0, 0.0)));
    let mut positions = Vec::new();
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
            _ => unimplemented!()
        }
        Ok(())
    })?;
    Ok(vertices)
}
