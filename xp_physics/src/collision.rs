use crate::{Sphere, Triangle};
use nalgebra_glm::{cross, vec3, Vec3};

fn triangle_normal(t: Triangle) -> Vec3 {
    let edge_0: Vec3 = t.v1.clone() - t.v0;
    let edge_1: Vec3 = t.v2 - t.v1;
    cross(&edge_0, &edge_1)
}

fn signed_distance(p: [f32; 3]) -> f32 {
    3.0 //    N * p + Cp;
}

pub fn detect(sphere: Sphere, triangle: Triangle, movement: Vec3) -> f32 {
    3.0
}

#[cfg(test)]
mod tests {
    use crate::collision::detect;
    use crate::{Sphere, Triangle};
    use nalgebra_glm::vec3;

    #[test]
    fn test_detect() {
        let triangle = Triangle::new(
            vec3(-2.0, 2.0, -2.0),
            vec3(-2.0, 2.0, 2.0),
            vec3(2.0, 2.0, 0.0),
        );
        let sphere = Sphere::new(vec3(0.0, 4.0, 0.0), 1.0);
        let movement = vec3(0.0, -3.0, 0.0);
        let t = detect(sphere, triangle, movement);
    }
}
