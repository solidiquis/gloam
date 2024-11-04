use crate::physics::kinematics::{angular, linear};
use nalgebra_glm as glm;

/// Free camera. TODO: Move camera code into trait and implement free camera and orbital camera.
pub struct Camera {
    position: glm::Vec3,
    target: glm::Vec3,
    up: glm::Vec3,
    movement_angular_velocity: f32,
    movement_linear_velocity: f32,
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
        self.target = angular::rotate_to_direction(
            &self.position,
            &self.up,
            &self.target,
            &direction,
            self.movement_angular_velocity,
            dtime,
        )
    }

    pub fn get_view_matrix(&self) -> glm::TMat4<f32> {
        glm::look_at(&self.position, &self.target, &self.up)
    }
}
