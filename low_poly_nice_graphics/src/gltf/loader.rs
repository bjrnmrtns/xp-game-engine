use crate::mesh::{triangle_normal, Mesh, Vertex};
use gltf::mesh::Mode;

#[derive(Debug)]
pub enum MeshLoadError {
    Gltf(gltf::Error),
    Decode(base64::DecodeError),
    UnsupportedBufferFormat,
    UnsupportedPrimitiveMode,
    MissingBlob,
}

impl From<gltf::Error> for MeshLoadError {
    fn from(e: gltf::Error) -> MeshLoadError {
        MeshLoadError::Gltf(e)
    }
}

impl From<base64::DecodeError> for MeshLoadError {
    fn from(e: base64::DecodeError) -> MeshLoadError {
        MeshLoadError::Decode(e)
    }
}

pub fn load_gltf(bytes: &[u8], mut named_mesh: impl FnMut(String, Mesh)) -> Result<(), MeshLoadError> {
    let gltf = gltf::Gltf::from_slice(bytes)?;
    let buffer_data = load_buffers(&gltf)?;
    for node in gltf.nodes() {
        if let Some(node_name) = node.name() {
            let mesh = node.mesh().unwrap();
            let mut vertices = Vec::new();
            for primitive in mesh.primitives() {
                if primitive.mode() != Mode::Triangles {
                    return Err(MeshLoadError::UnsupportedPrimitiveMode);
                }
                let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

                if let Some(positions) = reader.read_positions().map(|v| v.collect::<Vec<[f32; 3]>>()) {
                    if let Some(indices) = reader
                        .read_indices()
                        .map(|indices| indices.into_u32().collect::<Vec<u32>>())
                    {
                        assert!(indices.len() % 3 == 0);
                        for i in indices.chunks(3) {
                            let v0 = positions[i[0] as usize];
                            let v1 = positions[i[1] as usize];
                            let v2 = positions[i[2] as usize];
                            let n = triangle_normal(v0, v1, v2);
                            vertices.extend_from_slice(&[
                                Vertex {
                                    position: v0,
                                    normal: n,
                                    color: [1.0, 0.0, 0.0],
                                },
                                Vertex {
                                    position: v1,
                                    normal: n,
                                    color: [1.0, 0.0, 0.0],
                                },
                                Vertex {
                                    position: v2,
                                    normal: n,
                                    color: [1.0, 0.0, 0.0],
                                },
                            ]);
                        }
                    }
                }
            }
            named_mesh(
                node_name.to_string(),
                Mesh {
                    vertices,
                    just_loaded: true,
                },
            );
        }
    }
    Ok(())
}

fn load_buffers(gltf: &gltf::Gltf) -> Result<Vec<Vec<u8>>, MeshLoadError> {
    const OCTET_STREAM_URI: &str = "data:application/octet-stream;base64,";
    let mut buffer_data = Vec::new();
    for buffer in gltf.buffers() {
        match buffer.source() {
            gltf::buffer::Source::Uri(uri) => {
                if uri.starts_with(OCTET_STREAM_URI) {
                    buffer_data.push(base64::decode(&uri[OCTET_STREAM_URI.len()..])?);
                } else {
                    return Err(MeshLoadError::UnsupportedBufferFormat);
                }
            }
            gltf::buffer::Source::Bin => {
                if let Some(blob) = gltf.blob.as_deref() {
                    buffer_data.push(blob.into());
                } else {
                    return Err(MeshLoadError::MissingBlob);
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
        load_gltf(std::fs::read("res/gltf/test.gltf").unwrap().as_slice(), |mesh, name| {});
    }
}
