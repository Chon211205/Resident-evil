use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::map::{Map, TAMANO_CELDA};
use crate::player::Player;

use raylib::prelude::*;

const MINIMAPA_X: i32 = 15;
const MINIMAPA_Y: i32 = 15;
const TAMANO_MINIMAPA: i32 = 180;

pub fn render_minimap(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
) {
    let escala_x =
        TAMANO_MINIMAPA as f32
            / mapa.ancho() as f32;

    let escala_y =
        TAMANO_MINIMAPA as f32
            / mapa.alto() as f32;

    let escala =
        escala_x.min(escala_y);

    let ancho_mapa =
        mapa.ancho() as f32
            * escala;

    let alto_mapa =
        mapa.alto() as f32
            * escala;

    dibujar_fondo(
        framebuffer,
        MINIMAPA_X - 5,
        MINIMAPA_Y - 5,
        ancho_mapa as i32 + 10,
        alto_mapa as i32 + 10,
    );

    for fila in 0..mapa.alto() {
        for columna in 0..mapa.ancho() {
            let celda =
                mapa.celda(
                    fila as i32,
                    columna as i32,
                );

            let x =
                MINIMAPA_X
                    + (columna as f32
                        * escala) as i32;

            let y =
                MINIMAPA_Y
                    + (fila as f32
                        * escala) as i32;

            match celda {
                '#' => {
                    framebuffer.set_current_color(
                        Color::LIGHTGRAY,
                    );

                    dibujar_rectangulo(
                        framebuffer,
                        x,
                        y,
                        escala.ceil() as i32,
                        escala.ceil() as i32,
                    );
                }

                'E' => {
                    framebuffer.set_current_color(
                        Color::RED,
                    );

                    framebuffer.point_with_size(
                        x,
                        y,
                        2,
                    );
                }

                _ => {}
            }
        }
    }

    let player_x =
        MINIMAPA_X as f32
            + player.x
                / TAMANO_CELDA
                * escala;

    let player_y =
        MINIMAPA_Y as f32
            + player.y
                / TAMANO_CELDA
                * escala;

    framebuffer.set_current_color(
        Color::YELLOW,
    );

    framebuffer.point_with_size(
        player_x as i32,
        player_y as i32,
        4,
    );

    let direccion_x =
        player_x
            + camera.angle.cos()
                * 12.0;

    let direccion_y =
        player_y
            + camera.angle.sin()
                * 12.0;

    framebuffer.set_current_color(
        Color::GREEN,
    );

    framebuffer.dotted_line(
        player_x as i32,
        player_y as i32,
        direccion_x as i32,
        direccion_y as i32,
        2.0,
    );
}

fn dibujar_fondo(
    framebuffer: &mut Framebuffer,
    x: i32,
    y: i32,
    ancho: i32,
    alto: i32,
) {
    framebuffer.set_current_color(
        Color::new(
            10,
            10,
            10,
            220,
        ),
    );

    dibujar_rectangulo(
        framebuffer,
        x,
        y,
        ancho,
        alto,
    );
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