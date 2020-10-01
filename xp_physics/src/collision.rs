use crate::{Sphere, Triangle};
use nalgebra_glm::{cross, dot, normalize, vec3, Vec3};

pub struct Collision {
    pub t0: f32,
    pub t1: f32,
}

// plane constant is a point on the plane
fn signed_distance(p: &Vec3, plane_constant: f32, n: &Vec3) -> f32 {
    dot(&n, &p) + plane_constant
}

pub fn detect(sphere: &Sphere, triangle: &Triangle, movement: &Vec3) -> Collision {
    let t_normal_normalized = triangle.normal().normalize();
    let sd = signed_distance(&sphere.c, triangle.plane_constant(), &t_normal_normalized);
    let t0 = (1.0 - sd) / dot(&t_normal_normalized, &movement);
    let t1 = (-1.0 - sd) / dot(&t_normal_normalized, &movement);
    Collision { t0, t1 }
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
        let movement = vec3(0.0, -2.0, 0.0);
        let c = detect(&sphere, &triangle, &movement);
    }
}
