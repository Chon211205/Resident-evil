mod camera;
mod framebuffer;
mod interaction;
mod inventory;
mod map;
mod map_renderer;
mod player;
mod puzzle;
mod raycaster;
mod sprite_renderer;
mod texture_data;
mod zombie;
mod zombie_renderer;

use camera::Camera;
use framebuffer::Framebuffer;

use interaction::{
    disparar,
    interactuar,
    recoger_objetos_cercanos,
    InteractionResult,
    ShotResult,
};

use inventory::Inventory;
use map::Map;
use map_renderer::render_minimap;
use player::Player;
use puzzle::Puzzle;

use raycaster::{
    render_3d,
    ALTO_VENTANA,
    ANCHO_VENTANA,
};

use sprite_renderer::render_key_sprite;
use texture_data::TextureData;

use zombie::Zombie;
use zombie_renderer::render_zombies;

use raylib::prelude::*;

fn main() {
    let mut mapa =
        Map::new();

    let posiciones_zombies =
        mapa.extraer_zombies();

    let mut zombies =
        posiciones_zombies
            .into_iter()
            .map(|(x, y)| {
                Zombie::new(
                    x,
                    y,
                )
            })
            .collect::<Vec<Zombie>>();

    println!(
        "Zombies creados: {}",
        zombies.len(),
    );

    mapa.guardar_txt(
        "mapa_resident.txt",
    );

    let mut player =
        Player::new(
            &mapa,
        );

    let mut camera =
        Camera::new();

    let mut inventory =
        Inventory::new();

    let mut puzzle =
        Puzzle::new();

    let mut mensaje =
        String::new();

    let mut vida_jugador =
        100;

    let mut tiempo_disparo: f32 =
        0.0;

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

    ventana.set_target_fps(
        60,
    );

    ventana.disable_cursor();

    // ARMAS

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

    let pistol3 =
        ventana
            .load_texture(
                &thread,
                "assets/pistol3.png",
            )
            .expect(
                "No se pudo cargar assets/pistol3.png",
            );

    // LLAVE

    let key_texture =
        ventana
            .load_texture(
                &thread,
                "assets/key.png",
            )
            .expect(
                "No se pudo cargar assets/key.png",
            );

    // ZOMBIES

    let zombie_idle =
        ventana
            .load_texture(
                &thread,
                "assets/zombie1.png",
            )
            .expect(
                "No se pudo cargar assets/zombie1.png",
            );

    let zombie_run1 =
        ventana
            .load_texture(
                &thread,
                "assets/zombie2.png",
            )
            .expect(
                "No se pudo cargar assets/zombie2.png",
            );

    let zombie_run2 =
        ventana
            .load_texture(
                &thread,
                "assets/zombie3.png",
            )
            .expect(
                "No se pudo cargar assets/zombie3.png",
            );

    // TEXTURAS DEL MAPA

    let mut wall_image =
        Image::load_image(
            "assets/textures/wall.png",
        )
        .expect(
            "No se pudo cargar assets/textures/wall.png",
        );

    let mut floor_image =
        Image::load_image(
            "assets/textures/floor.png",
        )
        .expect(
            "No se pudo cargar assets/textures/floor.png",
        );

    let mut door_image =
        Image::load_image(
            "assets/textures/door.png",
        )
        .expect(
            "No se pudo cargar assets/textures/door.png",
        );

    let textura_pared =
        TextureData::from_image(
            &mut wall_image,
        );

    let textura_suelo =
        TextureData::from_image(
            &mut floor_image,
        );

    let textura_puerta =
        TextureData::from_image(
            &mut door_image,
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


        // ANIMACIÓN DISPARO


        if tiempo_disparo > 0.0 {
            tiempo_disparo -=
                delta_time;

            if tiempo_disparo < 0.0 {
                tiempo_disparo =
                    0.0;
            }
        }

        // CÁMARA

        camera.update(
            &ventana,
            delta_time,
            2.0,
        );

        // JUGADOR

        if vida_jugador > 0 {
            player.update(
                &ventana,
                &mapa,
                camera.angle,
                delta_time,
            );
        }

        // ZOMBIES

        if vida_jugador > 0 {
            for zombie in
                &mut zombies
            {
                let dano =
                    zombie.update(
                        &player,
                        &mapa,
                        delta_time,
                    );

                vida_jugador -=
                    dano;

                if vida_jugador < 0 {
                    vida_jugador =
                        0;
                }

                if dano > 0 {
                    mensaje =
                        format!(
                            "Un zombie te hizo {} de dano",
                            dano,
                        );
                }
            }
        }

        // RECOGER LLAVE

        let resultado_recoger =
            recoger_objetos_cercanos(
                &mut mapa,
                &player,
                &mut inventory,
                &mut puzzle,
            );

        if let InteractionResult::LlaveRecogida =
            resultado_recoger
        {
            mensaje =
                "Recogiste una llave"
                    .to_string();
        }

        // PUERTAS

        if ventana.is_key_pressed(
            KeyboardKey::KEY_E,
        ) {
            let resultado =
                interactuar(
                    &mut mapa,
                    &player,
                    &camera,
                    &mut inventory,
                    &mut puzzle,
                );

            match resultado {
                InteractionResult::LlaveRecogida => {
                    mensaje =
                        "Recogiste una llave"
                            .to_string();
                }

                InteractionResult::PuertaAbierta => {
                    mensaje =
                        "Abriste la puerta"
                            .to_string();
                }

                InteractionResult::PuertaCerrada => {
                    mensaje =
                        "La puerta esta cerrada"
                            .to_string();
                }

                InteractionResult::None => {}
            }
        }

        // APUNTAR

        let apuntando =
            ventana.is_mouse_button_down(
                MouseButton::MOUSE_BUTTON_RIGHT,
            );

        // DISPARAR

        if ventana.is_mouse_button_pressed(
            MouseButton::MOUSE_BUTTON_LEFT,
        ) {
            ventana.disable_cursor();

            if vida_jugador > 0 {
                tiempo_disparo =
                    0.12;

                let resultado =
                    disparar(
                        &mut zombies,
                        &player,
                        &camera,
                        &mapa,
                    );

                mensaje =
                    match resultado {
                        ShotResult::Miss => {
                            "Disparo fallido"
                                .to_string()
                        }

                        ShotResult::Hit {
                            vida_restante,
                        } => {
                            format!(
                                "Le diste al zombie. Vida: {}",
                                vida_restante,
                            )
                        }

                        ShotResult::Kill => {
                            "Zombie eliminado"
                                .to_string()
                        }
                    };
            }
        }

        // RESET

        if ventana.is_key_pressed(
            KeyboardKey::KEY_R,
        ) {
            player.reset();
            camera.reset();

            vida_jugador =
                100;

            mensaje =
                "Jugador reiniciado"
                    .to_string();
        }

        // VENTANA

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

        // RENDER 3D

        framebuffer.clear();

        render_3d(
            &mut framebuffer,
            &mapa,
            &player,
            &camera,
            &textura_pared,
            &textura_puerta,
            &textura_suelo,
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

        // ESCALADO VENTANA

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

        // SPRITE DEL ARMA

        let arma_actual =
            if tiempo_disparo > 0.0 {
                &pistol3
            } else if apuntando {
                &pistol2
            } else {
                &pistol1
            };

        let escala_base_arma =
            if tiempo_disparo > 0.0 {
                0.40
            } else if apuntando {
                0.42
            } else {
                0.38
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

        let retroceso =
            if tiempo_disparo > 0.0 {
                8.0 * escala
            } else {
                0.0
            };

        let arma_x =
            offset_x
                + ancho_render / 2.0
                - arma_ancho / 2.0;

        let arma_y =
            offset_y
                + alto_render
                - arma_alto
                - retroceso;

        // DRAWING

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

        // Llave
        render_key_sprite(
            &mut dibujo,
            &mapa,
            &player,
            &camera,
            &key_texture,
            offset_x,
            offset_y,
            escala,
        );

        // Zombies
        render_zombies(
            &mut dibujo,
            &mapa,
            &player,
            &camera,
            &zombies,
            &zombie_idle,
            &zombie_run1,
            &zombie_run2,
            offset_x,
            offset_y,
            escala,
        );

        // Arma
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

        // Mira
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

        // HUD

        let texto_vida =
            format!(
                "Vida: {}",
                vida_jugador,
            );

        dibujo.draw_text(
            &texto_vida,
            10,
            65,
            20,
            if vida_jugador > 30 {
                Color::GREEN
            } else {
                Color::RED
            },
        );

        if inventory.tiene_llave() {
            dibujo.draw_text(
                "Llave: SI",
                10,
                40,
                18,
                Color::YELLOW,
            );
        } else {
            dibujo.draw_text(
                "Llave: NO",
                10,
                40,
                18,
                Color::GRAY,
            );
        }

        if !mensaje.is_empty() {
            dibujo.draw_rectangle(
                20,
                dibujo.get_screen_height()
                    - 70,
                400,
                40,
                Color::new(
                    0,
                    0,
                    0,
                    180,
                ),
            );

            dibujo.draw_text(
                &mensaje,
                30,
                dibujo.get_screen_height()
                    - 60,
                20,
                Color::WHITE,
            );
        }

        // MUERTE

        if vida_jugador <= 0 {
            let texto =
                "HAS MUERTO";

            let ancho_texto =
                dibujo.measure_text(
                    texto,
                    50,
                );

            dibujo.draw_rectangle(
                0,
                0,
                dibujo.get_screen_width(),
                dibujo.get_screen_height(),
                Color::new(
                    0,
                    0,
                    0,
                    170,
                ),
            );

            dibujo.draw_text(
                texto,
                dibujo.get_screen_width()
                    / 2
                    - ancho_texto / 2,
                dibujo.get_screen_height()
                    / 2
                    - 25,
                50,
                Color::RED,
            );

            dibujo.draw_text(
                "Presiona R para reiniciar",
                dibujo.get_screen_width()
                    / 2
                    - 120,
                dibujo.get_screen_height()
                    / 2
                    + 40,
                20,
                Color::WHITE,
            );
        }

        // FPS

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