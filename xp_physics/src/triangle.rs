use nalgebra_glm::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Self { v0, v1, v2 }
    }
    pub fn normal(&self) -> Vec3 {
        nalgebra_glm::triangle_normal(&self.v0, &self.v1, &self.v2)
    }
    pub fn plane_constant(&self) -> f32 {
        let normal = self.normal().normalize();
        -(normal.x * self.v0.x + normal.y * self.v0.y + normal.z * self.v0.z)
    }
}
