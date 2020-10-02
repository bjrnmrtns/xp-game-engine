use nalgebra_glm::{dot, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

// point in triangle is only valid when point p is already on plane of triangle,
pub fn point_in_triangle(p0: &Vec3, p1: &Vec3, p2: &Vec3, p: &Vec3) -> bool {
    // point in triangle the barycentric technique
    let v0 = p2 - p0;
    let v1 = p1 - p0;
    let v2 = p - p0;
    let dot00 = dot(&v0, &v0);
    let dot01 = dot(&v0, &v1);
    let dot02 = dot(&v0, &v2);
    let dot11 = dot(&v1, &v1);
    let dot12 = dot(&v1, &v2);

    // compute barycentric coordinates
    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    // not sure if we want to exclude u + v == 1.0
    // currentl we make the triangle end edges inclusive (also part of triangle)
    // if u + v == 1.0 is excluded so are the end edges
    // as 0.0 is included it seems ok to include 1.0 as well
    return u >= 0.0 && v >= 0.0 && u + v <= 1.0;
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
    pub fn point_in_triangle(&self, p: &Vec3) -> bool {
        point_in_triangle(&self.v0, &self.v1, &self.v2, &p)
    }
}

#[cfg(test)]
mod tests {
    use crate::Triangle;
    use nalgebra_glm::vec3;

    #[test]
    fn point_in_triangle_test() {
        let t = Triangle::new(
            vec3(-1.0, 0.0, 0.0),
            vec3(0.0, 1.0, 1.0),
            vec3(1.0, 0.0, 0.0),
        );
        assert!(t.point_in_triangle(&vec3(0.0, 0.5, 0.5)));
        assert!(t.point_in_triangle(&vec3(0.0, 0.6, 0.6)));
        assert!(t.point_in_triangle(&vec3(0.0, 0.99, 0.99)));
        assert!(!t.point_in_triangle(&vec3(0.0, 1.001, 1.001)));

        assert!(t.point_in_triangle(&vec3(0.0, 0.0, 0.0)));
        // edge case where u + v == 1.0
        assert!(t.point_in_triangle(&vec3(0.0, 1.0, 1.0)));
    }
}
