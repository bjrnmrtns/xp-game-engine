use crate::{registry::Handle, renderer::VertexBuffer, transform::Transform};

pub struct Entity {
    pub vb_handle: Handle<VertexBuffer>,
    pub transform: Transform,
}
