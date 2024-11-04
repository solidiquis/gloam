pub mod kinematics {
    pub mod linear {
        use nalgebra_glm as glm;

        pub fn translate(
            initial_position: &glm::Vec3,
            direction: &glm::Vec3,
            velocity: f32,
            dtime: f32,
        ) -> glm::Vec3 {
            let normalized_direction = glm::normalize(direction);
            let displacement = normalized_direction * velocity * dtime;
            initial_position + displacement
        }
    }

    pub mod angular {
        use nalgebra_glm as glm;

        pub fn rotate_to_direction(
            position: &glm::Vec3,
            up: &glm::Vec3,
            target: &glm::Vec3,
            direction: &glm::Vec3,
            angular_velocity: f32,
            dtime: f32,
        ) -> glm::Vec3 {
            let yaw = direction.x;
            let pitch = direction.y;
            let effective_yaw = angular_velocity * yaw * dtime;
            let effective_pitch = angular_velocity * pitch * dtime;
            let forward = target - position;
            let normalized_forward = glm::normalize(&forward);
            let rotated_forward_yaw = glm::rotate_vec3(&normalized_forward, effective_yaw, up);
            let right = glm::normalize(&glm::cross(up, &rotated_forward_yaw));
            let rotated_forward_pitch =
                glm::rotate_vec3(&rotated_forward_yaw, effective_pitch, &right);
            position + rotated_forward_pitch
        }
    }
}
