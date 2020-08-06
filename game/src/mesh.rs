use crate::{terrain, graphics};
use nalgebra_glm::{triangle_normal, make_vec3};
use xp_ui::{Widget, UI};
use std::convert::TryInto;

pub fn create_terrain_mesh_from_tile(tile: &terrain::Tile) -> graphics::Mesh<graphics::default::Vertex> {
    let mut terrain = graphics::Mesh { vertices: Vec::new(), indices: Vec::new() };
    for z in 0..terrain::TILE_SIZE {
        for x in 0..terrain::TILE_SIZE {
            terrain.vertices.push(graphics::default::Vertex {
                position: tile.get_element(x, z).p.clone(),
                normal: [0.0, 1.0, 0.0],
                color: tile.color(),
            });
        }
    }
    const S: u32 = terrain::TILE_SIZE as u32;
    const S_MIN_1: u32 = S - 1;
    for x in 0..S_MIN_1 {
        for z in 0..S {
            let index = x + z * S;
            match (x, z) {
                (_, 0) => {
                    let p1 = index;
                    let p2 = index + S;
                    let p3 = index + 1;
                    terrain.indices.push(p1);
                    terrain.indices.push(p2);
                    terrain.indices.push(p3);
                    terrain.vertices[index as usize].normal = triangle_normal(&make_vec3(&terrain.vertices[p1 as usize].position), &make_vec3(&terrain.vertices[p2 as usize].position), &make_vec3(&terrain.vertices[p3 as usize].position)).as_slice().try_into().unwrap();
                },
                (_, S_MIN_1) => {
                    let p1 = index;
                    let p2 = index + 1;
                    let p3 = index - S + 1;
                    terrain.indices.push(p1);
                    terrain.indices.push(p2);
                    terrain.indices.push(p3);
                    terrain.vertices[index as usize].normal = triangle_normal(&make_vec3(&terrain.vertices[p1 as usize].position), &make_vec3(&terrain.vertices[p2 as usize].position), &make_vec3(&terrain.vertices[p3 as usize].position)).as_slice().try_into().unwrap();
                },
                (_, _) => {

                    let p1 = index;
                    let p2 = index + S;
                    let p3 = index + 1;
                    terrain.indices.push(p1);
                    terrain.indices.push(p2);
                    terrain.indices.push(p3);
                    terrain.vertices[index as usize].normal = triangle_normal(&make_vec3(&terrain.vertices[p1 as usize].position), &make_vec3(&terrain.vertices[p2 as usize].position), &make_vec3(&terrain.vertices[p3 as usize].position)).as_slice().try_into().unwrap();

                    // duplicate this vertex, as provoking vertices cannot be re-used
                    terrain.vertices.push(terrain.vertices[index as usize]);
                    let p4 = terrain.vertices.len() as u32 - 1;
                    let p5 = index + 1;
                    let p6 = index - S + 1;
                    terrain.indices.push(p4);
                    terrain.indices.push(p5);
                    terrain.indices.push(p6);
                    terrain.vertices[p4 as usize].normal = triangle_normal(&make_vec3(&terrain.vertices[p4 as usize].position), &make_vec3(&terrain.vertices[p5 as usize].position), &make_vec3(&terrain.vertices[p6 as usize].position)).as_slice().try_into().unwrap();
                }
            }
        }
    }
    terrain
}

pub fn create_mesh_from(obj_file_name: &str) -> graphics::Mesh<graphics::default::Vertex> {
    let (models, materials) = tobj::load_obj(obj_file_name, true).expect(format!("Could not read obj file: {}", obj_file_name).as_str());
    let mut mesh = graphics::Mesh { vertices: Vec::new(), indices: Vec::new() };
    for model in models {
        let color = if let Some(material_id) = model.mesh.material_id {
            materials[material_id].diffuse
        } else {
            [0.8, 0.0, 0.8]
        };
        let model_mesh = graphics::helpers::make_mesh_from_flat_obj(model.mesh.positions.as_ref(), model.mesh.indices.as_ref(), &color);
        let index_offset = mesh.vertices.len() as u32;
        mesh.vertices.extend(model_mesh.vertices);
        mesh.indices.extend(model_mesh.indices.iter().map(|i| i + index_offset ));
    }
    mesh
}

pub fn create_mesh<T>(ui: &UI<T, u32>) -> (graphics::Mesh::<graphics::ui::Vertex>, Vec<graphics::ui::Text>) {
    let mut mesh = graphics::Mesh::<graphics::ui::Vertex> { vertices: Vec::new(), indices: Vec::new() };
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
                    position: [layout.position.x + layout.size.width, layout.position.y - layout.size.height],
                    uv: [0.0, 0.0],
                    color: label.color,
                };
                text.push(graphics::ui::Text{
                    pos: (layout.position.x, layout.position.y - ui.window_size.1),
                    text: label.text.text.clone(),
                    font_size: label.text.font_size,
                    color: label.text.color,
                });
                let offset = mesh.vertices.len() as u32;
                mesh.indices.extend_from_slice(&[offset + 0, offset + 1, offset + 2, offset + 2, offset + 1, offset + 3]);
                mesh.vertices.extend_from_slice(&[top_left, bottom_left, top_right, bottom_right]);
            },
        }
    }
    (mesh, text)
}
