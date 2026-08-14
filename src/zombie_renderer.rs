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
    zombie_texture: &Texture2D,
    offset_x: f32,
    offset_y: f32,
    escala_pantalla: f32,
) {
    for zombie in zombies {
        if !zombie.vivo {
            continue;
        }

        render_zombie(
            dibujo,
            mapa,
            player,
            camera,
            zombie,
            zombie_texture,
            offset_x,
            offset_y,
            escala_pantalla,
        );
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

    if diferencia.abs() > FOV / 2.0 {
        return;
    }

    // Evita que el zombie se vea atravesando paredes.
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

    // Posición horizontal del zombie en pantalla.
    let pantalla_x =
        ANCHO_VENTANA as f32 / 2.0
            + diferencia.tan()
                * distancia_plano;

    // Corrección de distancia para que el sprite
    // coincida mejor con la perspectiva de las paredes.
    let distancia_corregida =
        distancia
            * diferencia.cos();

    let distancia_segura =
        distancia_corregida
            .max(0.001);

    // Altura que tendría una celda del mundo
    // a esta distancia.
    let altura_celda_proyectada =
        TAMANO_CELDA
            * distancia_plano
            / distancia_segura;

    // El zombie mide aproximadamente
    // 1.8 veces la altura de una celda.
    let alto_sprite =
        altura_celda_proyectada
            * 0.7;

    let escala_sprite =
        alto_sprite
            / zombie_texture.height()
                as f32;

    let ancho_sprite =
        zombie_texture.width()
            as f32
            * escala_sprite;

    // El suelo está en la parte inferior
    // de la celda proyectada.
    let suelo_pantalla =
        ALTO_VENTANA as f32 / 2.0
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

    // Los pies quedan exactamente en suelo_pantalla.
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