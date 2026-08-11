mod camera;
mod framebuffer;
mod map;
mod player;
mod raycaster;

use camera::Camera;
use framebuffer::Framebuffer;
use map::{Map, TAMANO_CELDA};
use player::Player;

use raycaster::{
    lanzar_rayo,
    render_3d,
    ALTO_VENTANA,
    ANCHO_VENTANA,
    FOV,
};

use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Vista {
    Mapa2D,
    Vista3D,
}

fn main() {
    let mapa = Map::new();

    mapa.guardar_txt("mapa_resident.txt");

    let mut player = Player::new(&mapa);
    let mut camera = Camera::new();

    let mut vista_actual = Vista::Vista3D;

    let mut framebuffer =
        Framebuffer::new(
            ANCHO_VENTANA,
            ALTO_VENTANA,
        );

    framebuffer.set_background_color(
        Color::BLACK,
    );

    let (mut ventana, thread) =
        raylib::init()
            .size(
                ANCHO_VENTANA,
                ALTO_VENTANA + 55,
            )
            .title("Survival Horror Raycasting")
            .build();

    ventana.set_target_fps(60);

    while !ventana.window_should_close() {
        let delta_time =
            ventana.get_frame_time();

        camera.update(
            &ventana,
            delta_time,
            2.0,
        );

        player.update(
            &ventana,
            &mapa,
            camera.angle,
            delta_time,
        );

        if ventana.is_key_pressed(
            KeyboardKey::KEY_M,
        ) {
            vista_actual =
                match vista_actual {
                    Vista::Vista3D =>
                        Vista::Mapa2D,

                    Vista::Mapa2D =>
                        Vista::Vista3D,
                };
        }

        if ventana.is_key_pressed(
            KeyboardKey::KEY_R,
        ) {
            player.reset();
            camera.reset();
        }

        framebuffer.clear();

        match vista_actual {
            Vista::Vista3D => {
                render_3d(
                    &mut framebuffer,
                    &mapa,
                    &player,
                    &camera,
                );
            }

            Vista::Mapa2D => {
                dibujar_mapa_2d(
                    &mut framebuffer,
                    &mapa,
                    &player,
                    &camera,
                );
            }
        }

        let textura =
            ventana
                .load_texture_from_image(
                    &thread,
                    framebuffer.image(),
                )
                .expect(
                    "No se pudo crear la textura",
                );

        let mut dibujo =
            ventana.begin_drawing(
                &thread,
            );

        dibujo.clear_background(
            Color::BLACK,
        );

        dibujo.draw_texture(
            &textura,
            0,
            0,
            Color::WHITE,
        );

        dibujo.draw_rectangle(
            0,
            ALTO_VENTANA,
            ANCHO_VENTANA,
            55,
            Color::new(
                15,
                15,
                15,
                255,
            ),
        );

        let nombre_vista =
            match vista_actual {
                Vista::Vista3D => "3D",
                Vista::Mapa2D => "Mapa 2D",
            };

        dibujo.draw_text(
            &format!(
                "Vista: {} | Angulo: {:.1}",
                nombre_vista,
                camera.angle.to_degrees(),
            ),
            10,
            ALTO_VENTANA + 5,
            18,
            Color::WHITE,
        );

        dibujo.draw_text(
            "WASD: mover | J/L: girar | I/K: camara | M: mapa | R: reset",
            10,
            ALTO_VENTANA + 30,
            14,
            Color::LIGHTGRAY,
        );
    }
}

fn dibujar_mapa_2d(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
) {
    let escala =
        calcular_escala_mapa(mapa);

    let ancho_mapa =
        mapa.ancho() as f32
            * escala;

    let alto_mapa =
        mapa.alto() as f32
            * escala;

    let offset_x =
        (ANCHO_VENTANA as f32
            - ancho_mapa)
            / 2.0;

    let offset_y =
        (ALTO_VENTANA as f32
            - alto_mapa)
            / 2.0;

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
                            x + escala as i32
                                / 2,
                            y + escala as i32
                                / 2,
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

    let cantidad_rayitos = 40;

    let angulo_inicial =
        camera.angle
            - FOV / 2.0;

    for rayo in 0..cantidad_rayitos {
        let angulo_rayo =
            angulo_inicial
                + FOV
                    * rayo as f32
                    / cantidad_rayitos
                        as f32;

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

    framebuffer.set_current_color(
        Color::YELLOW,
    );

    framebuffer.point_with_size(
        player_mapa_x as i32,
        player_mapa_y as i32,
        6,
    );

    let direccion_x =
        player_mapa_x
            + camera.angle.cos()
                * 20.0;

    let direccion_y =
        player_mapa_y
            + camera.angle.sin()
                * 20.0;

    framebuffer.set_current_color(
        Color::GREEN,
    );

    framebuffer.dotted_line(
        player_mapa_x as i32,
        player_mapa_y as i32,
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