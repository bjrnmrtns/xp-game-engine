pub const MAX_NR_OF_DIRECTIONAL_LIGHTS: usize = 1;
pub const MAX_NR_OF_SPOT_LIGHTS: usize = 10;
pub const MAX_NR_OF_POINT_LIGHTS: usize = 10;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DirectionalProperties {
    pub direction: [f32; 4],
    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
}

impl DirectionalProperties {
    pub fn new(direction: [f32; 4]) -> Self {
        Self {
            direction,
            ambient: [0.05, 0.05, 0.05, 1.0],
            diffuse: [0.4, 0.4, 0.4, 1.0],
            specular: [0.1, 0.1, 0.1, 1.0],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SpotProperties {
    pub position: [f32; 4],
    pub direction: [f32; 4],
    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    pub cut_off_inner: f32,
    pub cut_off_outer: f32,
    pub p0: f32,
    pub p1: f32,
    pub p2: f32,
}

impl SpotProperties {
    pub fn new(position: [f32; 4], direction: [f32; 4]) -> Self {
        Self {
            position,
            direction,
            ambient: [0.0, 0.0, 0.0, 1.0],
            diffuse: [1.0, 1.0, 1.0, 1.0],
            specular: [1.0, 1.0, 1.0, 1.0],
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
            cut_off_inner: (12.5 * (std::f32::consts::PI / 180.0)).cos(),
            cut_off_outer: (15.0 * (std::f32::consts::PI / 180.0)).cos(),
            p0: 0.0,
            p1: 0.0,
            p2: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PointProperties {
    pub position: [f32; 4],
    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    pub p0: f32,
}

impl PointProperties {
    pub fn new(position: [f32; 4]) -> Self {
        Self {
            position,
            ambient: [0.05, 0.05, 0.05, 1.0],
            diffuse: [0.8, 0.8, 0.8, 1.0],
            specular: [1.0, 1.0, 1.0, 1.0],
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
            p0: 0.0,
        }
    }
}

pub enum Light {
    Directional(DirectionalProperties),
    Spot(SpotProperties),
    Point(PointProperties),
}

unsafe impl bytemuck::Pod for DirectionalProperties {}
unsafe impl bytemuck::Zeroable for DirectionalProperties {}
unsafe impl bytemuck::Pod for SpotProperties {}
unsafe impl bytemuck::Zeroable for SpotProperties {}
unsafe impl bytemuck::Pod for PointProperties {}
unsafe impl bytemuck::Zeroable for PointProperties {}
