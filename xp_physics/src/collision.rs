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

pub fn detect(sphere: &Sphere, triangle: &Triangle, movement: &Vec3) -> Option<Collision> {
    assert_eq!(sphere.r, 1.0);
    let normal = triangle.normal().normalize();
    let sd = signed_distance(&sphere.c, triangle.plane_constant(), &normal);
    let plane_normal_dot_movement = dot(&normal, &movement);
    if plane_normal_dot_movement == 0.0 && sd.abs() >= 1.0 {
        return None;
    }
    if plane_normal_dot_movement != 0.0 {
        let t0 = (1.0 - sd) / plane_normal_dot_movement;
        let t1 = (-1.0 - sd) / plane_normal_dot_movement;
        let (t0, t1) = if t0 < t1 { (t0, t1) } else { (t1, t0) };
        if t0 > 1.0 || t1 < 0.0 {
            return None;
        }
        let t0 = if t0 > 0.0 { t0 } else { 0.0 };
        let t1 = if t1 < 1.0 { t1 } else { 1.0 };
        // check for contact point inside triangle, use t0 to calculate point and check if it is inside the triangle
        return Some(Collision { t0, t1 });
    } else if plane_normal_dot_movement == 0.0 && sd.abs() < 1.0 {
        // sphere embedded in plane
        None
    } else {
        None
    }
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
        let sphere = Sphere::new(vec3(0.0, 2.0, 0.0), 1.0);
        let movement = vec3(0.0, -2.0, 0.0);
        let c = detect(&sphere, &triangle, &movement);
    }
}
