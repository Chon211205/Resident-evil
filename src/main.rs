mod camera;
mod framebuffer;
mod map;
mod map_renderer;
mod player;
mod raycaster;

use camera::Camera;
use framebuffer::Framebuffer;
use map::Map;
use map_renderer::render_map_2d;
use player::Player;

use raycaster::{
    render_3d,
    ALTO_VENTANA,
    ANCHO_VENTANA,
};

use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Vista {
    Mapa2D,
    Vista3D,
}

fn main() {
    let mapa = Map::new();

    mapa.guardar_txt(
        "mapa_resident.txt",
    );

    let mut player =
        Player::new(&mapa);

    let mut camera =
        Camera::new();

    let mut vista_actual =
        Vista::Vista3D;

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
            .title(
                "Survival Horror Raycasting",
            )
            .build();

    ventana.set_target_fps(60);

    let pistol1 =
        ventana
            .load_texture(
                &thread,
                "assets/pistol1.png",
            )
            .expect(
                "No se pudo cargar assets/pistol1.png",
            );

    let pistol2 =
        ventana
            .load_texture(
                &thread,
                "assets/pistol2.png",
            )
            .expect(
                "No se pudo cargar assets/pistol2.png",
            );

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

        let apuntando =
            ventana.is_mouse_button_down(
                MouseButton::MOUSE_BUTTON_RIGHT,
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
                render_map_2d(
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

        if vista_actual == Vista::Vista3D {
            let arma_actual =
                if apuntando {
                    &pistol2
                } else {
                    &pistol1
                };

            let escala_arma =
                if apuntando {
                    0.70
                } else {
                    0.65
                };

            let arma_x =
                (
                    ANCHO_VENTANA as f32
                        - arma_actual.width() as f32
                            * escala_arma
                ) / 2.0;

            let arma_y =
                ALTO_VENTANA as f32
                    - arma_actual.height() as f32
                        * escala_arma;

            dibujo.draw_texture_ex(
                arma_actual,
                Vector2::new(
                    arma_x,
                    arma_y,
                ),
                0.0,
                escala_arma,
                Color::WHITE,
            );

            if apuntando {
                dibujo.draw_circle(
                    ANCHO_VENTANA / 2,
                    ALTO_VENTANA / 2,
                    3.0,
                    Color::RED,
                );
            }
        }

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
                Vista::Vista3D =>
                    "3D",

                Vista::Mapa2D =>
                    "Mapa 2D",
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
            "WASD: mover | J/L: girar | I/K: camara | Click derecho: apuntar | M: mapa | R: reset",
            10,
            ALTO_VENTANA + 30,
            14,
            Color::LIGHTGRAY,
        );
    }
}