mod camera;
mod framebuffer;
mod map;
mod map_renderer;
mod player;
mod raycaster;

use camera::Camera;
use framebuffer::Framebuffer;
use map::Map;
use map_renderer::render_minimap;
use player::Player;

use raycaster::{
    render_3d,
    ALTO_VENTANA,
    ANCHO_VENTANA,
};

use raylib::prelude::*;

fn main() {
    let mapa = Map::new();

    mapa.guardar_txt(
        "mapa_resident.txt",
    );

    let mut player =
        Player::new(&mapa);

    let mut camera =
        Camera::new();

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
                ALTO_VENTANA,
            )
            .resizable()
            .title(
                "Survival Horror Raycasting",
            )
            .build();

    ventana.set_target_fps(60);

    ventana.disable_cursor();

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

    let mut textura_pared =
        Image::load_image(
            "assets/textures/wall.jpg",
        )
        .expect(
            "No se pudo cargar assets/textures/wall.jpgg",
        );

    let mut textura_suelo =
        Image::load_image(
            "assets/textures/floor.jpg",
        )
        .expect(
            "No se pudo cargar assets/textures/floor.jpg",
        );

    let mut textura_framebuffer =
        ventana
            .load_texture_from_image(
                &thread,
                framebuffer.image(),
            )
            .expect(
                "No se pudo crear la textura del framebuffer",
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
            KeyboardKey::KEY_R,
        ) {
            player.reset();
            camera.reset();
        }

        if ventana.is_key_pressed(
            KeyboardKey::KEY_F11,
        ) {
            if ventana.is_window_maximized() {
                ventana.restore_window();
            } else {
                ventana.maximize_window();
            }
        }

        if ventana.is_key_pressed(
            KeyboardKey::KEY_TAB,
        ) {
            ventana.enable_cursor();
        }

        if ventana.is_mouse_button_pressed(
            MouseButton::MOUSE_BUTTON_LEFT,
        ) {
            ventana.disable_cursor();
        }

        framebuffer.clear();

        render_3d(
            &mut framebuffer,
            &mapa,
            &player,
            &camera,
            &mut textura_pared,
            &mut textura_suelo,
        );

        render_minimap(
            &mut framebuffer,
            &mapa,
            &player,
            &camera,
        );

        textura_framebuffer
            .update_texture(
                framebuffer.pixels(),
            )
            .expect(
                "No se pudo actualizar la textura del framebuffer",
            );

        let pantalla_ancho =
            ventana.get_screen_width()
                as f32;

        let pantalla_alto =
            ventana.get_screen_height()
                as f32;

        let escala_x =
            pantalla_ancho
                / ANCHO_VENTANA as f32;

        let escala_y =
            pantalla_alto
                / ALTO_VENTANA as f32;

        let escala =
            escala_x.min(
                escala_y,
            );

        let ancho_render =
            ANCHO_VENTANA as f32
                * escala;

        let alto_render =
            ALTO_VENTANA as f32
                * escala;

        let offset_x =
            (
                pantalla_ancho
                    - ancho_render
            ) / 2.0;

        let offset_y =
            (
                pantalla_alto
                    - alto_render
            ) / 2.0;

        let arma_actual =
            if apuntando {
                &pistol2
            } else {
                &pistol1
            };

        let escala_base_arma =
            if apuntando {
                0.70
            } else {
                0.65
            };

        let escala_arma =
            escala_base_arma
                * escala;

        let arma_ancho =
            arma_actual.width()
                as f32
                * escala_arma;

        let arma_alto =
            arma_actual.height()
                as f32
                * escala_arma;

        let arma_x =
            offset_x
                + ancho_render / 2.0
                - arma_ancho / 2.0;

        let arma_y =
            offset_y
                + alto_render
                - arma_alto;

        let mut dibujo =
            ventana.begin_drawing(
                &thread,
            );

        dibujo.clear_background(
            Color::BLACK,
        );

        dibujo.draw_texture_pro(
            &textura_framebuffer,
            Rectangle::new(
                0.0,
                0.0,
                ANCHO_VENTANA as f32,
                ALTO_VENTANA as f32,
            ),
            Rectangle::new(
                offset_x,
                offset_y,
                ancho_render,
                alto_render,
            ),
            Vector2::new(
                0.0,
                0.0,
            ),
            0.0,
            Color::WHITE,
        );

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
            let mira_x =
                offset_x
                    + ancho_render / 2.0;

            let mira_y =
                offset_y
                    + alto_render / 2.0;

            dibujo.draw_circle(
                mira_x as i32,
                mira_y as i32,
                3.0
                    * escala.max(1.0),
                Color::RED,
            );
        }

        let texto_fps =
            format!(
                "FPS: {}",
                dibujo.get_fps(),
            );

        dibujo.draw_text(
            &texto_fps,
            dibujo.get_screen_width()
                - 100,
            10,
            20,
            Color::GREEN,
        );
    }
}