use raylib::prelude::*;
use crate::gamepad;

pub struct Camera {
    pub angle: f32,
    pub vertical_offset: i32,
    pub use_3d_projection: bool,
}

impl Camera {
    pub fn focal_length() -> f32 {
        400.0 / (std::f32::consts::PI / 6.0).tan()
    }

    pub fn pitch_radians(&self) -> f32 {
        (self.vertical_offset as f32 / Self::focal_length()).atan()
    }

    pub fn new() -> Self {
        Self {
            angle: 0.0,
            vertical_offset: 0,
            use_3d_projection: false,
        }
    }

    /// Rectángulo de un sprite vertical anclado a una posición fija del mundo.
    /// Coordenadas relativas al framebuffer de 800×600.
    pub fn project_billboard(
        &self,
        dx: f32,
        dy: f32,
        bottom: f32,
        height: f32,
        aspect: f32,
    ) -> Option<(f32, f32, f32, f32)> {
        const EYE: f32 = 12.5;
        let focal = Self::focal_length();
        let pitch = self.pitch_radians();
        let (sin_yaw, cos_yaw) = self.angle.sin_cos();
        let (sin_pitch, cos_pitch) = pitch.sin_cos();
        let forward_flat = dx * cos_yaw + dy * sin_yaw;
        let horizontal = -dx * sin_yaw + dy * cos_yaw;
        let project = |world_y: f32| {
            let relative_y = world_y - EYE;
            let depth = cos_pitch * forward_flat + sin_pitch * relative_y;
            if depth <= 0.1 { return None; }
            let screen_y = 300.0 - (-sin_pitch * forward_flat + cos_pitch * relative_y) / depth * focal;
            Some((depth, screen_y))
        };
        let (bottom_depth, screen_bottom) = project(bottom)?;
        let (top_depth, screen_top) = project(bottom + height)?;
        let mid_depth = (bottom_depth + top_depth) * 0.5;
        let projected_height = screen_bottom - screen_top;
        if projected_height <= 0.0 { return None; }
        let projected_width = projected_height * aspect;
        let screen_x = 400.0 + horizontal / mid_depth * focal;
        Some((screen_x - projected_width * 0.5, screen_top, projected_width, projected_height))
    }

    /// Posición de un punto del suelo, para efectos anclados al mapa.
    pub fn project_ground(&self, dx: f32, dy: f32) -> Option<(f32, f32, f32)> {
        const EYE: f32 = 12.5;
        let (sin_yaw, cos_yaw) = self.angle.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch_radians().sin_cos();
        let forward_flat = dx * cos_yaw + dy * sin_yaw;
        let horizontal = -dx * sin_yaw + dy * cos_yaw;
        let depth = cos_pitch * forward_flat - sin_pitch * EYE;
        if depth <= 0.1 { return None; }
        let focal = Self::focal_length();
        Some((400.0 + horizontal / depth * focal,
              300.0 + (sin_pitch * forward_flat + cos_pitch * EYE) / depth * focal,
              depth))
    }

pub fn update(
    &mut self,
    window: &RaylibHandle,
    delta_time: f32,
    rotation_speed: f32,
) {
    let mouse_delta =
        window.get_mouse_delta();

    let sensibilidad_x = 0.003;
    let sensibilidad_y = 0.6;

    self.angle +=
        mouse_delta.x
            * sensibilidad_x;

    self.angle +=
        gamepad::eje(
            window,
            GamepadAxis::GAMEPAD_AXIS_RIGHT_X,
        ) * rotation_speed
            * delta_time
            * 1.8;

    // Stick derecho vertical: permite apuntar hacia arriba y abajo.
    // En raylib el eje Y es positivo al mover el stick hacia abajo,
    // igual que el desplazamiento vertical del mouse.
    let mirada_vertical_control =
        gamepad::eje(
            window,
            GamepadAxis::GAMEPAD_AXIS_RIGHT_Y,
        );

    if mirada_vertical_control != 0.0 {
        let movimiento_vertical =
            mirada_vertical_control
                * rotation_speed
                * delta_time
                * 95.0;

        // vertical_offset usa enteros; el minimo evita perder movimientos
        // suaves del stick por el redondeo de cada fotograma.
        self.vertical_offset +=
            movimiento_vertical
                .signum() as i32
                * movimiento_vertical
                    .abs()
                    .max(1.0) as i32;
    }

    self.vertical_offset +=
        (mouse_delta.y * sensibilidad_y)
            as i32;

    if window.is_key_down(
        KeyboardKey::KEY_J,
    ) {
        self.angle -=
            rotation_speed * delta_time;
    }

    if window.is_key_down(
        KeyboardKey::KEY_L,
    ) {
        self.angle +=
            rotation_speed * delta_time;
    }

    self.vertical_offset =
        self.vertical_offset.clamp(
            -150,
            150,
        );

    self.angle =
        normalize_angle(
            self.angle,
        );
}

    pub fn reset(&mut self) {
        self.angle = 0.0;
        self.vertical_offset = 0;
    }
}

fn normalize_angle(
    mut angle: f32,
) -> f32 {
    let full_rotation =
        2.0 * std::f32::consts::PI;

    while angle < 0.0 {
        angle += full_rotation;
    }

    while angle >= full_rotation {
        angle -= full_rotation;
    }

    angle
}

#[cfg(test)]
mod projection_tests {
    use super::Camera;

    #[test]
    fn billboard_is_centered_on_world_position() {
        let camera = Camera::new();
        let (x, y, width, height) = camera.project_billboard(50.0, 0.0, 0.0, 25.0, 0.5).unwrap();
        assert!((x + width * 0.5 - 400.0).abs() < 0.01);
        assert!((y + height * 0.5 - 300.0).abs() < 0.01);
    }

    #[test]
    fn camera_pitch_uses_same_focal_length_as_sprite_projection() {
        let mut camera = Camera::new();
        camera.vertical_offset = 100;
        let (_, y, _, height) = camera.project_billboard(50.0, 0.0, 12.5, 0.01, 1.0).unwrap();
        assert!((y + height * 0.5 - 400.0).abs() < 0.1);
    }

    #[test]
    fn ground_shadow_aligns_with_billboard_feet() {
        let camera = Camera::new();
        let (x, y, width, height) = camera.project_billboard(50.0, 8.0, 0.0, 20.0, 0.5).unwrap();
        let (ground_x, ground_y, _) = camera.project_ground(50.0, 8.0).unwrap();
        assert!((x + width * 0.5 - ground_x).abs() < 0.01);
        assert!((y + height - ground_y).abs() < 0.01);
    }
}
