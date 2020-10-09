use crate::graphics;
use crate::graphics::default::Vertex;
use crate::graphics::Mesh;
use genmesh::{MapToVertices, Triangulate, Vertices};
use nalgebra_glm::{make_vec3, triangle_normal, Vec3};
use std::collections::HashSet;
use std::convert::TryInto;
use xp_physics::{Sphere, Triangle};
use xp_ui::{Widget, UI};

fn make_mesh_from_flat_obj(
    vertices_flat: &[f32],
    indices: &[u32],
    in_color: &[f32; 3],
) -> Mesh<Vertex> {
    let mut vertices: Vec<Vertex> = vertices_flat
        .chunks(3)
        .map(|v| Vertex {
            position: [v[0], v[1], v[2]],
            normal: [0.0, 0.0, 0.0],
            color: *in_color,
        })
        .collect();
    let mut new_indices: Vec<u32> = Vec::new();
    let mut used_as_provoking: HashSet<u32> = HashSet::new();
    for face in indices.chunks(3) {
        if used_as_provoking.contains(&face[0]) {
            vertices.push(vertices[face[0] as usize].clone());
            new_indices.extend([vertices.len() as u32 - 1, face[1], face[2]].to_vec());
        } else {
            used_as_provoking.insert(face[0]);
            new_indices.extend(face);
        }
    }
    for face in new_indices.chunks(3) {
        let n = create_normal([
            vertices[face[0] as usize].position,
            vertices[face[1] as usize].position,
            vertices[face[2] as usize].position,
        ]);
        vertices[face[0] as usize].normal = n;
    }
    let mesh = Mesh {
        vertices,
        indices: new_indices,
    };
    mesh
}

fn create_normal(in_positions: [[f32; 3]; 3]) -> [f32; 3] {
    triangle_normal(
        &make_vec3(&in_positions[0]),
        &make_vec3(&in_positions[1]),
        &make_vec3(&in_positions[2]),
    )
    .as_slice()
    .try_into()
    .unwrap()
}

pub fn create_collision_triangle_and_sphere(
    triangle: Triangle,
    sphere: Sphere,
    sphere_movement: Vec3,
    render_sphere_at_t: &[f32],
) -> (Vec<graphics::debug::Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    vertices.push(graphics::debug::Vertex {
        position: triangle.v0.into(),
    });
    vertices.push(graphics::debug::Vertex {
        position: triangle.v1.into(),
    });
    vertices.push(graphics::debug::Vertex {
        position: triangle.v2.into(),
    });
    assert_eq!(sphere.r, 1.0);
    for t in render_sphere_at_t {
        vertices.extend(
            genmesh::generators::SphereUv::new(10, 10)
                .vertex(|v| {
                    let pos = [
                        sphere.c[0] + v.pos.x + sphere_movement[0] * t,
                        sphere.c[1] + v.pos.y + sphere_movement[1] * t,
                        sphere.c[2] + v.pos.z + sphere_movement[2] * t,
                    ];
                    graphics::debug::Vertex { position: pos }
                })
                .triangulate()
                .vertices(),
        );
    }
    let mut indices = vec![0; vertices.len() * 2];
    let mut iindex = 0;
    for vindex in (0..vertices.len()).step_by(3) {
        indices[iindex] = vindex as u32;
        indices[iindex + 1] = vindex as u32 + 1;
        indices[iindex + 2] = vindex as u32 + 1;
        indices[iindex + 3] = vindex as u32 + 2;
        indices[iindex + 4] = vindex as u32 + 2;
        indices[iindex + 5] = vindex as u32;
        iindex += 3;
    }
    (vertices, indices)
}

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
        let model_mesh = make_mesh_from_flat_obj(
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
