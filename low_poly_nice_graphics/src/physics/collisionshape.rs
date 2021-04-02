#[derive(Clone)]
pub enum BodyStatus {
    Static,
    Dynamic,
}

#[derive(Clone)]
pub struct Cuboid {
    pub half_extent_x: f32,
    pub half_extent_y: f32,
    pub half_extent_z: f32,
}

#[derive(Clone)]
pub struct Sphere {
    pub radius: f32,
}

#[derive(Clone)]
pub enum Body {
    Cuboid(Cuboid),
    Sphere(Sphere),
}

#[derive(Clone)]
pub struct CollisionShape {
    pub body_status: BodyStatus,
    pub body: Body,
}
