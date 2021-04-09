use crate::gltf::MeshLoadError;

#[derive(Debug)]
pub enum WorldLoadError {
    MeshLoadError(MeshLoadError),
}

impl From<MeshLoadError> for WorldLoadError {
    fn from(e: MeshLoadError) -> WorldLoadError {
        WorldLoadError::MeshLoadError(e)
    }
}
