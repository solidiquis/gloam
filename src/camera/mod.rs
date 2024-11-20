use crate::physics::kinematics::linear;
use nalgebra_glm as glm;

/// Free camera. TODO: Move camera code into trait and implement free camera and orbital camera.
pub struct Camera {
    pub position: glm::Vec3,
    pub target: glm::Vec3,
    pub up: glm::Vec3,
    pub movement_angular_velocity: f32,
    pub movement_linear_velocity: f32,
}

impl Camera {
    pub fn new(
        position: glm::Vec3,
        target: glm::Vec3,
        up: glm::Vec3,
        movement_linear_velocity: f32,
        movement_angular_velocity: f32,
    ) -> Self {
        Self {
            position,
            target,
            up,
            movement_linear_velocity,
            movement_angular_velocity,
        }
    }

    pub fn move_forward(&mut self, dtime: f32) {
        let direction = glm::normalize(&(self.target - self.position));
        self.translate(direction, dtime);
    }

    pub fn move_forward_right(&mut self, dtime: f32) {
        let forward = glm::normalize(&(self.target - self.position));
        let left = -glm::normalize(&glm::cross(&self.up, &forward));
        let direction = glm::normalize(&(forward + left));
        self.translate(direction, dtime);
    }

    pub fn move_forward_left(&mut self, dtime: f32) {
        let forward = glm::normalize(&(self.target - self.position));
        let right = glm::normalize(&glm::cross(&self.up, &forward));
        let direction = glm::normalize(&(forward + right));
        self.translate(direction, dtime);
    }

    pub fn move_backward_right(&mut self, dtime: f32) {
        let backward = -glm::normalize(&(self.target - self.position));
        let right = glm::normalize(&glm::cross(&self.up, &backward));
        let direction = glm::normalize(&(backward + right));
        self.translate(direction, dtime);
    }

    pub fn move_backward_left(&mut self, dtime: f32) {
        let backward = -glm::normalize(&(self.target - self.position));
        let left = -glm::normalize(&glm::cross(&self.up, &backward));
        let direction = glm::normalize(&(backward + left));
        self.translate(direction, dtime);
    }

    pub fn move_backward(&mut self, dtime: f32) {
        let direction = glm::normalize(&-(self.target - self.position));
        self.translate(direction, dtime);
    }

    pub fn move_left(&mut self, dtime: f32) {
        let direction_to_target = glm::normalize(&(self.target - self.position));
        let direction = glm::normalize(&glm::cross(&self.up, &direction_to_target));
        self.translate(direction, dtime);
    }

    pub fn move_right(&mut self, dtime: f32) {
        let direction_to_target = glm::normalize(&(self.target - self.position));
        let direction = glm::normalize(&glm::cross(&self.up, &direction_to_target));
        self.translate(-direction, dtime);
    }

    pub fn translate(&mut self, direction: glm::Vec3, dtime: f32) {
        self.target = linear::translate(
            &self.target,
            &direction,
            self.movement_linear_velocity,
            dtime,
        );
        self.position = linear::translate(
            &self.position,
            &direction,
            self.movement_linear_velocity,
            dtime,
        );
    }

    pub fn rotate_to_direction(&mut self, direction: glm::Vec3, dtime: f32) {
        let yaw = direction.x;
        let pitch = direction.y;
        let effective_yaw = self.movement_angular_velocity * yaw * dtime;
        let effective_pitch = self.movement_angular_velocity * pitch * dtime;
        let forward = self.target - self.position;
        let normalized_forward = glm::normalize(&forward);
        let rotated_forward_yaw = glm::rotate_vec3(&normalized_forward, effective_yaw, &self.up);
        let right = glm::normalize(&glm::cross(&self.up, &rotated_forward_yaw));
        let rotated_forward_pitch = glm::rotate_vec3(&rotated_forward_yaw, effective_pitch, &right);
        self.target = self.position + rotated_forward_pitch;
    }

    pub fn get_view_matrix(&self) -> glm::TMat4<f32> {
        glm::look_at(&self.position, &self.target, &self.up)
    }
}
