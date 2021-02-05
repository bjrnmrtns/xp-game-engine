pub struct DirectionalProperties {
    pub direction: [f32; 3],
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
}

impl Default for DirectionalProperties {
    fn default() -> Self {
        Self {
            direction: [-0.2, -1.0, -0.3],
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

impl Default for SpotProperties {
    fn default() -> Self {
        Self {
            position: [0.0, 5.0, 0.0],
            direction: [0.0, -1.0, 0.0],
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

impl Default for PointProperties {
    fn default() -> Self {
        Self {
            position: [10.0, 5.0, 10.0],
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
