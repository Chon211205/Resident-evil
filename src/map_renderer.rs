use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::map::{
    Map,
    TAMANO_CELDA,
};
use crate::player::Player;

use raylib::prelude::*;

fn pintar_rectangulo(
    framebuffer: &mut Framebuffer,
    x: i32,
    y: i32,
    ancho: i32,
    alto: i32,
    color: Color,
) {
    for py in y..(y + alto) {
        for px in x..(x + ancho) {
            framebuffer.point_color(
                px,
                py,
                color,
            );
        }
    }
}

fn pintar_borde_rectangulo(
    framebuffer: &mut Framebuffer,
    x: i32,
    y: i32,
    ancho: i32,
    alto: i32,
    color: Color,
) {
    for px in x..(x + ancho) {
        framebuffer.point_color(
            px,
            y,
            color,
        );

        framebuffer.point_color(
            px,
            y + alto - 1,
            color,
        );
    }

    for py in y..(y + alto) {
        framebuffer.point_color(
            x,
            py,
            color,
        );

        framebuffer.point_color(
            x + ancho - 1,
            py,
            color,
        );
    }
}

fn pintar_circulo(
    framebuffer: &mut Framebuffer,
    centro_x: i32,
    centro_y: i32,
    radio: i32,
    color: Color,
) {
    for y in -radio..=radio {
        for x in -radio..=radio {
            if x * x + y * y
                <= radio * radio
            {
                framebuffer.point_color(
                    centro_x + x,
                    centro_y + y,
                    color,
                );
            }
        }
    }
}

fn pintar_linea(
    framebuffer: &mut Framebuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    let mut x0 =
        x0;

    let mut y0 =
        y0;

    let dx =
        (x1 - x0).abs();

    let sx =
        if x0 < x1 {
            1
        } else {
            -1
        };

    let dy =
        -(y1 - y0).abs();

    let sy =
        if y0 < y1 {
            1
        } else {
            -1
        };

    let mut error =
        dx + dy;

    loop {
        framebuffer.point_color(
            x0,
            y0,
            color,
        );

        if x0 == x1
            && y0 == y1
        {
            break;
        }

        let error2 =
            2 * error;

        if error2 >= dy {
            error +=
                dy;

            x0 +=
                sx;
        }

        if error2 <= dx {
            error +=
                dx;

            y0 +=
                sy;
        }
    }
}

pub fn render_minimap(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
) {
    let mapa_ancho =
        mapa.ancho();

    let mapa_alto =
        mapa.alto();

    if mapa_ancho == 0
        || mapa_alto == 0
    {
        return;
    }

    let max_ancho =
        125.0;

    let max_alto =
        90.0;

    let escala_x =
        max_ancho
            / mapa_ancho as f32;

    let escala_y =
        max_alto
            / mapa_alto as f32;

    let escala =
        escala_x
            .min(
                escala_y,
            )
            .max(
                1.0,
            );

    let margen =
        12;

    let origen_x =
        margen;

    let origen_y =
        margen;

    let ancho =
        (
            mapa_ancho as f32
                * escala
        )
            .ceil()
            as i32;

    let alto =
        (
            mapa_alto as f32
                * escala
        )
            .ceil()
            as i32;

    pintar_rectangulo(
        framebuffer,
        origen_x - 5,
        origen_y - 5,
        ancho + 10,
        alto + 10,
        Color::new(
            10,
            10,
            10,
            230,
        ),
    );

    pintar_borde_rectangulo(
        framebuffer,
        origen_x - 5,
        origen_y - 5,
        ancho + 10,
        alto + 10,
        Color::GRAY,
    );

    for fila in 0..mapa_alto {
        for columna in 0..mapa_ancho {
            let celda =
                mapa.celda(
                    fila as i32,
                    columna as i32,
                );

            let color =
                match celda {
                    '#' => {
                        Some(
                            Color::LIGHTGRAY,
                        )
                    }

                    'D' => {
                        Some(
                            Color::BROWN,
                        )
                    }

                    'O' => {
                        Some(
                            Color::DARKBROWN,
                        )
                    }

                    'K' => {
                        Some(
                            Color::YELLOW,
                        )
                    }

                    'A' => {
                        Some(
                            Color::ORANGE,
                        )
                    }

                    'H' => {
                        Some(
                            Color::GREEN,
                        )
                    }

                    _ => {
                        None
                    }
                };

            let Some(color) =
                color
            else {
                continue;
            };

            let x =
                origen_x
                    + (
                        columna as f32
                            * escala
                    ) as i32;

            let y =
                origen_y
                    + (
                        fila as f32
                            * escala
                    ) as i32;

            let tamano =
                escala
                    .ceil()
                    .max(
                        1.0,
                    ) as i32;

            pintar_rectangulo(
                framebuffer,
                x,
                y,
                tamano,
                tamano,
                color,
            );
        }
    }

    let jugador_columna =
        player.x
            / TAMANO_CELDA;

    let jugador_fila =
        player.y
            / TAMANO_CELDA;

    let jugador_x =
        origen_x as f32
            + jugador_columna
                * escala;

    let jugador_y =
        origen_y as f32
            + jugador_fila
                * escala;

    pintar_circulo(
        framebuffer,
        jugador_x.round() as i32,
        jugador_y.round() as i32,
        2,
        Color::RED,
    );

    let largo =
        6.0;

    let direccion_x =
        jugador_x
            + camera.angle.cos()
                * largo;

    let direccion_y =
        jugador_y
            + camera.angle.sin()
                * largo;

    pintar_linea(
        framebuffer,
        jugador_x.round() as i32,
        jugador_y.round() as i32,
        direccion_x.round() as i32,
        direccion_y.round() as i32,
        Color::YELLOW,
    );
}