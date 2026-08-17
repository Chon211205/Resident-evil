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
mod weapon;
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

use map::{
    Map,
    TipoSpawnZombie,
    TAMANO_CELDA,
};

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
    render_heal_sprites,
    render_key_sprite,
};

use texture_data::TextureData;

use weapon::{
    atacar_con_hacha,
    puede_bloquear_ataque,
    ArmaActual,
};

use zombie::{
    TipoZombie,
    Zombie,
};

use zombie_renderer::render_zombies;

use rand::Rng;

use raylib::audio::RaylibAudio;
use raylib::prelude::*;

const MAX_ZOMBIES_VIVOS: usize =
    18;

const DISTANCIA_MIN_SPAWN: f32 =
    120.0;

const DISTANCIA_MAX_SPAWN: f32 =
    380.0;

const DISTANCIA_ENTRE_ZOMBIES: f32 =
    35.0;

fn crear_zombies(
    mapa: &mut Map,
) -> Vec<Zombie> {
    mapa.extraer_zombies()
        .into_iter()
        .map(
            |(
                x,
                y,
                tipo,
            )| {
                match tipo {
                    TipoSpawnZombie::Normal => {
                        Zombie::new(
                            x,
                            y,
                        )
                    }

                    TipoSpawnZombie::ConLlave => {
                        Zombie::new_con_llave(
                            x,
                            y,
                        )
                    }

                    TipoSpawnZombie::Medio => {
                        Zombie::new_medio(
                            x,
                            y,
                        )
                    }

                    TipoSpawnZombie::Fuerte => {
                        Zombie::new_fuerte(
                            x,
                            y,
                        )
                    }
                }
            },
        )
        .collect()
}

fn calcular_distancia(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
) -> f32 {
    let dx =
        x2 - x1;

    let dy =
        y2 - y1;

    (
        dx * dx
            + dy * dy
    )
        .sqrt()
}

fn buscar_spawn_horda(
    mapa: &Map,
    player: &Player,
    zombies: &[Zombie],
) -> Option<(
    f32,
    f32,
)> {
    if mapa.ancho() <= 2
        || mapa.alto() <= 2
    {
        return None;
    }

    let mut rng =
        rand::thread_rng();

    for _ in 0..500 {
        let columna =
            rng.gen_range(
                1..mapa.ancho() - 1,
            );

        let fila =
            rng.gen_range(
                1..mapa.alto() - 1,
            );

        let celda =
            mapa.celda(
                fila as i32,
                columna as i32,
            );

        if !matches!(
            celda,
            ' ' | 'C'
        ) {
            continue;
        }

        let x =
            columna as f32
                * TAMANO_CELDA
                + TAMANO_CELDA
                    / 2.0;

        let y =
            fila as f32
                * TAMANO_CELDA
                + TAMANO_CELDA
                    / 2.0;

        let distancia_jugador =
            calcular_distancia(
                player.x,
                player.y,
                x,
                y,
            );

        if distancia_jugador
            < DISTANCIA_MIN_SPAWN
        {
            continue;
        }

        if distancia_jugador
            > DISTANCIA_MAX_SPAWN
        {
            continue;
        }

        if mapa.es_pared(
            x,
            y,
        ) {
            continue;
        }

        let ocupado =
            zombies
                .iter()
                .filter(
                    |zombie| {
                        zombie.vivo
                    },
                )
                .any(
                    |zombie| {
                        calcular_distancia(
                            zombie.x,
                            zombie.y,
                            x,
                            y,
                        )
                            < DISTANCIA_ENTRE_ZOMBIES
                    },
                );

        if ocupado {
            continue;
        }

        return Some(
            (
                x,
                y,
            ),
        );
    }

    None
}

fn cantidad_horda(
    numero_horda: i32,
) -> usize {
    match numero_horda {
        1 => 7,
        2 => 9,
        3 => 11,
        4 => 13,

        _ => 14,
    }
}

fn crear_zombie_horda(
    numero_horda: i32,
    indice: usize,
    x: f32,
    y: f32,
) -> Zombie {
    match numero_horda {
        1 => {
            Zombie::new(
                x,
                y,
            )
        }

        2 => {
            if indice % 4 == 0 {
                Zombie::new_medio(
                    x,
                    y,
                )
            } else {
                Zombie::new(
                    x,
                    y,
                )
            }
        }

        3 => {
            match indice % 6 {
                0 => {
                    Zombie::new_fuerte(
                        x,
                        y,
                    )
                }

                1 | 2 => {
                    Zombie::new_medio(
                        x,
                        y,
                    )
                }

                _ => {
                    Zombie::new(
                        x,
                        y,
                    )
                }
            }
        }

        _ => {
            match indice % 5 {
                0 => {
                    Zombie::new_fuerte(
                        x,
                        y,
                    )
                }

                1 | 2 => {
                    Zombie::new_medio(
                        x,
                        y,
                    )
                }

                _ => {
                    Zombie::new(
                        x,
                        y,
                    )
                }
            }
        }
    }
}

fn generar_horda(
    zombies: &mut Vec<Zombie>,
    mapa: &Map,
    player: &Player,
    numero_horda: i32,
) -> usize {
    let vivos =
        zombies
            .iter()
            .filter(
                |zombie| {
                    zombie.vivo
                },
            )
            .count();

    if vivos
        >= MAX_ZOMBIES_VIVOS
    {
        return 0;
    }

    let cantidad_deseada =
        cantidad_horda(
            numero_horda,
        );

    let disponibles =
        MAX_ZOMBIES_VIVOS
            .saturating_sub(
                vivos,
            );

    let cantidad =
        cantidad_deseada
            .min(
                disponibles,
            );

    let mut generados =
        0;

    for indice in 0..cantidad {
        let spawn =
            buscar_spawn_horda(
                mapa,
                player,
                zombies,
            );

        let Some(
            (
                x,
                y,
            ),
        ) = spawn
        else {
            continue;
        };

        let mut zombie =
            crear_zombie_horda(
                numero_horda,
                indice,
                x,
                y,
            );

        zombie.persiguiendo =
            true;

        zombies.push(
            zombie,
        );

        generados +=
            1;
    }

    generados
}

fn detener_sonidos_zombies(
    sonidos: &AudioManager,
) {
    sonidos.detener_zombie();

    sonidos
        .detener_zombie_medio();

    sonidos
        .detener_zombie_fuerte();
}

fn main() {
    let mut nivel_actual =
        1;

    let mut juego_completado =
        false;

    let mut enemigos_matados =
        0_i32;

    let mut numero_horda =
        1_i32;

    let mut siguiente_horda =
        4_i32;

    let mut mapa =
        Map::new(
            nivel_actual,
        );

    let mut zombies =
        crear_zombies(
            &mut mapa,
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

    let mut arma_equipada =
        ArmaActual::Pistola;

    let mut mensaje =
        format!(
            "NIVEL {}",
            nivel_actual,
        );

    let mut vida_jugador =
        100;

    let mut balas_cargador =
        8;

    let mut balas_reserva =
        24;

    let mut tiempo_disparo =
        0.0_f32;

    let mut tiempo_hachazo =
        0.0_f32;

    let mut recargando =
        false;

    let mut tiempo_recarga =
        0.0_f32;

    const DURACION_RECARGA: f32 =
        0.8;

    const CAMBIO_FRAME_RECARGA: f32 =
        0.4;

    const DURACION_HACHAZO: f32 =
        0.22;

    let mut framebuffer =
        Framebuffer::new(
            ANCHO_VENTANA,
            ALTO_VENTANA,
        );

    framebuffer
        .set_background_color(
            Color::BLACK,
        );

    let (
        mut ventana,
        thread,
    ) =
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

    ventana.toggle_fullscreen();

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
                "No se pudo cargar pistol1.png",
            );

    let pistol2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistol2.png",
            )
            .expect(
                "No se pudo cargar pistol2.png",
            );

    let pistol3 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistol3.png",
            )
            .expect(
                "No se pudo cargar pistol3.png",
            );

    let pistol_r =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistolR.png",
            )
            .expect(
                "No se pudo cargar pistolR.png",
            );

    let pistol_r2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistolR2.png",
            )
            .expect(
                "No se pudo cargar pistolR2.png",
            );

    let axe1 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/axe1.png",
            )
            .expect(
                "No se pudo cargar axe1.png",
            );

    let axe2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/axe2.png",
            )
            .expect(
                "No se pudo cargar axe2.png",
            );

    let axe3 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/axe3.png",
            )
            .expect(
                "No se pudo cargar axe3.png",
            );

    let key_texture =
        ventana
            .load_texture(
                &thread,
                "assets/textures/key.png",
            )
            .expect(
                "No se pudo cargar key.png",
            );

    let ammo_texture =
        ventana
            .load_texture(
                &thread,
                "assets/textures/ammo.png",
            )
            .expect(
                "No se pudo cargar ammo.png",
            );

    let heal_texture =
        ventana
            .load_texture(
                &thread,
                "assets/textures/heal.png",
            )
            .expect(
                "No se pudo cargar heal.png",
            );

    let zombie1 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombie1.png",
            )
            .expect(
                "No se pudo cargar zombie1.png",
            );

    let zombie2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombie2.png",
            )
            .expect(
                "No se pudo cargar zombie2.png",
            );

    let zombie3 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombie3.png",
            )
            .expect(
                "No se pudo cargar zombie3.png",
            );

    let zombie_v21 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombieV21.png",
            )
            .expect(
                "No se pudo cargar zombieV21.png",
            );

    let zombie_v22 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombieV22.png",
            )
            .expect(
                "No se pudo cargar zombieV22.png",
            );

    let zombie_v23 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombiev23.png",
            )
            .expect(
                "No se pudo cargar zombiev23.png",
            );

    let zombie_v31 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombieV31.png",
            )
            .expect(
                "No se pudo cargar zombieV31.png",
            );

    let zombie_v32 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombieV32.png",
            )
            .expect(
                "No se pudo cargar zombieV32.png",
            );

    let zombie_v33 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombieV33.png",
            )
            .expect(
                "No se pudo cargar zombieV33.png",
            );

    let mut wall_image =
        Image::load_image(
            "assets/textures/wall.png",
        )
        .expect(
            "No se pudo cargar wall.png",
        );

    let mut window_image =
        Image::load_image(
            "assets/textures/window.png",
        )
        .expect(
            "No se pudo cargar window.png",
        );

    let mut door_image =
        Image::load_image(
            "assets/textures/door.png",
        )
        .expect(
            "No se pudo cargar door.png",
        );

    let mut floor_image =
        Image::load_image(
            "assets/textures/floor.png",
        )
        .expect(
            "No se pudo cargar floor.png",
        );

    let mut floor2_image =
        Image::load_image(
            "assets/textures/floor2.png",
        )
        .expect(
            "No se pudo cargar floor2.png",
        );

    let textura_pared =
        TextureData::from_image(
            &mut wall_image,
        );

    let textura_ventana =
        TextureData::from_image(
            &mut window_image,
        );

    let textura_puerta =
        TextureData::from_image(
            &mut door_image,
        );

    let textura_suelo =
        TextureData::from_image(
            &mut floor_image,
        );

    let textura_suelo2 =
        TextureData::from_image(
            &mut floor2_image,
        );

    let mut textura_framebuffer =
        ventana
            .load_texture_from_image(
                &thread,
                framebuffer.image(),
            )
            .expect(
                "No se pudo crear framebuffer",
            );

    while !ventana.window_should_close() {
        let delta_time =
            ventana.get_frame_time();

        if tiempo_disparo > 0.0 {
            tiempo_disparo -=
                delta_time;

            tiempo_disparo =
                tiempo_disparo
                    .max(
                        0.0,
                    );
        }

        if tiempo_hachazo > 0.0 {
            tiempo_hachazo -=
                delta_time;

            tiempo_hachazo =
                tiempo_hachazo
                    .max(
                        0.0,
                    );
        }

        if recargando
            && vida_jugador > 0
            && !juego_completado
        {
            tiempo_recarga -=
                delta_time;

            if tiempo_recarga <= 0.0 {
                tiempo_recarga =
                    0.0;

                recargando =
                    false;

                let faltantes =
                    8
                        - balas_cargador;

                let cantidad =
                    faltantes
                        .min(
                            balas_reserva,
                        );

                balas_cargador +=
                    cantidad;

                balas_reserva -=
                    cantidad;

                mensaje =
                    "Arma recargada"
                        .to_string();
            }
        }

        damage_effect.update(
            delta_time,
        );

        if vida_jugador > 0
            && !juego_completado
        {
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
        }

        let bloqueando =
            vida_jugador > 0
                && !juego_completado
                && arma_equipada
                    == ArmaActual::Hacha
                && tiempo_hachazo
                    <= 0.0
                && ventana
                    .is_mouse_button_down(
                        MouseButton::
                            MOUSE_BUTTON_RIGHT,
                    );

        if vida_jugador > 0
            && !juego_completado
        {
            for zombie in &mut zombies {
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
                    match zombie.tipo {
                        TipoZombie::Normal => {
                            sonidos.zombie();
                        }

                        TipoZombie::Medio => {
                            sonidos
                                .zombie_medio();
                        }

                        TipoZombie::Fuerte => {
                            sonidos
                                .zombie_fuerte();
                        }
                    }
                }

                if dano > 0 {
                    let bloqueo_valido =
                        bloqueando
                            && puede_bloquear_ataque(
                                zombie,
                                &player,
                                &camera,
                            );

                    let dano_final =
                        if bloqueo_valido {
                            3
                        } else {
                            dano
                        };

                    vida_jugador -=
                        dano_final;

                    vida_jugador =
                        vida_jugador.max(
                            0,
                        );

                    if bloqueo_valido {
                        sonidos
                            .bloqueo_hacha();

                        mensaje =
                            format!(
                                "Bloqueaste. Dano: {}",
                                dano_final,
                            );
                    } else {
                        sonidos.dano();

                        if vida_jugador <= 0 {
                            mensaje =
                                "Has muerto"
                                    .to_string();
                        } else {
                            mensaje =
                                format!(
                                    "Zombie te hizo {} de dano",
                                    dano_final,
                                );
                        }
                    }

                    damage_effect
                        .activar();
                }
            }
        }

        let hay_normal =
            zombies
                .iter()
                .any(
                    |zombie| {
                        zombie.vivo
                            && zombie.persiguiendo
                            && zombie.tipo
                                == TipoZombie::Normal
                    },
                );

        let hay_medio =
            zombies
                .iter()
                .any(
                    |zombie| {
                        zombie.vivo
                            && zombie.persiguiendo
                            && zombie.tipo
                                == TipoZombie::Medio
                    },
                );

        let hay_fuerte =
            zombies
                .iter()
                .any(
                    |zombie| {
                        zombie.vivo
                            && zombie.persiguiendo
                            && zombie.tipo
                                == TipoZombie::Fuerte
                    },
                );

        if !hay_normal
            || vida_jugador <= 0
            || juego_completado
        {
            sonidos
                .detener_zombie();
        }

        if !hay_medio
            || vida_jugador <= 0
            || juego_completado
        {
            sonidos
                .detener_zombie_medio();
        }

        if !hay_fuerte
            || vida_jugador <= 0
            || juego_completado
        {
            sonidos
                .detener_zombie_fuerte();
        }

        if vida_jugador > 0
            && !juego_completado
        {
            let resultado =
                recoger_objetos_cercanos(
                    &mut mapa,
                    &player,
                    &mut inventory,
                    &mut puzzle,
                    vida_jugador,
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
                    sonidos
                        .recoger_municion();

                    balas_reserva +=
                        cantidad;

                    mensaje =
                        format!(
                            "Recogiste {} balas",
                            cantidad,
                        );
                }

                InteractionResult::CuracionRecogida(
                    cantidad,
                ) => {
                    sonidos.curacion();

                    vida_jugador =
                        (
                            vida_jugador
                                + cantidad
                        )
                            .min(
                                100,
                            );

                    mensaje =
                        format!(
                            "Te curaste {} de vida",
                            cantidad,
                        );
                }

                InteractionResult::SalidaNivel => {
                    detener_sonidos_zombies(
                        &sonidos,
                    );

                    if nivel_actual == 1 {
                        nivel_actual =
                            2;

                        mapa =
                            Map::new(
                                nivel_actual,
                            );

                        zombies =
                            crear_zombies(
                                &mut mapa,
                            );

                        player =
                            Player::new(
                                &mapa,
                            );

                        camera =
                            Camera::new();

                        puzzle =
                            Puzzle::new();

                        damage_effect =
                            DamageEffect::new();

                        tiempo_disparo =
                            0.0;

                        tiempo_hachazo =
                            0.0;

                        recargando =
                            false;

                        tiempo_recarga =
                            0.0;

                        mensaje =
                            format!(
                                "NIVEL 2 - HORDA {} - BAJAS {}",
                                numero_horda,
                                enemigos_matados,
                            );
                    } else {
                        juego_completado =
                            true;

                        recargando =
                            false;

                        tiempo_recarga =
                            0.0;

                        tiempo_disparo =
                            0.0;

                        tiempo_hachazo =
                            0.0;

                        detener_sonidos_zombies(
                            &sonidos,
                        );

                        mensaje =
                            format!(
                                "MISION COMPLETA - {} BAJAS",
                                enemigos_matados,
                            );
                    }
                }

                _ => {}
            }
        }

        if vida_jugador > 0
            && !juego_completado
        {
            if ventana.is_key_pressed(
                KeyboardKey::KEY_ONE,
            ) {
                arma_equipada =
                    ArmaActual::Pistola;

                tiempo_hachazo =
                    0.0;

                mensaje =
                    "Pistola equipada"
                        .to_string();
            }

            if ventana.is_key_pressed(
                KeyboardKey::KEY_TWO,
            ) {
                arma_equipada =
                    ArmaActual::Hacha;

                recargando =
                    false;

                tiempo_recarga =
                    0.0;

                tiempo_disparo =
                    0.0;

                mensaje =
                    "Hacha equipada"
                        .to_string();
            }

            if ventana.is_key_pressed(
                KeyboardKey::KEY_E,
            ) {
                match interactuar(
                    &mut mapa,
                    &player,
                    &camera,
                    &mut inventory,
                    &mut puzzle,
                ) {
                    InteractionResult::PuertaAbierta => {
                        sonidos.puerta();

                        mensaje =
                            "Abriste la puerta"
                                .to_string();
                    }

                    InteractionResult::PuertaCerrada => {
                        mensaje =
                            "Necesitas una llave"
                                .to_string();
                    }

                    _ => {}
                }
            }
        }

        let apuntando =
            vida_jugador > 0
                && !juego_completado
                && arma_equipada
                    == ArmaActual::Pistola
                && !recargando
                && ventana
                    .is_mouse_button_down(
                        MouseButton::
                            MOUSE_BUTTON_RIGHT,
                    );

        if vida_jugador > 0
            && !juego_completado
            && ventana
                .is_mouse_button_pressed(
                    MouseButton::
                        MOUSE_BUTTON_LEFT,
                )
        {
            ventana.disable_cursor();

            match arma_equipada {
                ArmaActual::Pistola => {
                    if !recargando {
                        if balas_cargador > 0 {
                            balas_cargador -=
                                1;

                            tiempo_disparo =
                                0.12;

                            sonidos.disparo();

                            let vivos_antes =
                                zombies
                                    .iter()
                                    .filter(
                                        |zombie| {
                                            zombie.vivo
                                        },
                                    )
                                    .count();

                            let resultado =
                                disparar(
                                    &mut zombies,
                                    &player,
                                    &camera,
                                    &mut mapa,
                                );

                            let vivos_despues =
                                zombies
                                    .iter()
                                    .filter(
                                        |zombie| {
                                            zombie.vivo
                                        },
                                    )
                                    .count();

                            if vivos_despues
                                < vivos_antes
                            {
                                let bajas =
                                    vivos_antes
                                        - vivos_despues;

                                enemigos_matados +=
                                    bajas
                                        as i32;

                                sonidos
                                    .zombie_muere();
                            }

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
                                            "Impacto. Vida zombie: {}",
                                            vida_restante,
                                        )
                                    }

                                    ShotResult::Kill => {
                                        format!(
                                            "Zombie eliminado - BAJAS {}",
                                            enemigos_matados,
                                        )
                                    }

                                    ShotResult::KillConLlave => {
                                        format!(
                                            "Zombie eliminado - BAJAS {} - DEJO LLAVE",
                                            enemigos_matados,
                                        )
                                    }
                                };
                        } else {
                            sonidos
                                .sin_municion();

                            if balas_reserva > 0 {
                                mensaje =
                                    "Cargador vacio. Presiona R"
                                        .to_string();
                            } else {
                                mensaje =
                                    "Sin municion"
                                        .to_string();
                            }
                        }
                    }
                }

                ArmaActual::Hacha => {
                    if tiempo_hachazo <= 0.0
                        && !bloqueando
                    {
                        tiempo_hachazo =
                            DURACION_HACHAZO;

                        sonidos.hachazo();

                        let vivos_antes =
                            zombies
                                .iter()
                                .filter(
                                    |zombie| {
                                        zombie.vivo
                                    },
                                )
                                .count();

                        let resultado =
                            atacar_con_hacha(
                                &mut zombies,
                                &player,
                                &camera,
                                &mut mapa,
                            );

                        let vivos_despues =
                            zombies
                                .iter()
                                .filter(
                                    |zombie| {
                                        zombie.vivo
                                    },
                                )
                                .count();

                        if vivos_despues
                            < vivos_antes
                        {
                            let bajas =
                                vivos_antes
                                    - vivos_despues;

                            enemigos_matados +=
                                bajas
                                    as i32;

                            sonidos
                                .zombie_muere();
                        }

                        mensaje =
                            match resultado {
                                ShotResult::Miss => {
                                    "Hachazo fallido"
                                        .to_string()
                                }

                                ShotResult::Hit {
                                    vida_restante,
                                } => {
                                    format!(
                                        "Golpe. Vida zombie: {}",
                                        vida_restante,
                                    )
                                }

                                ShotResult::Kill => {
                                    format!(
                                        "Zombie eliminado - BAJAS {}",
                                        enemigos_matados,
                                    )
                                }

                                ShotResult::KillConLlave => {
                                    format!(
                                        "Zombie eliminado - BAJAS {} - DEJO LLAVE",
                                        enemigos_matados,
                                    )
                                }
                            };
                    }
                }
            }
        }

        if vida_jugador > 0
            && !juego_completado
        {
            while enemigos_matados
                >= siguiente_horda
            {
                let horda_actual =
                    numero_horda;

                let generados =
                    generar_horda(
                        &mut zombies,
                        &mapa,
                        &player,
                        horda_actual,
                    );

                if generados > 0 {
                    mensaje =
                        format!(
                            "HORDA {} - {} ENEMIGOS",
                            horda_actual,
                            generados,
                        );

                    sonidos.zombie();

                    if horda_actual >= 2 {
                        sonidos
                            .zombie_medio();
                    }

                    if horda_actual >= 3 {
                        sonidos
                            .zombie_fuerte();
                    }
                }

                numero_horda +=
                    1;

                siguiente_horda +=
                    6;
            }
        }

        if vida_jugador > 0
            && !juego_completado
            && ventana.is_key_pressed(
                KeyboardKey::KEY_R,
            )
            && arma_equipada
                == ArmaActual::Pistola
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

        if ventana.is_key_pressed(
            KeyboardKey::KEY_F5,
        ) {
            detener_sonidos_zombies(
                &sonidos,
            );

            nivel_actual =
                1;

            juego_completado =
                false;

            enemigos_matados =
                0;

            numero_horda =
                1;

            siguiente_horda =
                4;

            mapa =
                Map::new(
                    nivel_actual,
                );

            zombies =
                crear_zombies(
                    &mut mapa,
                );

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

            arma_equipada =
                ArmaActual::Pistola;

            vida_jugador =
                100;

            balas_cargador =
                8;

            balas_reserva =
                24;

            tiempo_disparo =
                0.0;

            tiempo_hachazo =
                0.0;

            recargando =
                false;

            tiempo_recarga =
                0.0;

            mensaje =
                "NIVEL 1"
                    .to_string();

            ventana.disable_cursor();
        }

        if ventana.is_key_pressed(
            KeyboardKey::KEY_F11,
        ) {
            ventana.toggle_fullscreen();
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
            &textura_ventana,
            &textura_puerta,
            &textura_suelo,
            &textura_suelo2,
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
                "No se pudo actualizar framebuffer",
            );

        let pantalla_ancho =
            ventana
                .get_screen_width()
                as f32;

        let pantalla_alto =
            ventana
                .get_screen_height()
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
            )
                / 2.0;

        let offset_y =
            (
                pantalla_alto
                    - alto_render
            )
                / 2.0;

        let textura_arma =
            match arma_equipada {
                ArmaActual::Pistola => {
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
                    }
                }

                ArmaActual::Hacha => {
                    if bloqueando {
                        &axe3
                    } else if tiempo_hachazo > 0.0 {
                        &axe2
                    } else {
                        &axe1
                    }
                }
            };

        let escala_base_arma =
            match arma_equipada {
                ArmaActual::Pistola => {
                    if recargando {
                        0.28
                    } else if tiempo_disparo > 0.0 {
                        0.34
                    } else if apuntando {
                        0.36
                    } else {
                        0.32
                    }
                }

                ArmaActual::Hacha => {
                    if bloqueando {
                        0.42
                    } else if tiempo_hachazo > 0.0 {
                        0.37
                    } else {
                        0.34
                    }
                }
            };

        let escala_arma =
            escala_base_arma
                * escala;

        let arma_ancho =
            textura_arma
                .width()
                as f32
                * escala_arma;

        let arma_alto =
            textura_arma
                .height()
                as f32
                * escala_arma;

        let retroceso =
            if arma_equipada
                == ArmaActual::Pistola
                && tiempo_disparo > 0.0
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

        let scissor_x =
            offset_x
                .round()
                .max(
                    0.0,
                )
                as i32;

        let scissor_y =
            offset_y
                .round()
                .max(
                    0.0,
                )
                as i32;

        let scissor_ancho =
            ancho_render
                .round()
                .max(
                    1.0,
                )
                as i32;

        let scissor_alto =
            alto_render
                .round()
                .max(
                    1.0,
                )
                as i32;

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

        unsafe {
            raylib::ffi::BeginScissorMode(
                scissor_x,
                scissor_y,
                scissor_ancho,
                scissor_alto,
            );
        }

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

        render_heal_sprites(
            &mut dibujo,
            &mapa,
            &player,
            &camera,
            &heal_texture,
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

            &zombie_v21,
            &zombie_v22,
            &zombie_v23,

            &zombie_v31,
            &zombie_v32,
            &zombie_v33,

            offset_x,
            offset_y,
            escala,
        );

        if vida_jugador > 0
            && !juego_completado
        {
            dibujo.draw_texture_ex(
                textura_arma,

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
                        + ancho_render
                            / 2.0;

                let mira_y =
                    offset_y
                        + alto_render
                            / 2.0;

                dibujo.draw_circle(
                    mira_x
                        as i32,

                    mira_y
                        as i32,

                    3.0
                        * escala.max(
                            1.0,
                        ),

                    Color::RED,
                );
            }
        }

        unsafe {
            raylib::ffi::EndScissorMode();
        }

        render_hud(
            &mut dibujo,
            vida_jugador,
            balas_cargador,
            balas_reserva,
            &inventory,
            &mensaje,
            offset_x,
            offset_y,
            ancho_render,
            alto_render,
            escala,
        );

        let vivos_actuales =
            zombies
                .iter()
                .filter(
                    |zombie| {
                        zombie.vivo
                    },
                )
                .count();

        let escala_ui =
            escala.clamp(
                1.0,
                1.25,
            );

        let stats_size =
            (
                16.0
                    * escala_ui
            )
                as i32;

        let stats =
            format!(
                "BAJAS: {}  |  HORDA: {}  |  ENEMIGOS: {}",
                enemigos_matados,
                numero_horda,
                vivos_actuales,
            );

        let stats_width =
            dibujo.measure_text(
                &stats,
                stats_size,
            );

        dibujo.draw_text(
            &stats,

            (
                offset_x
                    + ancho_render
                    / 2.0
                    - stats_width
                        as f32
                        / 2.0
            ) as i32,

            (
                offset_y
                    + 12.0
                        * escala_ui
            ) as i32,

            stats_size,

            Color::LIGHTGRAY,
        );

        damage_effect.render(
            &mut dibujo,
        );

        if vida_jugador <= 0 {
            let screen_width =
                dibujo
                    .get_screen_width();

            let screen_height =
                dibujo
                    .get_screen_height();

            dibujo.draw_rectangle(
                0,
                0,
                screen_width,
                screen_height,

                Color::new(
                    0,
                    0,
                    0,
                    215,
                ),
            );

            let titulo =
                "GAME OVER";

            let stats =
                format!(
                    "BAJAS: {}   HORDA ALCANZADA: {}",
                    enemigos_matados,
                    numero_horda,
                );

            let reiniciar =
                "F5 - REINICIAR";

            let tamano_titulo =
                64;

            let tamano_stats =
                24;

            let tamano_opcion =
                22;

            let centro_x =
                screen_width
                    / 2;

            let centro_y =
                screen_height
                    / 2;

            let ancho_titulo =
                dibujo.measure_text(
                    titulo,
                    tamano_titulo,
                );

            let ancho_stats =
                dibujo.measure_text(
                    &stats,
                    tamano_stats,
                );

            let ancho_reiniciar =
                dibujo.measure_text(
                    reiniciar,
                    tamano_opcion,
                );

            dibujo.draw_text(
                titulo,

                centro_x
                    - ancho_titulo
                        / 2,

                centro_y
                    - 110,

                tamano_titulo,

                Color::RED,
            );

            dibujo.draw_text(
                &stats,

                centro_x
                    - ancho_stats
                        / 2,

                centro_y,

                tamano_stats,

                Color::LIGHTGRAY,
            );

            dibujo.draw_text(
                reiniciar,

                centro_x
                    - ancho_reiniciar
                        / 2,

                centro_y
                    + 65,

                tamano_opcion,

                Color::WHITE,
            );
        }

        if juego_completado {
            let screen_width =
                dibujo
                    .get_screen_width();

            let screen_height =
                dibujo
                    .get_screen_height();

            dibujo.draw_rectangle(
                0,
                0,
                screen_width,
                screen_height,

                Color::new(
                    0,
                    0,
                    0,
                    225,
                ),
            );

            let titulo =
                "MISSION COMPLETE";

            let subtitulo =
                "ESCAPASTE DEL LABORATORIO";

            let stats =
                format!(
                    "BAJAS: {}   HORDAS: {}",
                    enemigos_matados,
                    numero_horda,
                );

            let reiniciar =
                "F5 - JUGAR DE NUEVO";

            let tamano_titulo =
                56;

            let tamano_subtitulo =
                24;

            let tamano_stats =
                22;

            let tamano_opcion =
                20;

            let centro_x =
                screen_width
                    / 2;

            let centro_y =
                screen_height
                    / 2;

            let ancho_titulo =
                dibujo.measure_text(
                    titulo,
                    tamano_titulo,
                );

            let ancho_subtitulo =
                dibujo.measure_text(
                    subtitulo,
                    tamano_subtitulo,
                );

            let ancho_stats =
                dibujo.measure_text(
                    &stats,
                    tamano_stats,
                );

            let ancho_reiniciar =
                dibujo.measure_text(
                    reiniciar,
                    tamano_opcion,
                );

            dibujo.draw_text(
                titulo,

                centro_x
                    - ancho_titulo
                        / 2,

                centro_y
                    - 125,

                tamano_titulo,

                Color::GREEN,
            );

            dibujo.draw_text(
                subtitulo,

                centro_x
                    - ancho_subtitulo
                        / 2,

                centro_y
                    - 35,

                tamano_subtitulo,

                Color::WHITE,
            );

            dibujo.draw_text(
                &stats,

                centro_x
                    - ancho_stats
                        / 2,

                centro_y
                    + 15,

                tamano_stats,

                Color::LIGHTGRAY,
            );

            dibujo.draw_text(
                reiniciar,

                centro_x
                    - ancho_reiniciar
                        / 2,

                centro_y
                    + 80,

                tamano_opcion,

                Color::GRAY,
            );
        }
    }
}