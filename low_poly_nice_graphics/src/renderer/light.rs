pub const MAX_NR_OF_DIRECTIONAL_LIGHTS: usize = 1;
pub const MAX_NR_OF_SPOT_LIGHTS: usize = 10;
pub const MAX_NR_OF_POINT_LIGHTS: usize = 10;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DirectionalProperties {
    pub direction: [f32; 3],
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
}

impl DirectionalProperties {
    pub fn new(direction: [f32; 3]) -> Self {
        Self {
            direction,
            ambient: [0.05, 0.05, 0.05],
            diffuse: [0.4, 0.4, 0.4],
            specular: [0.5, 0.5, 0.5],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SpotProperties {
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub padding0: f32, //padding added because glsl alignment after vec3 is wrongly padded
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    pub cut_off_inner: f32,
    pub cut_off_outer: f32,
}

impl SpotProperties {
    pub fn new(position: [f32; 3], direction: [f32; 3]) -> Self {
        Self {
            position,
            direction,
            ambient: [0.0, 0.0, 0.0],
            diffuse: [1.0, 1.0, 1.0],
            specular: [1.0, 1.0, 1.0],
            padding0: 0.0,
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
            cut_off_inner: (12.5 * (std::f32::consts::FRAC_PI_2 / 360.0)).cos(),
            cut_off_outer: (15.0 * (std::f32::consts::FRAC_PI_2 / 360.0)).cos(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PointProperties {
    pub position: [f32; 3],
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub padding0: f32, //padding added because glsl alignment after vec3 is wrongly padded
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl PointProperties {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            position,
            ambient: [0.05, 0.05, 0.05],
            diffuse: [0.8, 0.8, 0.8],
            specular: [1.0, 1.0, 1.0],
            padding0: 0.0,
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
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
