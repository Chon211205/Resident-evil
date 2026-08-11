use crate::map::{Map, TAMANO_CELDA};
use raylib::prelude::*;
use std::f32::consts::PI;

pub const VELOCIDAD_MOVIMIENTO: f32 = 100.0;

pub struct Player {
    pub x: f32,
    pub y: f32,

    initial_x: f32,
    initial_y: f32,
}

impl Player {
    pub fn new(map: &Map) -> Self {
        let (fila, columna) = map
            .buscar_jugador()
            .expect("No se encontró el jugador P");

        let x =
            columna as f32 * TAMANO_CELDA
                + TAMANO_CELDA / 2.0;

        let y =
            fila as f32 * TAMANO_CELDA
                + TAMANO_CELDA / 2.0;

        Self {
            x,
            y,
            initial_x: x,
            initial_y: y,
        }
    }

    pub fn update(
        &mut self,
        window: &RaylibHandle,
        map: &Map,
        camera_angle: f32,
        delta_time: f32,
    ) {
        let velocidad =
            VELOCIDAD_MOVIMIENTO * delta_time;

        // W = adelante
        if window.is_key_down(KeyboardKey::KEY_W) {
            self.mover(
                map,
                camera_angle,
                velocidad,
            );
        }

        // S = atrás
        if window.is_key_down(KeyboardKey::KEY_S) {
            self.mover(
                map,
                camera_angle,
                -velocidad,
            );
        }

        // A = izquierda
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.mover(
                map,
                camera_angle - PI / 2.0,
                velocidad,
            );
        }

        // D = derecha
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.mover(
                map,
                camera_angle + PI / 2.0,
                velocidad,
            );
        }
    }

    fn mover(
        &mut self,
        map: &Map,
        angle: f32,
        movimiento: f32,
    ) {
        let nuevo_x =
            self.x + angle.cos() * movimiento;

        let nuevo_y =
            self.y + angle.sin() * movimiento;

        if !map.es_pared(
            nuevo_x,
            self.y,
        ) {
            self.x = nuevo_x;
        }

        if !map.es_pared(
            self.x,
            nuevo_y,
        ) {
            self.y = nuevo_y;
        }
    }

    pub fn reset(&mut self) {
        self.x = self.initial_x;
        self.y = self.initial_y;
    }
}