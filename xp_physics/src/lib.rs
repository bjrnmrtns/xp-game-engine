mod collision;
mod collision_detect;
mod collision_response;
mod response;
mod sphere;
mod triangle;

use crate::collision::DISTANCE_EPSILON;
pub use crate::response::Response;
pub use collision::Collision;
pub use collision_detect::sphere_triangle_detect_collision;
pub use collision_response::sphere_triangle_calculate_response;
pub use sphere::Sphere;
pub use triangle::Triangle;

pub fn collision_response(response: Response, triangles: &[Triangle]) -> Response {
    let mut response = response;
    loop {
        let closest_collision = triangles
            .iter()
            .filter_map(|t| sphere_triangle_detect_collision(&response, t))
            .min_by(|c0, c1| {
                if c0.time_to < c1.time_to {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
        match closest_collision {
            Some(c) => response = sphere_triangle_calculate_response(&response, &c),
            None => {
                break response;
            }
        }
        if nalgebra_glm::length(&response.movement) < DISTANCE_EPSILON {
            break response;
        }
    }
}
