use crate::camera::Camera;
use crate::map::{Map, TAMANO_CELDA};
use crate::player::Player;
use crate::raycaster::{
    lanzar_rayo,
    ALTO_VENTANA,
    ANCHO_VENTANA,
    FOV,
};
use crate::zombie::Zombie;

use raylib::prelude::*;
use std::f32::consts::PI;

pub fn render_zombies(
    dibujo: &mut RaylibDrawHandle,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
    zombies: &[Zombie],
    zombie_idle: &Texture2D,
    zombie_run1: &Texture2D,
    zombie_run2: &Texture2D,
    offset_x: f32,
    offset_y: f32,
    escala_pantalla: f32,
) {
    for zombie in zombies {
        if !zombie.vivo {
            continue;
        }

        let textura_actual =
            obtener_textura_zombie(
                zombie,
                zombie_idle,
                zombie_run1,
                zombie_run2,
            );

        render_zombie(
            dibujo,
            mapa,
            player,
            camera,
            zombie,
            textura_actual,
            offset_x,
            offset_y,
            escala_pantalla,
        );
    }
}

fn obtener_textura_zombie<'a>(
    zombie: &Zombie,
    zombie_idle: &'a Texture2D,
    zombie_run1: &'a Texture2D,
    zombie_run2: &'a Texture2D,
) -> &'a Texture2D {
    if !zombie.persiguiendo {
        return zombie_idle;
    }

    let velocidad_animacion =
        6.0;

    let frame =
        (
            zombie.tiempo_animacion
                * velocidad_animacion
        ) as i32
            % 2;

    if frame == 0 {
        zombie_run1
    } else {
        zombie_run2
    }
}

fn render_zombie(
    dibujo: &mut RaylibDrawHandle,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
    zombie: &Zombie,
    zombie_texture: &Texture2D,
    offset_x: f32,
    offset_y: f32,
    escala_pantalla: f32,
) {
    let dx =
        zombie.x - player.x;

    let dy =
        zombie.y - player.y;

    let distancia =
        (dx * dx + dy * dy)
            .sqrt();

    if distancia < 1.0 {
        return;
    }

    let angulo_zombie =
        dy.atan2(dx);

    let mut diferencia =
        angulo_zombie
            - camera.angle;

    while diferencia > PI {
        diferencia -=
            2.0 * PI;
    }

    while diferencia < -PI {
        diferencia +=
            2.0 * PI;
    }

    // Fuera del campo visual.
    if diferencia.abs()
        > FOV / 2.0
    {
        return;
    }

    // Evita ver zombies
    // a través de paredes.
    let hit =
        lanzar_rayo(
            mapa,
            player.x,
            player.y,
            angulo_zombie,
        );

    if hit.distancia
        < distancia - 5.0
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

    // Altura de una celda
    // proyectada a esta distancia.
    let altura_celda_proyectada =
        TAMANO_CELDA
            * distancia_plano
            / distancia_segura;

    // Tamaño del zombie.
    // Si lo querés más grande o pequeño,
    // cambiá este número.
    let alto_sprite =
        altura_celda_proyectada
            * 0.65;

    let escala_sprite =
        alto_sprite
            / zombie_texture.height()
                as f32;

    let ancho_sprite =
        zombie_texture.width()
            as f32
            * escala_sprite;

    // Posición del piso
    // según la distancia.
    let suelo_pantalla =
        ALTO_VENTANA as f32
            / 2.0
            + camera.vertical_offset
                as f32
            + altura_celda_proyectada
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
        zombie_texture,
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