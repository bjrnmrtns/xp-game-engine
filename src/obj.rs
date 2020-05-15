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
    v_index: u32,
}

fn parse_face(face: &str) -> ObjResult<Face> {
    let mut indices = face.split("/");
    let first: u32 = indices.next().unwrap_or("").parse()?;
    Ok(Face { v_index: (first - 1), })
}

pub fn parse_obj<R>(reader: R) -> ObjResult<(Vec<[f32; 3]>, Vec<u32>)>
    where R: std::io::BufRead {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    parse(reader, |obj_type, args| {
        match obj_type {
            "v" => {
                let args = str_args_to_f32!(args);
                positions.push(match args.len() {
                    3 => [args[0], args[1], args[2]],
                    4 => [args[0] / args[3], args[1] / args[3], args[2] / args[3]],
                    _ => return Err(ObjError::WrongNumberOfArguments),
                });
            }
            "f" => {
                if args.len() < 3 {
                    return Err(ObjError::WrongNumberOfArguments);
                }
                indices.push(parse_face(args[0])?.v_index);
                indices.push(parse_face(args[1])?.v_index);
                indices.push(parse_face(args[2])?.v_index);
            }
            _ => ()
        }
        Ok(())
    })?;
    Ok((positions, indices))
}
