use tobj::LoadError;

#[derive(Debug)]
pub enum MeshError {
    LoadError(LoadError),
    IOError(std::io::Error),
    RequestAdapter,
}

impl From<LoadError> for MeshError {
    fn from(e: LoadError) -> MeshError {
        MeshError::LoadError(e)
    }
}
