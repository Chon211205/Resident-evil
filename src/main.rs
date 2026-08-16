mod audio;
mod camera;
mod damage_effect;
mod framebuffer;
mod hud;
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

use audio::AudioManager;
use camera::Camera;
use damage_effect::DamageEffect;
use framebuffer::Framebuffer;
use hud::render_hud;

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

use sprite_renderer::{
    render_ammo_sprites,
    render_key_sprite,
};

use texture_data::TextureData;

use zombie::Zombie;
use zombie_renderer::render_zombies;

use raylib::audio::RaylibAudio;
use raylib::prelude::*;

fn main() {
    let mut mapa =
        Map::new();

    let posiciones_zombies =
        mapa.extraer_zombies();

    let mut zombies =
        posiciones_zombies
            .into_iter()
            .map(
                |(
                    x,
                    y,
                    tiene_llave,
                )| {
                    if tiene_llave {
                        Zombie::new_con_llave(
                            x,
                            y,
                        )
                    } else {
                        Zombie::new(
                            x,
                            y,
                        )
                    }
                },
            )
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

    let mut damage_effect =
        DamageEffect::new();

    let mut mensaje =
        String::new();

    let mut vida_jugador =
        100;

    let mut balas_cargador =
        8;

    let mut balas_reserva =
        24;

    let mut tiempo_disparo: f32 =
        0.0;

    let mut recargando =
        false;

    let mut tiempo_recarga: f32 =
        0.0;

    const DURACION_RECARGA: f32 =
        0.8;

    const CAMBIO_FRAME_RECARGA: f32 =
        0.4;

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

    let audio =
        RaylibAudio::init_audio_device()
            .expect(
                "No se pudo iniciar el audio",
            );

    let sonidos =
        AudioManager::new(
            &audio,
        );

    let pistol1 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistol1.png",
            )
            .expect(
                "No se pudo cargar assets/textures/pistol1.png",
            );

    let pistol2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistol2.png",
            )
            .expect(
                "No se pudo cargar assets/textures/pistol2.png",
            );

    let pistol3 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistol3.png",
            )
            .expect(
                "No se pudo cargar assets/textures/pistol3.png",
            );

    let pistol_r =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistolR.png",
            )
            .expect(
                "No se pudo cargar assets/textures/pistolR.png",
            );

    let pistol_r2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistolR2.png",
            )
            .expect(
                "No se pudo cargar assets/textures/pistolR2.png",
            );

    let key_texture =
        ventana
            .load_texture(
                &thread,
                "assets/textures/key.png",
            )
            .expect(
                "No se pudo cargar assets/textures/key.png",
            );

    let ammo_texture =
        ventana
            .load_texture(
                &thread,
                "assets/textures/ammo.png",
            )
            .expect(
                "No se pudo cargar assets/textures/ammo.png",
            );

    let zombie1 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombie1.png",
            )
            .expect(
                "No se pudo cargar assets/textures/zombie1.png",
            );

    let zombie2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombie2.png",
            )
            .expect(
                "No se pudo cargar assets/textures/zombie2.png",
            );

    let zombie3 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombie3.png",
            )
            .expect(
                "No se pudo cargar assets/textures/zombie3.png",
            );

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

        if tiempo_disparo > 0.0 {
            tiempo_disparo -=
                delta_time;

            if tiempo_disparo < 0.0 {
                tiempo_disparo =
                    0.0;
            }
        }

        if recargando {
            tiempo_recarga -=
                delta_time;

            if tiempo_recarga <= 0.0 {
                tiempo_recarga =
                    0.0;

                recargando =
                    false;

                let balas_faltantes =
                    8
                        - balas_cargador;

                let cantidad_recargar =
                    balas_faltantes.min(
                        balas_reserva,
                    );

                balas_cargador +=
                    cantidad_recargar;

                balas_reserva -=
                    cantidad_recargar;

                mensaje =
                    "Arma recargada"
                        .to_string();
            }
        }

        damage_effect.update(
            delta_time,
        );

        camera.update(
            &ventana,
            delta_time,
            2.0,
        );

        if vida_jugador > 0 {
            player.update(
                &ventana,
                &mapa,
                camera.angle,
                delta_time,
            );
        }

        if vida_jugador > 0 {
            for zombie in
                &mut zombies
            {
                let estaba_persiguiendo =
                    zombie.persiguiendo;

                let dano =
                    zombie.update(
                        &player,
                        &mapa,
                        delta_time,
                    );

                if !estaba_persiguiendo
                    && zombie.persiguiendo
                {
                    sonidos.zombie();
                }

                if dano > 0 {
                    vida_jugador -=
                        dano;

                    if vida_jugador < 0 {
                        vida_jugador =
                            0;
                    }

                    sonidos.dano();

                    mensaje =
                        format!(
                            "Un zombie te hizo {} de dano",
                            dano,
                        );

                    damage_effect.activar();
                }
            }
        }

        let resultado_recoger =
            recoger_objetos_cercanos(
                &mut mapa,
                &player,
                &mut inventory,
                &mut puzzle,
            );

        match resultado_recoger {
            InteractionResult::LlaveRecogida => {
                sonidos.llave();

                mensaje =
                    "Recogiste una llave"
                        .to_string();
            }

            InteractionResult::MunicionRecogida(
                cantidad,
            ) => {
                balas_reserva +=
                    cantidad;

                mensaje =
                    format!(
                        "Recogiste {} balas",
                        cantidad,
                    );
            }

            _ => {}
        }

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
                    sonidos.llave();

                    mensaje =
                        "Recogiste una llave"
                            .to_string();
                }

                InteractionResult::MunicionRecogida(
                    cantidad,
                ) => {
                    balas_reserva +=
                        cantidad;

                    mensaje =
                        format!(
                            "Recogiste {} balas",
                            cantidad,
                        );
                }

                InteractionResult::PuertaAbierta => {
                    sonidos.puerta();

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

        let apuntando =
            ventana.is_mouse_button_down(
                MouseButton::MOUSE_BUTTON_RIGHT,
            );

        if ventana.is_mouse_button_pressed(
            MouseButton::MOUSE_BUTTON_LEFT,
        ) {
            ventana.disable_cursor();

            if vida_jugador > 0
                && !recargando
            {
                if balas_cargador > 0 {
                    tiempo_disparo =
                        0.12;

                    balas_cargador -=
                        1;

                    sonidos.disparo();

                    let resultado =
                        disparar(
                            &mut zombies,
                            &player,
                            &camera,
                            &mut mapa,
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

                            ShotResult::KillConLlave => {
                                "Zombie eliminado. Dejo una llave"
                                    .to_string()
                            }
                        };
                } else {
                    sonidos.sin_municion();

                    mensaje =
                        if balas_reserva > 0 {
                            "Cargador vacio. Presiona R para recargar"
                                .to_string()
                        } else {
                            "Sin municion"
                                .to_string()
                        };
                }
            }
        }

        if ventana.is_key_pressed(
            KeyboardKey::KEY_R,
        ) {
            if vida_jugador > 0
                && !recargando
            {
                if balas_cargador == 8 {
                    mensaje =
                        "El cargador ya esta lleno"
                            .to_string();
                } else if balas_reserva <= 0 {
                    mensaje =
                        "No tienes balas de reserva"
                            .to_string();
                } else {
                    recargando =
                        true;

                    tiempo_recarga =
                        DURACION_RECARGA;

                    tiempo_disparo =
                        0.0;

                    sonidos.recarga();

                    mensaje =
                        "Recargando..."
                            .to_string();
                }
            }
        }

        if ventana.is_key_pressed(
            KeyboardKey::KEY_F5,
        ) {
            mapa =
                Map::new();

            let posiciones_zombies =
                mapa.extraer_zombies();

            zombies =
                posiciones_zombies
                    .into_iter()
                    .map(
                        |(
                            x,
                            y,
                            tiene_llave,
                        )| {
                            if tiene_llave {
                                Zombie::new_con_llave(
                                    x,
                                    y,
                                )
                            } else {
                                Zombie::new(
                                    x,
                                    y,
                                )
                            }
                        },
                    )
                    .collect::<Vec<Zombie>>();

            player =
                Player::new(
                    &mapa,
                );

            camera =
                Camera::new();

            inventory =
                Inventory::new();

            puzzle =
                Puzzle::new();

            damage_effect =
                DamageEffect::new();

            vida_jugador =
                100;

            balas_cargador =
                8;

            balas_reserva =
                24;

            tiempo_disparo =
                0.0;

            recargando =
                false;

            tiempo_recarga =
                0.0;

            mensaje =
                "Juego reiniciado"
                    .to_string();
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

        let pantalla_ancho =
            ventana.get_screen_width()
                as f32;

        let pantalla_alto =
            ventana.get_screen_height()
                as f32;

        let escala_x =
            pantalla_ancho
                / ANCHO_VENTANA
                    as f32;

        let escala_y =
            pantalla_alto
                / ALTO_VENTANA
                    as f32;

        let escala =
            escala_x.min(
                escala_y,
            );

        let ancho_render =
            ANCHO_VENTANA
                as f32
                * escala;

        let alto_render =
            ALTO_VENTANA
                as f32
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
            if recargando {
                if tiempo_recarga
                    > CAMBIO_FRAME_RECARGA
                {
                    &pistol_r2
                } else {
                    &pistol_r
                }
            } else if tiempo_disparo > 0.0 {
                &pistol3
            } else if apuntando {
                &pistol2
            } else {
                &pistol1
            };

        let escala_base_arma =
            if recargando {
                0.32
            } else if tiempo_disparo > 0.0 {
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
            if tiempo_disparo > 0.0
                && !recargando
            {
                8.0
                    * escala
            } else {
                0.0
            };

        let arma_x =
            offset_x
                + ancho_render
                    / 2.0
                - arma_ancho
                    / 2.0;

        let arma_y =
            offset_y
                + alto_render
                - arma_alto
                - retroceso;

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
                ANCHO_VENTANA
                    as f32,
                ALTO_VENTANA
                    as f32,
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

        render_ammo_sprites(
            &mut dibujo,
            &mapa,
            &player,
            &camera,
            &ammo_texture,
            offset_x,
            offset_y,
            escala,
        );

        render_zombies(
            &mut dibujo,
            &mapa,
            &player,
            &camera,
            &zombies,
            &zombie1,
            &zombie2,
            &zombie3,
            offset_x,
            offset_y,
            escala,
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

        if apuntando
            && !recargando
        {
            let mira_x =
                offset_x
                    + ancho_render
                        / 2.0;

            let mira_y =
                offset_y
                    + alto_render
                        / 2.0;

            dibujo.draw_circle(
                mira_x as i32,
                mira_y as i32,
                3.0
                    * escala.max(
                        1.0,
                    ),
                Color::RED,
            );
        }

        render_hud(
            &mut dibujo,
            vida_jugador,
            balas_cargador,
            balas_reserva,
            &inventory,
            &mensaje,
        );

        damage_effect.render(
            &mut dibujo,
        );
    }
}