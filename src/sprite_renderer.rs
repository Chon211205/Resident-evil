use crate::camera::Camera;
use crate::map::{Map, TAMANO_CELDA};
use crate::player::Player;
use crate::raycaster::{
    ALTO_VENTANA,
    ANCHO_VENTANA,
    FOV,
};

use raylib::prelude::*;
use std::f32::consts::PI;

pub fn render_key_sprite(
    dibujo: &mut RaylibDrawHandle,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
    key_texture: &Texture2D,
    offset_x: f32,
    offset_y: f32,
    escala_pantalla: f32,
) {
    let Some((fila, columna)) =
        buscar_llave(mapa)
    else {
        return;
    };

    let key_x =
        columna as f32
            * TAMANO_CELDA
            + TAMANO_CELDA / 2.0;

    let key_y =
        fila as f32
            * TAMANO_CELDA
            + TAMANO_CELDA / 2.0;

    let dx =
        key_x - player.x;

    let dy =
        key_y - player.y;

    let distancia =
        (dx * dx + dy * dy)
            .sqrt();

    if distancia <= 1.0 {
        return;
    }

    let angulo_llave =
        dy.atan2(dx);

    let mut diferencia =
        angulo_llave
            - camera.angle;

    while diferencia > PI {
        diferencia -=
            2.0 * PI;
    }

    while diferencia < -PI {
        diferencia +=
            2.0 * PI;
    }

    if diferencia.abs()
        > FOV / 2.0
    {
        return;
    }

    let distancia_plano =
        (ANCHO_VENTANA as f32 / 2.0)
            / (FOV / 2.0).tan();

    let pantalla_x =
        ANCHO_VENTANA as f32 / 2.0
            + diferencia.tan()
                * distancia_plano;

    let tamano =
        (1200.0 / distancia)
            .clamp(
                12.0,
                90.0,
            );

    let escala_sprite =
        tamano
            / key_texture.height()
                as f32;

    let ancho_sprite =
        key_texture.width() as f32
            * escala_sprite;

    let alto_sprite =
        key_texture.height() as f32
            * escala_sprite;

    let x =
        offset_x
            + (
                pantalla_x
                    - ancho_sprite / 2.0
            )
                * escala_pantalla;

    let suelo_pantalla =
        ALTO_VENTANA as f32
            * 0.62
            + camera.vertical_offset
                as f32;

    let y =
        offset_y
            + (
                suelo_pantalla
                    - alto_sprite
            )
                * escala_pantalla;

    dibujo.draw_texture_ex(
        key_texture,
        Vector2::new(
            x,
            y,
        ),
        0.0,
        escala_sprite
            * escala_pantalla,
        Color::WHITE,
    );
}

fn buscar_llave(
    mapa: &Map,
) -> Option<(usize, usize)> {
    for fila in 0..mapa.alto() {
        for columna in 0..mapa.ancho() {
            if mapa.celda(
                fila as i32,
                columna as i32,
            ) == 'K'
            {
                return Some(
                    (
                        fila,
                        columna,
                    ),
                );
            }
        }
    }

    None
}