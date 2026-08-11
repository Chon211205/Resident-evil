use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Camera {
    pub angle: f32,
    pub vertical_offset: i32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            angle: 0.0,
            vertical_offset: 0,
        }
    }

    pub fn update(
        &mut self,
        window: &RaylibHandle,
        delta_time: f32,
        rotation_speed: f32,
    ) {
        if window.is_key_down(KeyboardKey::KEY_J) {
            self.angle -= rotation_speed * delta_time;
        }

        if window.is_key_down(KeyboardKey::KEY_L) {
            self.angle += rotation_speed * delta_time;
        }

        if window.is_key_down(KeyboardKey::KEY_I) {
            self.vertical_offset -= 3;
        }

        if window.is_key_down(KeyboardKey::KEY_K) {
            self.vertical_offset += 3;
        }

        self.vertical_offset =
            self.vertical_offset.clamp(-150, 150);

        self.angle = normalize_angle(self.angle);
    }

    pub fn reset(&mut self) {
        self.angle = 0.0;
        self.vertical_offset = 0;
    }
}

fn normalize_angle(mut angle: f32) -> f32 {
    let full_rotation = 2.0 * PI;

    while angle < 0.0 {
        angle += full_rotation;
    }

    while angle >= full_rotation {
        angle -= full_rotation;
    }

    angle
}