use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::map::{Map, TAMANO_CELDA};
use crate::player::Player;
use crate::raycaster::{
    lanzar_rayo,
    ALTO_VENTANA,
    ANCHO_VENTANA,
    FOV,
};

use raylib::prelude::*;

pub fn render_map_2d(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
) {
    let escala =
        calcular_escala_mapa(mapa);

    let ancho_mapa =
        mapa.ancho() as f32 * escala;

    let alto_mapa =
        mapa.alto() as f32 * escala;

    let offset_x =
        (ANCHO_VENTANA as f32 - ancho_mapa) / 2.0;

    let offset_y =
        (ALTO_VENTANA as f32 - alto_mapa) / 2.0;

    for fila in 0..mapa.alto() {
        for columna in 0..mapa.ancho() {
            let celda =
                mapa.celda(
                    fila as i32,
                    columna as i32,
                );

            let x =
                offset_x as i32
                    + columna as i32
                        * escala as i32;

            let y =
                offset_y as i32
                    + fila as i32
                        * escala as i32;

            match celda {
                '#' => {
                    framebuffer
                        .set_current_color(
                            Color::DARKGRAY,
                        );

                    dibujar_rectangulo(
                        framebuffer,
                        x,
                        y,
                        escala as i32,
                        escala as i32,
                    );
                }

                'E' => {
                    framebuffer
                        .set_current_color(
                            Color::RED,
                        );

                    framebuffer
                        .point_with_size(
                            x + escala as i32 / 2,
                            y + escala as i32 / 2,
                            4,
                        );
                }

                _ => {}
            }
        }
    }

    let player_mapa_x =
        offset_x
            + player.x
                / TAMANO_CELDA
                * escala;

    let player_mapa_y =
        offset_y
            + player.y
                / TAMANO_CELDA
                * escala;

    dibujar_fov(
        framebuffer,
        mapa,
        player,
        camera,
        player_mapa_x,
        player_mapa_y,
        offset_x,
        offset_y,
        escala,
    );

    framebuffer
        .set_current_color(
            Color::YELLOW,
        );

    framebuffer
        .point_with_size(
            player_mapa_x as i32,
            player_mapa_y as i32,
            6,
        );

    dibujar_direccion(
        framebuffer,
        camera,
        player_mapa_x,
        player_mapa_y,
    );
}

fn dibujar_fov(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
    player_mapa_x: f32,
    player_mapa_y: f32,
    offset_x: f32,
    offset_y: f32,
    escala: f32,
) {
    let cantidad_rayitos = 40;

    let angulo_inicial =
        camera.angle - FOV / 2.0;

    for rayo in 0..cantidad_rayitos {
        let angulo_rayo =
            angulo_inicial
                + FOV
                    * rayo as f32
                    / cantidad_rayitos as f32;

        let distancia =
            lanzar_rayo(
                mapa,
                player.x,
                player.y,
                angulo_rayo,
            );

        let choque_x =
            player.x
                + angulo_rayo.cos()
                    * distancia;

        let choque_y =
            player.y
                + angulo_rayo.sin()
                    * distancia;

        let choque_mapa_x =
            offset_x
                + choque_x
                    / TAMANO_CELDA
                    * escala;

        let choque_mapa_y =
            offset_y
                + choque_y
                    / TAMANO_CELDA
                    * escala;

        framebuffer
            .set_current_color(
                Color::RED,
            );

        framebuffer.dotted_line(
            player_mapa_x as i32,
            player_mapa_y as i32,
            choque_mapa_x as i32,
            choque_mapa_y as i32,
            7.0,
        );
    }
}

fn dibujar_direccion(
    framebuffer: &mut Framebuffer,
    camera: &Camera,
    player_x: f32,
    player_y: f32,
) {
    let direccion_x =
        player_x
            + camera.angle.cos()
                * 20.0;

    let direccion_y =
        player_y
            + camera.angle.sin()
                * 20.0;

    framebuffer
        .set_current_color(
            Color::GREEN,
        );

    framebuffer.dotted_line(
        player_x as i32,
        player_y as i32,
        direccion_x as i32,
        direccion_y as i32,
        3.0,
    );
}

fn calcular_escala_mapa(
    mapa: &Map,
) -> f32 {
    let escala_x =
        ANCHO_VENTANA as f32
            / mapa.ancho() as f32;

    let escala_y =
        ALTO_VENTANA as f32
            / mapa.alto() as f32;

    escala_x.min(escala_y)
        * 0.9
}

fn dibujar_rectangulo(
    framebuffer: &mut Framebuffer,
    x: i32,
    y: i32,
    ancho: i32,
    alto: i32,
) {
    if ancho <= 0 || alto <= 0 {
        return;
    }

    for pixel_y in y..y + alto {
        for pixel_x in x..x + ancho {
            framebuffer.point(
                pixel_x,
                pixel_y,
            );
        }
    }
}