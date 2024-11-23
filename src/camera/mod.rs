use nalgebra_glm as glm;

pub mod free;
pub use free::FreeCamera;

pub trait Camera {
    fn get_view_matrix(&self) -> glm::TMat4<f32>;
}
