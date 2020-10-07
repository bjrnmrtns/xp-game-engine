use crate::{Collision, Response, Sphere, Triangle};
use nalgebra_glm::{dot, Vec3};
use xp_math::get_roots;

// plane constant is a point on the plane
fn signed_distance(p: &Vec3, plane_constant: f32, n: &Vec3) -> f32 {
    dot(&n, &p) + plane_constant
}

fn min(vals: &[f32]) -> Option<f32> {
    if vals.is_empty() {
        return None;
    }
    let mut min = vals[0];
    for val in vals {
        if *val < min {
            min = *val;
        }
    }
    Some(min)
}

fn detect_triangle_collision(
    sphere: &Sphere,
    triangle: &Triangle,
    movement: &Vec3,
    signed_distance: f32,
    plane_normal_dot_movement: f32,
) -> Option<f32> {
    let t0 = (1.0 - signed_distance) / plane_normal_dot_movement;
    let t1 = (-1.0 - signed_distance) / plane_normal_dot_movement;
    let (t0, t1) = if t0 < t1 { (t0, t1) } else { (t1, t0) };
    if t0 > 1.0 || t1 < 0.0 {
        return None;
    }
    let t0 = if t0 > 0.0 { t0 } else { 0.0 };
    let p = sphere.c + movement * t0;
    if triangle.point_in_triangle(&p) {
        return Some(t0);
    }
    None
}

fn detect_vertex_collision(
    sphere: &Sphere,
    v0: &Vec3,
    movement: &Vec3,
    movement_squared_length: f32,
) -> Vec<f32> {
    let a = movement_squared_length;
    let b = 2.0 * dot(&movement, &(sphere.c - v0));
    let c = nalgebra_glm::length(&(v0 - sphere.c));
    let c = c * c - 1.0;
    let mut ts = Vec::new();
    if let Some(ts_v) = get_roots(a, b, c) {
        ts.extend_from_slice(&[ts_v.0, ts_v.1])
    }
    ts
}

fn detect_vertices_collision(
    sphere: &Sphere,
    triangle: &Triangle,
    movement: &Vec3,
    movement_squared_length: f32,
) -> Vec<f32> {
    let mut ts = Vec::new();
    ts.extend_from_slice(
        detect_vertex_collision(&sphere, &triangle.v0, &movement, movement_squared_length)
            .as_slice(),
    );
    ts.extend_from_slice(
        detect_vertex_collision(&sphere, &triangle.v1, &movement, movement_squared_length)
            .as_slice(),
    );
    ts.extend_from_slice(
        detect_vertex_collision(&sphere, &triangle.v2, &movement, movement_squared_length)
            .as_slice(),
    );
    ts
}

fn detect_edge_collision(
    sphere: &Sphere,
    v0: &Vec3,
    v1: &Vec3,
    movement: &Vec3,
    movement_squared_length: f32,
) -> Vec<f32> {
    let edge = v1 - v0;
    let base_to_vertex = v0 - sphere.c;
    let base_to_vertex_length = nalgebra_glm::length(&base_to_vertex);
    let edge_length = nalgebra_glm::length(&edge);
    let edge_length_squared = edge_length * edge_length;
    let edge_dot_movement = dot(&edge, &movement);
    let edge_dot_base_to_vertex = dot(&edge, &base_to_vertex);

    let a = edge_length_squared * -movement_squared_length + edge_dot_movement * edge_dot_movement;
    let b = edge_length_squared * (2.0 * dot(&movement, &base_to_vertex))
        - 2.0 * edge_dot_movement * edge_dot_base_to_vertex;
    let c = edge_length_squared * (1.0 - base_to_vertex_length * base_to_vertex_length)
        + edge_dot_base_to_vertex * edge_dot_base_to_vertex;
    let mut ts = Vec::new();
    if let Some((r0, r1)) = get_roots(a, b, c) {
        let t = min(&[r0, r1]).unwrap();
        let f = (edge_dot_movement * t - edge_dot_base_to_vertex) / edge_length_squared;
        if f >= 0.0 && f <= 1.0 {
            ts.push(t);
        }
    }
    ts
}

fn detect_edges_collision(
    sphere: &Sphere,
    triangle: &Triangle,
    movement: &Vec3,
    movement_squared_length: f32,
) -> Vec<f32> {
    let mut ts = Vec::new();
    ts.extend_from_slice(
        detect_edge_collision(
            &sphere,
            &triangle.v0,
            &triangle.v1,
            &movement,
            movement_squared_length,
        )
        .as_slice(),
    );
    ts.extend_from_slice(
        detect_edge_collision(
            &sphere,
            &triangle.v1,
            &triangle.v2,
            &movement,
            movement_squared_length,
        )
        .as_slice(),
    );
    ts.extend_from_slice(
        detect_edge_collision(
            &sphere,
            &triangle.v2,
            &triangle.v0,
            &movement,
            movement_squared_length,
        )
        .as_slice(),
    );
    ts
}

// Sphere/Triangle collision detection Kasper Fauerby (2003) (Swept Sphere Volume Continuous Collision Detection)
pub fn sphere_triangle_detect_collision(
    response: &Response,
    triangle: &Triangle,
) -> Option<Collision> {
    let sphere = &response.sphere;
    let movement = &response.movement;
    assert_eq!(sphere.r, 1.0);
    let normal = triangle.normal().normalize();
    let normalized_movement = movement.normalize();

    // if triangle does not face towards movement, we ignore it and return
    if dot(&normal, &normalized_movement) > 0.0 {
        return None;
    }

    let plane_normal_dot_movement = dot(&normal, &movement);
    let sd = signed_distance(&sphere.c, triangle.plane_constant(), &normal);

    // if the movement is parallel to the plane and the distance is bigger than the sphere radius
    // we cannot collide and return
    if plane_normal_dot_movement == 0.0 && sd.abs() >= 1.0 {
        return None;
    }

    let movement_length = nalgebra_glm::length(&movement);
    let movement_squared_length = movement_length * movement_length;

    // if the movment is not parallel to the plane and is more that sphere radius away, we check
    // if we get a plane collision and later a inside triangle collision
    if plane_normal_dot_movement != 0.0 && sd.abs() >= 1.0 {
        if let Some(t) =
            detect_triangle_collision(&sphere, &triangle, &movement, sd, plane_normal_dot_movement)
        {
            return Some(Collision {
                time_to: t,
                distance_to: movement_length * t,
                position: sphere.c + movement * t,
            });
        }
    }
    let mut ts = detect_vertices_collision(&sphere, &triangle, &movement, movement_squared_length);
    ts.extend_from_slice(
        detect_edges_collision(&sphere, &triangle, &movement, movement_squared_length).as_slice(),
    );
    if let Some(t) = min(ts.as_slice()) {
        if t > 0.0 {
            return Some(Collision {
                time_to: t,
                distance_to: movement_length * t,
                position: sphere.c + movement * t,
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::collision_detect::sphere_triangle_detect_collision;
    use crate::{Response, Sphere, Triangle};
    use nalgebra_glm::vec3;

    #[test]
    fn test_detect_where_collision_inside_triangle() {
        let triangle = Triangle::new(
            vec3(-2.0, 2.0, -2.0),
            vec3(-2.0, 2.0, 2.0),
            vec3(2.0, 2.0, 0.0),
        );
        let sphere = Sphere::new(vec3(0.0, 4.0, 0.0), 1.0);
        let movement = vec3(0.0, -2.0, 0.0);
        let c = sphere_triangle_detect_collision(&Response { sphere, movement }, &triangle);
        assert_eq!(c.unwrap().time_to, 0.5);
    }

    #[test]
    fn test_detect_where_collision_against_vertex() {
        let triangle = Triangle::new(
            vec3(0.0, 0.0, 0.0),
            vec3(-2.0, -1.0, 0.0),
            vec3(2.0, -1.0, 0.0),
        );
        // vertex will be hit at 0.0, 0.0, 0.0
        let sphere = Sphere::new(vec3(0.0, 4.0, 0.0), 1.0);
        let movement = vec3(0.0, -8.0, 0.0);
        let c = sphere_triangle_detect_collision(&Response { sphere, movement }, &triangle);
        assert_eq!(c.unwrap().time_to, 0.375);
    }

    #[test]
    fn test_detect_where_collision_against_edge() {
        let triangle = Triangle::new(
            vec3(0.0, -2.0, 0.0),
            vec3(-2.0, -1.0, 0.0),
            vec3(2.0, -1.0, 0.0),
        );
        // vertex will be hit at 0.0, 0.0, 0.0
        let sphere = Sphere::new(vec3(0.0, 4.0, 0.0), 1.0);
        let movement = vec3(0.0, -8.0, 0.0);
        let c = sphere_triangle_detect_collision(&Response { sphere, movement }, &triangle);
        assert_eq!(c.unwrap().time_to, 0.5);
    }
}
