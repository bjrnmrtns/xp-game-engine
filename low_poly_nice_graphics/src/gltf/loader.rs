#[derive(Debug)]
pub enum GltfError {
    Gltf(gltf::Error),
    Decode(base64::DecodeError),
    NotSupportedUri,
    MissingBlob,
}

impl From<gltf::Error> for GltfError {
    fn from(e: gltf::Error) -> GltfError {
        GltfError::Gltf(e)
    }
}

impl From<base64::DecodeError> for GltfError {
    fn from(e: base64::DecodeError) -> GltfError {
        GltfError::Decode(e)
    }
}

pub fn load_gltf(bytes: &[u8]) -> Result<(), GltfError> {
    let gltf = gltf::Gltf::from_slice(bytes)?;
    for node in gltf.nodes() {
        println!("{}", node.name().unwrap());
        let mesh = node.mesh().unwrap();
        for primitive in mesh.primitives() {
            let mode = primitive.mode();
            //primitive.reader(|buffer| Some(&bu))
        }
    }
    Ok(())
}

fn load_buffers(gltf: &gltf::Gltf) -> Result<Vec<Vec<u8>>, GltfError> {
    const OCTET_STREAM_URI: &str = "data:application/octet-stream;base64,";
    let mut buffer_data = Vec::new();
    for buffer in gltf.buffers() {
        match buffer.source() {
            gltf::buffer::Source::Uri(uri) => {
                if uri.starts_with(OCTET_STREAM_URI) {
                    buffer_data.push(base64::decode(&uri[OCTET_STREAM_URI.len()..])?);
                } else {
                    return Err(GltfError::NotSupportedUri);
                }
            }
            gltf::buffer::Source::Bin => {
                if let Some(blob) = gltf.blob.as_deref() {
                    buffer_data.push(blob.into());
                } else {
                    return Err(GltfError::MissingBlob);
                }
            }
        }
    }

    Ok(buffer_data)
}

#[cfg(test)]
mod tests {
    use crate::gltf::loader::load_gltf;

    #[test]
    fn load_gltf_test() {
        load_gltf(std::fs::read("res/gltf/test.gltf").unwrap().as_slice());
    }
}
