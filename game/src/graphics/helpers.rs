use crate::graphics::default::Vertex;
use crate::graphics::Mesh;
use nalgebra_glm::{cross, make_vec3, triangle_normal, Vec3};
use std::collections::HashSet;
use std::convert::TryInto;

pub fn ensure_unique_provoking_vertices(
    vertices: &[[f32; 3]],
    indices: &[u32],
) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut new_vertices = vertices.to_vec();
    let mut new_indices = indices.to_vec();
    let mut provs_used: HashSet<u32> = HashSet::new();
    for face in indices.chunks(3).enumerate() {
        // first vertex of face is a provoking vertex
        if provs_used.contains(&face.1[0]) {
            new_vertices.push(vertices[face.1[0] as usize].clone());
            new_indices[&face.0 * 3] = new_vertices.len() as u32 - 1;
        } else {
            provs_used.insert(face.1[0]);
        }
    }
    (new_vertices, new_indices)
}

pub fn make_mesh_from_flat_obj(
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

pub fn enhance_provoking_vertices(vertices: &[[f32; 3]], indices: &[u32]) -> Vec<Vertex> {
    let mut mesh_vertices: Vec<Vertex> = vertices
        .iter()
        .map(|v| Vertex {
            position: *v,
            normal: [0.0, 1.0, 0.0],
            color: [1.0, 0.0, 0.0],
        })
        .collect();
    for face in indices.chunks(3) {
        let edge_0: Vec3 =
            make_vec3(&vertices[face[2] as usize]) - make_vec3(&vertices[face[0] as usize]);
        let edge_1: Vec3 =
            make_vec3(&vertices[face[1] as usize]) - make_vec3(&vertices[face[0] as usize]);
        let n: Vec3 = cross(&edge_1, &edge_0).normalize();
        mesh_vertices[face[0] as usize].normal = n.as_slice().try_into().unwrap();
    }
    mesh_vertices
}

pub fn enhance_provoking_vertices2(mut mesh: Mesh<Vertex>) -> Mesh<Vertex> {
    for face in mesh.indices.chunks(3) {
        let edge_0: Vec3 = make_vec3(&mesh.vertices[face[2] as usize].position)
            - make_vec3(&mesh.vertices[face[0] as usize].position);
        let edge_1: Vec3 = make_vec3(&mesh.vertices[face[1] as usize].position)
            - make_vec3(&mesh.vertices[face[0] as usize].position);
        let n: Vec3 = cross(&edge_1, &edge_0).normalize();
        mesh.vertices[face[0] as usize].normal = n.as_slice().try_into().unwrap();
    }
    mesh
}
