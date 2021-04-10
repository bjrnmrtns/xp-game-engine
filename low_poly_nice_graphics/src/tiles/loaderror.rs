use crate::gltf::MeshLoadError;

#[derive(Debug)]
pub enum LoadError {
    MeshLoadError(MeshLoadError),
}

impl From<MeshLoadError> for LoadError {
    fn from(e: MeshLoadError) -> LoadError {
        LoadError::MeshLoadError(e)
    }
}
