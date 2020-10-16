use crate::error::MeshError;
use crate::Triangle;
use nalgebra_glm::{vec3, Vec3};
use tobj::{Material, Model};

pub struct Obj {
    models: Vec<Model>,
    materials: Vec<Material>,
    m_i: usize,
    f_i: usize,
}

impl Obj {
    pub fn load(file_name: &str) -> Result<Self, MeshError> {
        let (models, materials) = tobj::load_obj(file_name, true)?;
        Ok(Self {
            models,
            materials,
            m_i: 0,
            f_i: 0,
        })
    }
}

impl Iterator for Obj {
    type Item = Triangle<Vec3>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.m_i == self.models.len() {
                break None;
            }
            if self.f_i >= self.models[self.m_i].mesh.indices.len() {
                self.f_i = 0;
                self.m_i += 1;
                continue;
            }
            let diffuse_color: Option<Vec3> =
                if let Some(material_id) = self.models[self.m_i].mesh.material_id {
                    Some(vec3(
                        self.materials[material_id].diffuse[0],
                        self.materials[material_id].diffuse[1],
                        self.materials[material_id].diffuse[2],
                    ))
                } else {
                    None
                };

            let vi0 = (self.models[self.m_i].mesh.indices[self.f_i] * 3) as usize;
            let vi1 = (self.models[self.m_i].mesh.indices[self.f_i + 1] * 3) as usize;
            let vi2 = (self.models[self.m_i].mesh.indices[self.f_i + 2] * 3) as usize;
            self.f_i += 3;
            break Some(Self::Item {
                positions: [
                    vec3(
                        self.models[self.m_i].mesh.positions[vi0],
                        self.models[self.m_i].mesh.positions[vi0 + 1],
                        self.models[self.m_i].mesh.positions[vi0 + 2],
                    ),
                    vec3(
                        self.models[self.m_i].mesh.positions[vi1],
                        self.models[self.m_i].mesh.positions[vi1 + 1],
                        self.models[self.m_i].mesh.positions[vi1 + 2],
                    ),
                    vec3(
                        self.models[self.m_i].mesh.positions[vi2],
                        self.models[self.m_i].mesh.positions[vi2 + 1],
                        self.models[self.m_i].mesh.positions[vi2 + 2],
                    ),
                ],
                diffuse_color: diffuse_color,
            });
        }
    }
}
