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
}
