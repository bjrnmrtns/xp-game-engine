use crate::graphics;
use genmesh::{MapToVertices, Triangulate, Vertices};
use xp_ui::{Widget, UI};

pub fn create_player_sphere() -> graphics::Mesh<graphics::default::Vertex> {
    let mut mesh = graphics::Mesh {
        vertices: Vec::new(),
        indices: Vec::new(),
    };
    mesh.vertices = genmesh::generators::SphereUv::new(10, 10)
        .vertex(|v| graphics::default::Vertex {
            position: v.pos.into(),
            normal: v.normal.into(),
            color: [0.0, 1.0, 0.0],
        })
        .triangulate()
        .vertices()
        .collect();
    mesh.indices = vec![0; mesh.vertices.len()];
    for index in (0..mesh.indices.len()).step_by(3) {
        mesh.indices[index] = index as u32;
        mesh.indices[index + 1] = index as u32 + 1;
        mesh.indices[index + 2] = index as u32 + 2;
    }
    mesh
}

pub fn create_mesh_from(obj_file_name: &str) -> graphics::Mesh<graphics::default::Vertex> {
    let (models, materials) = tobj::load_obj(obj_file_name, true)
        .expect(format!("Could not read obj file: {}", obj_file_name).as_str());
    let mut mesh = graphics::Mesh {
        vertices: Vec::new(),
        indices: Vec::new(),
    };
    for model in models {
        let color = if let Some(material_id) = model.mesh.material_id {
            materials[material_id].diffuse
        } else {
            [0.8, 0.0, 0.8]
        };
        let model_mesh = graphics::helpers::make_mesh_from_flat_obj(
            model.mesh.positions.as_ref(),
            model.mesh.indices.as_ref(),
            &color,
        );
        let index_offset = mesh.vertices.len() as u32;
        mesh.vertices.extend(model_mesh.vertices);
        mesh.indices
            .extend(model_mesh.indices.iter().map(|i| i + index_offset));
    }
    mesh
}

pub fn create_mesh<T>(
    ui: &UI<T, u32>,
) -> (
    graphics::Mesh<graphics::ui::Vertex>,
    Vec<graphics::ui::Text>,
) {
    let mut mesh = graphics::Mesh::<graphics::ui::Vertex> {
        vertices: Vec::new(),
        indices: Vec::new(),
    };
    let mut text = Vec::new();
    for (_, widget) in ui.widgets() {
        match widget {
            Widget::LabelW(layout, label) => {
                let top_left = graphics::ui::Vertex {
                    position: [layout.position.x, layout.position.y],
                    uv: [0.0, 0.0],
                    color: label.color,
                };
                let bottom_left = graphics::ui::Vertex {
                    position: [layout.position.x, layout.position.y - layout.size.height],
                    uv: [0.0, 0.0],
                    color: label.color,
                };
                let top_right = graphics::ui::Vertex {
                    position: [layout.position.x + layout.size.width, layout.position.y],
                    uv: [0.0, 0.0],
                    color: label.color,
                };
                let bottom_right = graphics::ui::Vertex {
                    position: [
                        layout.position.x + layout.size.width,
                        layout.position.y - layout.size.height,
                    ],
                    uv: [0.0, 0.0],
                    color: label.color,
                };
                text.push(graphics::ui::Text {
                    pos: (layout.position.x, layout.position.y - ui.window_size.1),
                    text: label.text.text.clone(),
                    font_size: label.text.font_size,
                    color: label.text.color,
                });
                let offset = mesh.vertices.len() as u32;
                mesh.indices.extend_from_slice(&[
                    offset + 0,
                    offset + 1,
                    offset + 2,
                    offset + 2,
                    offset + 1,
                    offset + 3,
                ]);
                mesh.vertices
                    .extend_from_slice(&[top_left, bottom_left, top_right, bottom_right]);
            }
        }
    }
    (mesh, text)
}
