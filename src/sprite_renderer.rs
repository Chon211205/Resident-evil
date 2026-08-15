use crate::camera::Camera;
use crate::map::{
    Map,
    TAMANO_CELDA,
};
use crate::player::Player;

use crate::raycaster::{
    lanzar_rayo,
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
    for fila in 0..mapa.alto() {
        for columna in 0..mapa.ancho() {
            if mapa.celda(
                fila as i32,
                columna as i32,
            ) == 'K'
            {
                render_objeto(
                    dibujo,
                    mapa,
                    player,
                    camera,
                    fila,
                    columna,
                    key_texture,
                    0.30,
                    offset_x,
                    offset_y,
                    escala_pantalla,
                );
            }
        }
    }
}

pub fn render_ammo_sprites(
    dibujo: &mut RaylibDrawHandle,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
    ammo_texture: &Texture2D,
    offset_x: f32,
    offset_y: f32,
    escala_pantalla: f32,
) {
    for fila in 0..mapa.alto() {
        for columna in 0..mapa.ancho() {
            if mapa.celda(
                fila as i32,
                columna as i32,
            ) == 'A'
            {
                render_objeto(
                    dibujo,
                    mapa,
                    player,
                    camera,
                    fila,
                    columna,
                    ammo_texture,
                    0.38,
                    offset_x,
                    offset_y,
                    escala_pantalla,
                );
            }
        }
    }
}

fn render_objeto(
    dibujo: &mut RaylibDrawHandle,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
    fila: usize,
    columna: usize,
    textura: &Texture2D,
    factor_tamano: f32,
    offset_x: f32,
    offset_y: f32,
    escala_pantalla: f32,
) {
    let objeto_x =
        columna as f32
            * TAMANO_CELDA
            + TAMANO_CELDA / 2.0;

    let objeto_y =
        fila as f32
            * TAMANO_CELDA
            + TAMANO_CELDA / 2.0;

    let dx =
        objeto_x - player.x;

    let dy =
        objeto_y - player.y;

    let distancia =
        (dx * dx + dy * dy)
            .sqrt();

    if distancia < 1.0 {
        return;
    }

    let angulo_objeto =
        dy.atan2(dx);

    let mut diferencia =
        angulo_objeto
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

    let hit =
        lanzar_rayo(
            mapa,
            player.x,
            player.y,
            angulo_objeto,
        );

    if hit.distancia
        < distancia - 4.0
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

    let distancia_corregida =
        distancia
            * diferencia.cos();

    let distancia_segura =
        distancia_corregida
            .max(0.001);

    let altura_celda =
        TAMANO_CELDA
            * distancia_plano
            / distancia_segura;

    let alto_sprite =
        altura_celda
            * factor_tamano;

    let escala_sprite =
        alto_sprite
            / textura.height()
                as f32;

    let ancho_sprite =
        textura.width()
            as f32
            * escala_sprite;

    let suelo_pantalla =
        ALTO_VENTANA as f32 / 2.0
            + camera.vertical_offset
                as f32
            + altura_celda
                / 2.0;

    let x =
        offset_x
            + (
                pantalla_x
                    - ancho_sprite / 2.0
            ) * escala_pantalla;

    let y =
        offset_y
            + (
                suelo_pantalla
                    - alto_sprite
            ) * escala_pantalla;

    dibujo.draw_texture_ex(
        textura,
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