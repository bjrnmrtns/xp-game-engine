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
            ambient: [0.2, 0.2, 0.2],
            diffuse: [0.5, 0.5, 0.5],
            specular: [1.0, 1.0, 1.0],
        }
    }
}

pub struct SpotProperties {
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub cut_off_inner: f32,
    pub cut_off_outer: f32,
}

impl SpotProperties {
    pub fn new(position: [f32; 3], direction: [f32; 3]) -> Self {
        Self {
            position,
            direction,
            cut_off_inner: (12.5 * (std::f32::consts::FRAC_PI_2 / 360.0)).cos(),
            cut_off_outer: (17.5 * (std::f32::consts::FRAC_PI_2 / 360.0)).cos(),
        }
    }
}

pub struct PointProperties {
    pub position: [f32; 3],
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl PointProperties {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            position,
            ambient: [0.2, 0.2, 0.2],
            diffuse: [0.5, 0.5, 0.5],
            specular: [1.0, 1.0, 1.0],
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
