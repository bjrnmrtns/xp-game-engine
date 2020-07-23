
#[derive(Debug)]
pub enum GraphicsError {
    String(String),
    IOError(std::io::Error),
    RequestAdapter,
}

impl From<String> for GraphicsError {
    fn from(e: String) -> GraphicsError {
        GraphicsError::String(e)
    }
}

impl From<std::io::Error> for GraphicsError {
    fn from(e: std::io::Error) -> GraphicsError {
        GraphicsError::IOError(e)
    }
}

