#[derive(Debug)]
pub enum RendererError {
    String(String),
    IOError(std::io::Error),
    RequestAdapter,
}

impl From<String> for RendererError {
    fn from(e: String) -> RendererError {
        RendererError::String(e)
    }
}

impl From<std::io::Error> for RendererError {
    fn from(e: std::io::Error) -> RendererError {
        RendererError::IOError(e)
    }
}
