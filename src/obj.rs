use std::fmt::{Formatter};
use crate::obj::ObjError::ParseObjError;

type Result<T> = std::result::Result<T, ObjError>;

#[derive(Debug)]
pub enum ObjError {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    ParseObjError,
}

impl From<std::num::ParseIntError> for ObjError {
    fn from(err: std::num::ParseIntError) -> ObjError {
        ObjError::ParseInt(err)
    }
}

impl From<std::num::ParseFloatError> for ObjError {
    fn from(err: std::num::ParseFloatError) -> ObjError {
        ObjError::ParseFloat(err)
    }
}

impl From<std::io::Error> for ObjError {
    fn from(err: std::io::Error) -> ObjError {
        ObjError::Io(err)
    }
}

impl std::fmt::Display for ObjError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            ObjError::Io(ref e) => e.fmt(f),
            ObjError::ParseInt(ref e) => e.fmt(f),
            ObjError::ParseFloat(ref e) => e.fmt(f),
            ObjError::ParseObjError => write!(f, "failed to parse obj"),
        }
    }
}

impl std::error::Error for ObjError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ObjError::Io(ref e) => Some(e),
            ObjError::ParseInt(ref e) => Some(e),
            ObjError::ParseFloat(ref e) => Some(e),
            ObjError::ParseObjError => None,

        }
    }
}

fn obj_lex<T, F>(input: T, mut callback: F) -> Result<()>
    where T: std::io::BufRead, F: std::ops::FnMut(&str, &[&str]) -> Result<()>
{
    let mut ml = String::new();
    for line in input.lines() {
        // Get line and remove everything after #.
        let line_with_comments = line.unwrap();
        let line = line_with_comments.split("#").next().unwrap();

        if line.ends_with('\\') {
            ml.push_str(&line[0..line.len()-1]);
            ml.push(' ');
            continue
        }
        ml.push_str(line);

        let mut tokens = ml.split_whitespace();
        if let Some(statement) = tokens.next() {
            let mut args = Vec::new();
            for token in tokens {
                args.push(token);
            }
            callback(&statement, &args)?;
        }
        ml.clear();
    }
    Ok(())
}

pub fn parse_faces(s: &str) -> Result<(i32, i32, i32)> {
    let mut indices = s.split('/');

    let first  = indices.next().unwrap_or("");
    let second = indices.next().unwrap_or("");
    let third  = indices.next().unwrap_or("");

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

pub struct Obj {
    pub positions: Vec<(f32, f32, f32, f32)>,
}