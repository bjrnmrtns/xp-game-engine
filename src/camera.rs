use nalgebra_glm::*;
/*void Camera::Move(float forward, float left) {
glm::vec3 direction = CalculateDirectionToMoveIn(around, updown);
glm::vec3 leftdirection(-direction.z, direction.y, direction.x);
position += direction * -forward;
position += leftdirection * left;
}
void Camera::MouseMove(float around_diff, float updown_diff) {
updown += updown_diff;
around += around_diff;
updown = std::min(pi_2 - 0.1f, std::max(-pi_2 + 0.1f, updown));
}
const glm::mat4 Camera::GetView() {
glm::mat4 arcBallRotation = ArcBallRotation(around, updown);
glm::vec3 directiontopos(glm::vec4(0.0f, 0.0f, -1.0f, 1.0f) * arcBallRotation);
glm::vec3 upvector(glm::vec4(0.0f, 1.0f, 0.0f, 1.0f) * arcBallRotation);
return glm::lookAt((glm::normalize(directiontopos) * glm::vec3(zoom, zoom, zoom)) + position, position, glm::normalize(upvector));
}
glm::vec3 Camera::CalculateDirectionToMoveIn(float around, float updown) {
glm::vec4 direction = glm::vec4(0.0f, 0.0f, -1.0f, 1.0f) * ArcBallRotation(around, updown);
direction.y = 0.0f; // remove y component, we are only interested in plane movement
return glm::normalize(glm::vec3(direction));
}
glm::mat4 Camera::ArcBallRotation(float around, float updown) {
return glm::rotate(glm::mat4(1.0f), updown, glm::vec3(1.0f, 0.0f, 0.0f)) *
glm::rotate(glm::mat4(1.0f), around, glm::vec3(0.0f, 1.0f, 0.0f));
}

glm::vec3 position = glm::vec3(0.0f, 5.0f, 0.0f);
float updown = 0.0f;
float around = 0.0f;
float zoom = 20.0f;
*/

pub struct Camera {
    initial_up: Vec3,
    initial_direction: Vec3,
    position: Vec3,
    orientation: Quat,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            initial_up: vec3(0.0, 1.0, 0.0),
            initial_direction: vec3(0.0, 0.0, -1.0),
            position: vec3(0.0, 0.0, 2.0),
            orientation: quat_identity(),
        }
    }

    // rotate around right vector (cross product between up and direction
    pub fn pitch(&mut self, val: f32) {
        // calculate up and direction see below
        // calculate right by cross product of up and direction
        // use quat_angle_axis
        // store new orientation by doing orientation * rot
    }

    // rotate around up vector
    pub fn yaw(&mut self, val: f32) {
        // calculate up via orientation, intitial_up
        // use quat_angle_axis()
        // store new orientation by doing orientation * rot
    }

    // rotate around direction vector
    pub fn roll(&mut self, val: f32) {
        // calculate direction via orientation, intitial_direction
        // use quat_angle_axis()
        // store new orientation by doing orientation * rot
    }

    pub fn get_view(&self) -> Mat4 {
        let up = quat_rotate_vec3(&self.orientation, &self.initial_up);
        let direction = quat_rotate_vec3(&self.orientation, &self.initial_direction);
        look_at(&self.position, &(&self.position + &direction), &up)
    }
}