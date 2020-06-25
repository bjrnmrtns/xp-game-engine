use nalgebra_glm::{Vec3, Quat, vec4_to_vec3, quat_to_mat4, vec4, quat_angle_axis, vec3};

const GLOBAL_DIR_X: f32 = 1.0;
const GLOBAL_DIR_Y: f32 = 1.0;
const GLOBAL_DIR_Z: f32 = -1.0;

pub fn move_along_local_axis(position: &Vec3, orientation: &Quat, forward: f32, right: f32, up: f32) -> Vec3 {
    // point small vector in same direction as player and add it to player position
    let movement = vec4_to_vec3(&(quat_to_mat4(&orientation) * vec4(GLOBAL_DIR_X * right, GLOBAL_DIR_Y * up, GLOBAL_DIR_Z * forward, 1.0)));
    position + movement
}

pub fn rotate_around_local_axis(orientation: &Quat, around_x: f32, around_y: f32, around_z: f32) -> Quat {
    // most important rule about quaternions is that all rotations are around local quaternion axis, say you rotate around x -> store in quatX, rotate around y -> store in quatY, multiply quatX
    // with quatY -> quatX * quatY, means that the rotation quatY is around local Y of quatX.
    orientation * quat_angle_axis(around_x, &vec3(1.0, 0.0, 0.0)) * quat_angle_axis(around_y, &vec3(0.0, 1.0, 0.0)) * quat_angle_axis(around_z, &vec3(0.0, 0.0, 1.0))
}
