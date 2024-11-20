use nalgebra_glm as glm;

/// Tracks mouse action and cursor kinematics in NDC.
pub struct MouseCursor {
    primary_button: MouseState,
    current_position: glm::Vec2,
    current_velocity: glm::Vec2,
    win_width: i32,
    win_height: i32,
}

#[derive(Default, Copy, Clone)]
pub enum MouseState {
    #[default]
    Up,
    Down,
}

impl MouseCursor {
    pub fn new(win_width: i32, win_height: i32) -> Self {
        Self {
            win_width,
            win_height,
            primary_button: MouseState::default(),
            current_position: glm::Vec2::default(),
            current_velocity: glm::Vec2::default(),
        }
    }

    /// `x` and `y` are expected to be screen coordinates; it will be converted to NDC.
    pub fn update_position_and_velocity(&mut self, x: f32, y: f32, dtime: f32) {
        let new_ndc_x = (2.0 * x / self.win_width as f32) - 1.0;
        let new_ndc_y = 1.0 - (2.0 * y / self.win_height as f32);
        self.update_position_and_velocity_ndc(new_ndc_x, new_ndc_y, dtime);
    }

    pub fn update_position_and_velocity_ndc(&mut self, ndc_x: f32, ndc_y: f32, dtime: f32) {
        let initial_x = self.current_position.x;
        let initial_y = self.current_position.y;

        let x_velocity = (ndc_x - initial_x) / dtime;
        let y_velocity = (ndc_y - initial_y) / dtime;

        self.current_position.x = ndc_x;
        self.current_position.y = ndc_y;
        self.current_velocity.x = x_velocity;
        self.current_velocity.y = y_velocity;
    }

    pub fn down(&mut self) {
        self.primary_button = MouseState::Down;
    }

    pub fn up(&mut self) {
        self.primary_button = MouseState::Up;
    }

    pub fn primary_button_is_held(&self) -> bool {
        matches!(self.primary_button, MouseState::Down)
    }

    pub fn get_velocity(&self) -> &glm::Vec2 {
        &self.current_velocity
    }

    pub fn get_position(&self) -> &glm::Vec2 {
        &self.current_position
    }

    pub fn update_win_dimensions(&mut self, width: i32, height: i32) {
        self.win_width = width;
        self.win_height = height;
    }
}
