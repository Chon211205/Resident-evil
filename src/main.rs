mod audio;
mod camera;
mod damage_effect;
mod framebuffer;
mod hud;
mod interaction;
mod inventory;
mod map;
mod map_renderer;
mod menu;
mod player;
mod puzzle;
mod raycaster;
mod sprite_renderer;
mod texture_data;
mod tyrant;
mod tyrant_renderer;
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

use menu::{
    AccionMenu,
    EstadoJuego,
    Menu,
};

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

use tyrant::Tyrant;
use tyrant_renderer::render_tyrant;

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

const DISTANCIA_MIN_SPAWN: f32 = 120.0;
const DISTANCIA_MAX_SPAWN: f32 = 380.0;
const DISTANCIA_ENTRE_ZOMBIES: f32 = 35.0;

fn crear_zombies(mapa: &mut Map) -> Vec<Zombie> {
    mapa.extraer_zombies()
        .into_iter()
        .map(|(x, y, tipo)| {
            match tipo {
                TipoSpawnZombie::Normal => {
                    Zombie::new(x, y)
                }

                TipoSpawnZombie::ConLlave => {
                    Zombie::new_con_llave(x, y)
                }

                TipoSpawnZombie::Medio => {
                    Zombie::new_medio(x, y)
                }

                TipoSpawnZombie::Fuerte => {
                    Zombie::new_fuerte(x, y)
                }
            }
        })
        .collect()
}

fn calcular_distancia(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;

    (dx * dx + dy * dy).sqrt()
}

fn max_zombies_para_horda(
    numero_horda: i32,
) -> usize {
    let extra =
        (numero_horda / 5)
            .max(0)
            as usize
            * 2;

    (18 + extra).min(30)
}

fn cantidad_horda(
    numero_horda: i32,
) -> usize {
    match numero_horda {
        1 => 7,
        2 => 9,
        3 => 11,
        4 => 13,
        5 => 14,

        _ => (
            14
                + (numero_horda - 5)
                    .max(0)
                    as usize
        )
            .min(18),
    }
}

fn buscar_spawn_horda(
    mapa: &Map,
    player: &Player,
    zombies: &[Zombie],
) -> Option<(f32, f32)> {
    if mapa.ancho() <= 2
        || mapa.alto() <= 2
    {
        return None;
    }

    let mut rng =
        rand::thread_rng();

    for _ in 0..800 {
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
            ' ' | 'C' | 'P' | 'O'
        ) {
            continue;
        }

        let x =
            columna as f32
                * TAMANO_CELDA
                + TAMANO_CELDA / 2.0;

        let y =
            fila as f32
                * TAMANO_CELDA
                + TAMANO_CELDA / 2.0;

        let distancia =
            calcular_distancia(
                player.x,
                player.y,
                x,
                y,
            );

        if distancia
            < DISTANCIA_MIN_SPAWN
        {
            continue;
        }

        if distancia
            > DISTANCIA_MAX_SPAWN
        {
            continue;
        }

        let ocupado =
            zombies
                .iter()
                .filter(|zombie| zombie.vivo)
                .any(|zombie| {
                    calcular_distancia(
                        zombie.x,
                        zombie.y,
                        x,
                        y,
                    ) < DISTANCIA_ENTRE_ZOMBIES
                });

        if ocupado {
            continue;
        }

        return Some((x, y));
    }

    None
}

fn crear_zombie_horda(
    numero_horda: i32,
    indice: usize,
    x: f32,
    y: f32,
) -> Zombie {
    match numero_horda {
        1 => {
            Zombie::new(x, y)
        }

        2 => {
            if indice % 4 == 0 {
                Zombie::new_medio(x, y)
            } else {
                Zombie::new(x, y)
            }
        }

        3 | 4 => {
            match indice % 6 {
                0 => {
                    Zombie::new_fuerte(x, y)
                }

                1 | 2 => {
                    Zombie::new_medio(x, y)
                }

                _ => {
                    Zombie::new(x, y)
                }
            }
        }

        _ => {
            match indice % 5 {
                0 | 1 => {
                    Zombie::new_fuerte(x, y)
                }

                2 | 3 => {
                    Zombie::new_medio(x, y)
                }

                _ => {
                    Zombie::new(x, y)
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
            .filter(|zombie| zombie.vivo)
            .count();

    let maximo =
        max_zombies_para_horda(
            numero_horda,
        );

    if vivos >= maximo {
        return 0;
    }

    let cantidad =
        cantidad_horda(
            numero_horda,
        )
            .min(
                maximo
                    .saturating_sub(vivos),
            );

    let mut generados = 0;

    for indice in 0..cantidad {
        let Some((x, y)) =
            buscar_spawn_horda(
                mapa,
                player,
                zombies,
            )
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

        zombie.persiguiendo = true;

        zombies.push(zombie);

        generados += 1;
    }

    generados
}

fn buscar_tyrant(
    mapa: &mut Map,
) -> Option<Tyrant> {
    for fila in 0..mapa.alto() {
        for columna in 0..mapa.ancho() {
            if mapa.celda(
                fila as i32,
                columna as i32,
            ) == 'Y'
            {
                let x =
                    columna as f32
                        * TAMANO_CELDA
                        + TAMANO_CELDA / 2.0;

                let y =
                    fila as f32
                        * TAMANO_CELDA
                        + TAMANO_CELDA / 2.0;

                mapa.cambiar_celda(
                    fila as i32,
                    columna as i32,
                    ' ',
                );

                return Some(
                    Tyrant::new(
                        x,
                        y,
                    ),
                );
            }
        }
    }

    None
}

fn detener_sonidos_enemigos(
    sonidos: &AudioManager<'_>,
) {
    sonidos.detener_zombie();
    sonidos.detener_zombie_medio();
    sonidos.detener_zombie_fuerte();
    sonidos.detener_tyrant();
}

fn cambiar_nivel(
    nivel_destino: i32,
    portal_destino: char,
    indice_portal: usize,
    mapa: &mut Map,
    zombies: &mut Vec<Zombie>,
    tyrant: &mut Option<Tyrant>,
    player: &mut Player,
    camera: &mut Camera,
    puzzle: &mut Puzzle,
    damage_effect: &mut DamageEffect,
) {
    *mapa =
        Map::new(
            nivel_destino,
        );

    *zombies =
        crear_zombies(
            mapa,
        );

    *tyrant =
        buscar_tyrant(
            mapa,
        );

    *player =
        Player::new(
            mapa,
        );

    if let Some((x, y)) =
        mapa.posicion_entrada_portal(
            portal_destino,
            indice_portal,
        )
    {
        player.x = x;
        player.y = y;
    }

    *camera =
        Camera::new();

    *puzzle =
        Puzzle::new();

    *damage_effect =
        DamageEffect::new();
}

fn main() {
    let mut estado_juego =
        EstadoJuego::Menu;

    let mut menu =
        Menu::new();

    let mut nivel_actual =
        1;

    let mut enemigos_matados =
        0_i32;

    let mut numero_horda =
        1_i32;

    let mut siguiente_horda =
        4_i32;

    let mut tiempo_cambio_nivel =
        0.0_f32;

    let mut mapa =
        Map::new(
            nivel_actual,
        );

    let mut zombies =
        crear_zombies(
            &mut mapa,
        );

    let mut tyrant =
        buscar_tyrant(
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
        "NIVEL 1".to_string();

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
                "Survival Horror Arcade",
            )
            .build();

    ventana.toggle_fullscreen();
    ventana.set_target_fps(60);
    ventana.enable_cursor();

    let audio =
        RaylibAudio::
            init_audio_device()
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
            .unwrap();

    let pistol2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistol2.png",
            )
            .unwrap();

    let pistol3 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistol3.png",
            )
            .unwrap();

    let pistol_r =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistolR.png",
            )
            .unwrap();

    let pistol_r2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/pistolR2.png",
            )
            .unwrap();

    let axe1 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/axe1.png",
            )
            .unwrap();

    let axe2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/axe2.png",
            )
            .unwrap();

    let axe3 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/axe3.png",
            )
            .unwrap();

    let key_texture =
        ventana
            .load_texture(
                &thread,
                "assets/textures/key.png",
            )
            .unwrap();

    let ammo_texture =
        ventana
            .load_texture(
                &thread,
                "assets/textures/ammo.png",
            )
            .unwrap();

    let heal_texture =
        ventana
            .load_texture(
                &thread,
                "assets/textures/heal.png",
            )
            .unwrap();

    let zombie1 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombie1.png",
            )
            .unwrap();

    let zombie2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombie2.png",
            )
            .unwrap();

    let zombie3 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombie3.png",
            )
            .unwrap();

    let zombie_v21 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombieV21.png",
            )
            .unwrap();

    let zombie_v22 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombieV22.png",
            )
            .unwrap();

    let zombie_v23 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombiev23.png",
            )
            .unwrap();

    let zombie_v31 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombieV31.png",
            )
            .unwrap();

    let zombie_v32 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombieV32.png",
            )
            .unwrap();

    let zombie_v33 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/zombieV33.png",
            )
            .unwrap();

    let tyrant1 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/tyrant1.png",
            )
            .expect(
                "No se pudo cargar tyrant1.png",
            );

    let tyrant2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/tyrant2.png",
            )
            .expect(
                "No se pudo cargar tyrant2.png",
            );

    let tyrant3 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/tyrant3.png",
            )
            .expect(
                "No se pudo cargar tyrant3.png",
            );

    let mut wall_image =
        Image::load_image(
            "assets/textures/wall.png",
        )
        .unwrap();

    let mut window_image =
        Image::load_image(
            "assets/textures/window.png",
        )
        .unwrap();

    let mut door_image =
        Image::load_image(
            "assets/textures/door.png",
        )
        .unwrap();

    let mut up_image =
        Image::load_image(
            "assets/textures/up.png",
        )
        .unwrap();

    let mut down_image =
        Image::load_image(
            "assets/textures/down.png",
        )
        .unwrap();

    let mut floor_image =
        Image::load_image(
            "assets/textures/floor.png",
        )
        .unwrap();

    let mut floor2_image =
        Image::load_image(
            "assets/textures/floor2.png",
        )
        .unwrap();

    let mut roof_image =
        Image::load_image(
            "assets/textures/roof.png",
        )
        .unwrap();

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

    let textura_subir =
        TextureData::from_image(
            &mut up_image,
        );

    let textura_bajar =
        TextureData::from_image(
            &mut down_image,
        );

    let textura_suelo =
        TextureData::from_image(
            &mut floor_image,
        );

    let textura_suelo2 =
        TextureData::from_image(
            &mut floor2_image,
        );

    let textura_techo =
        TextureData::from_image(
            &mut roof_image,
        );

    let mut textura_framebuffer =
        ventana
            .load_texture_from_image(
                &thread,
                framebuffer.image(),
            )
            .unwrap();

    while !ventana
        .window_should_close()
    {
        let delta_time =
            ventana
                .get_frame_time();

        match estado_juego {
            EstadoJuego::Menu => {
                detener_sonidos_enemigos(
                    &sonidos,
                );

                ventana.enable_cursor();

                match menu.update(
                    &ventana,
                ) {
                    AccionMenu::Jugar => {
                        estado_juego =
                            EstadoJuego::Jugando;

                        ventana.disable_cursor();
                    }

                    AccionMenu::Controles => {
                        estado_juego =
                            EstadoJuego::Controles;
                    }

                    AccionMenu::Salir => {
                        break;
                    }

                    AccionMenu::Ninguna => {}
                }

                let mut dibujo =
                    ventana
                        .begin_drawing(
                            &thread,
                        );

                menu.render_menu(
                    &mut dibujo,
                );

                continue;
            }

            EstadoJuego::Controles => {
                detener_sonidos_enemigos(
                    &sonidos,
                );

                ventana.enable_cursor();

                if ventana
                    .is_key_pressed(
                        KeyboardKey::
                            KEY_BACKSPACE,
                    )
                {
                    estado_juego =
                        EstadoJuego::Menu;
                }

                let mut dibujo =
                    ventana
                        .begin_drawing(
                            &thread,
                        );

                menu.render_controles(
                    &mut dibujo,
                );

                continue;
            }

            EstadoJuego::Jugando => {}
        }

        if ventana
            .is_key_pressed(
                KeyboardKey::
                    KEY_BACKSPACE,
            )
        {
            detener_sonidos_enemigos(
                &sonidos,
            );

            estado_juego =
                EstadoJuego::Menu;

            ventana.enable_cursor();

            continue;
        }

        if tiempo_cambio_nivel > 0.0 {
            tiempo_cambio_nivel -=
                delta_time;

            tiempo_cambio_nivel =
                tiempo_cambio_nivel
                    .max(0.0);
        }

        if tiempo_disparo > 0.0 {
            tiempo_disparo -=
                delta_time;

            tiempo_disparo =
                tiempo_disparo
                    .max(0.0);
        }

        if tiempo_hachazo > 0.0 {
            tiempo_hachazo -=
                delta_time;

            tiempo_hachazo =
                tiempo_hachazo
                    .max(0.0);
        }

        if recargando
            && vida_jugador > 0
        {
            tiempo_recarga -=
                delta_time;

            if tiempo_recarga <= 0.0 {
                recargando = false;
                tiempo_recarga = 0.0;

                let faltantes =
                    8 - balas_cargador;

                let cantidad =
                    faltantes.min(
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

        if vida_jugador > 0 {
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
                && arma_equipada
                    == ArmaActual::Hacha
                && tiempo_hachazo <= 0.0
                && ventana
                    .is_mouse_button_down(
                        MouseButton::
                            MOUSE_BUTTON_RIGHT,
                    );

        if vida_jugador > 0 {
            for zombie
                in &mut zombies
            {
                let dano =
                    zombie.update(
                        &player,
                        &mapa,
                        delta_time,
                    );

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
                        vida_jugador
                            .max(0);

                    if bloqueo_valido {
                        sonidos
                            .bloqueo_hacha();

                        mensaje =
                            "Ataque bloqueado"
                                .to_string();
                    } else {
                        sonidos.dano();

                        mensaje =
                            format!(
                                "Dano recibido: {}",
                                dano_final,
                            );
                    }

                    damage_effect
                        .activar();
                }
            }
        }

        let hay_normal =
            zombies
                .iter()
                .any(|zombie| {
                    zombie.vivo
                        && zombie.persiguiendo
                        && zombie.tipo
                            == TipoZombie::Normal
                });

        let hay_medio =
            zombies
                .iter()
                .any(|zombie| {
                    zombie.vivo
                        && zombie.persiguiendo
                        && zombie.tipo
                            == TipoZombie::Medio
                });

        let hay_fuerte =
            zombies
                .iter()
                .any(|zombie| {
                    zombie.vivo
                        && zombie.persiguiendo
                        && zombie.tipo
                            == TipoZombie::Fuerte
                });

        if vida_jugador > 0
            && hay_normal
        {
            sonidos.zombie();
        } else {
            sonidos.detener_zombie();
        }

        if vida_jugador > 0
            && hay_medio
        {
            sonidos.zombie_medio();
        } else {
            sonidos.detener_zombie_medio();
        }

        if vida_jugador > 0
            && hay_fuerte
        {
            sonidos.zombie_fuerte();
        } else {
            sonidos.detener_zombie_fuerte();
        }

        if vida_jugador > 0 {
            if let Some(
                tyrant_actual,
            ) =
                tyrant.as_mut()
            {
                let distancia_tyrant =
                    calcular_distancia(
                        player.x,
                        player.y,
                        tyrant_actual.x,
                        tyrant_actual.y,
                    );

                if distancia_tyrant
                    <= 450.0
                {
                    sonidos.tyrant();
                } else {
                    sonidos.detener_tyrant();
                }

                let dano =
                    tyrant_actual.update(
                        &player,
                        &mapa,
                        delta_time,
                    );

                if dano > 0 {
                    let dano_final =
                        if bloqueando {
                            8
                        } else {
                            dano
                        };

                    vida_jugador -=
                        dano_final;

                    vida_jugador =
                        vida_jugador
                            .max(0);

                    if bloqueando {
                        sonidos
                            .bloqueo_hacha();

                        mensaje =
                            "Bloqueaste al TYRANT"
                                .to_string();
                    } else {
                        sonidos.dano();

                        mensaje =
                            format!(
                                "TYRANT -{} VIDA",
                                dano_final,
                            );
                    }

                    damage_effect
                        .activar();
                }
            } else {
                sonidos.detener_tyrant();
            }
        } else {
            sonidos.detener_tyrant();
        }

        if vida_jugador > 0 {
            let resultado =
                recoger_objetos_cercanos(
                    &mut mapa,
                    &player,
                    &mut inventory,
                    &mut puzzle,
                    vida_jugador,
                );

            match resultado {
                InteractionResult::
                    LlaveRecogida =>
                {
                    sonidos.llave();

                    mensaje =
                        "Recogiste una llave"
                            .to_string();
                }

                InteractionResult::
                    MunicionRecogida(
                        cantidad,
                    ) =>
                {
                    sonidos
                        .recoger_municion();

                    balas_reserva +=
                        cantidad;

                    mensaje =
                        format!(
                            "+{} balas",
                            cantidad,
                        );
                }

                InteractionResult::
                    CuracionRecogida(
                        cantidad,
                    ) =>
                {
                    sonidos.curacion();

                    vida_jugador =
                        (
                            vida_jugador
                                + cantidad
                        )
                            .min(100);

                    mensaje =
                        format!(
                            "+{} vida",
                            cantidad,
                        );
                }

                _ => {}
            }
        }

        if vida_jugador > 0 {
            if ventana
                .is_key_pressed(
                    KeyboardKey::KEY_ONE,
                )
            {
                arma_equipada =
                    ArmaActual::Pistola;

                tiempo_hachazo = 0.0;

                mensaje =
                    "Pistola equipada"
                        .to_string();
            }

            if ventana
                .is_key_pressed(
                    KeyboardKey::KEY_TWO,
                )
            {
                arma_equipada =
                    ArmaActual::Hacha;

                recargando = false;
                tiempo_recarga = 0.0;
                tiempo_disparo = 0.0;

                mensaje =
                    "Hacha equipada"
                        .to_string();
            }

            if ventana
                .is_key_pressed(
                    KeyboardKey::KEY_E,
                )
            {
                match interactuar(
                    &mut mapa,
                    &player,
                    &camera,
                    &mut inventory,
                    &mut puzzle,
                ) {
                    InteractionResult::
                        PuertaAbierta =>
                    {
                        sonidos.puerta();

                        mensaje =
                            "Abriste la puerta"
                                .to_string();
                    }

                    InteractionResult::
                        PuertaCerrada =>
                    {
                        mensaje =
                            "Necesitas una llave"
                                .to_string();
                    }

                    InteractionResult::
                        SubirNivel(
                            indice,
                        ) =>
                    {
                        if nivel_actual == 1
                            && tiempo_cambio_nivel
                                <= 0.0
                        {
                            detener_sonidos_enemigos(
                                &sonidos,
                            );

                            nivel_actual = 2;

                            cambiar_nivel(
                                nivel_actual,
                                'B',
                                indice,
                                &mut mapa,
                                &mut zombies,
                                &mut tyrant,
                                &mut player,
                                &mut camera,
                                &mut puzzle,
                                &mut damage_effect,
                            );

                            tiempo_cambio_nivel =
                                1.0;

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
                                    "NIVEL 2 - HORDA {}",
                                    numero_horda,
                                );
                        }
                    }

                    InteractionResult::
                        BajarNivel(
                            indice,
                        ) =>
                    {
                        if nivel_actual == 2
                            && tiempo_cambio_nivel
                                <= 0.0
                        {
                            detener_sonidos_enemigos(
                                &sonidos,
                            );

                            nivel_actual = 1;

                            cambiar_nivel(
                                nivel_actual,
                                'X',
                                indice,
                                &mut mapa,
                                &mut zombies,
                                &mut tyrant,
                                &mut player,
                                &mut camera,
                                &mut puzzle,
                                &mut damage_effect,
                            );

                            tiempo_cambio_nivel =
                                1.0;

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
                                    "NIVEL 1 - HORDA {}",
                                    numero_horda,
                                );
                        }
                    }

                    _ => {}
                }
            }
        }

        let apuntando =
            vida_jugador > 0
                && arma_equipada
                    == ArmaActual::Pistola
                && !recargando
                && ventana
                    .is_mouse_button_down(
                        MouseButton::
                            MOUSE_BUTTON_RIGHT,
                    );

        if vida_jugador > 0
            && ventana
                .is_mouse_button_pressed(
                    MouseButton::
                        MOUSE_BUTTON_LEFT,
                )
        {
            match arma_equipada {
                ArmaActual::Pistola => {
                    if !recargando {
                        if balas_cargador > 0 {
                            balas_cargador -= 1;

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
                                enemigos_matados +=
                                    (
                                        vivos_antes
                                            - vivos_despues
                                    )
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
                                            "Impacto - {} HP",
                                            vida_restante,
                                        )
                                    }

                                    ShotResult::
                                        HeadshotHit {
                                            vida_restante,
                                        } =>
                                    {
                                        format!(
                                            "HEADSHOT x2 - {} HP",
                                            vida_restante,
                                        )
                                    }

                                    ShotResult::Kill => {
                                        format!(
                                            "BAJAS: {}",
                                            enemigos_matados,
                                        )
                                    }

                                    ShotResult::
                                        KillConLlave =>
                                    {
                                        format!(
                                            "BAJAS: {} - LLAVE",
                                            enemigos_matados,
                                        )
                                    }

                                    ShotResult::
                                        HeadshotKill =>
                                    {
                                        format!(
                                            "HEADSHOT x2 - BAJA {}",
                                            enemigos_matados,
                                        )
                                    }

                                    ShotResult::
                                        HeadshotKillConLlave =>
                                    {
                                        format!(
                                            "HEADSHOT x2 - BAJA {} - LLAVE",
                                            enemigos_matados,
                                        )
                                    }
                                };
                        } else {
                            sonidos
                                .sin_municion();

                            mensaje =
                                "Sin municion"
                                    .to_string();
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
                            enemigos_matados +=
                                (
                                    vivos_antes
                                        - vivos_despues
                                )
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
                                        "Golpe - {} HP",
                                        vida_restante,
                                    )
                                }

                                ShotResult::Kill => {
                                    format!(
                                        "BAJAS: {}",
                                        enemigos_matados,
                                    )
                                }

                                ShotResult::
                                    KillConLlave =>
                                {
                                    format!(
                                        "BAJAS: {} - LLAVE",
                                        enemigos_matados,
                                    )
                                }

                                ShotResult::
                                    HeadshotHit {
                                            vida_restante,
                                        } =>
                                {
                                    format!(
                                        "Golpe - {} HP",
                                        vida_restante,
                                    )
                                }

                                ShotResult::
                                    HeadshotKill =>
                                {
                                    format!(
                                        "BAJAS: {}",
                                        enemigos_matados,
                                    )
                                }

                                ShotResult::
                                    HeadshotKillConLlave =>
                                {
                                    format!(
                                        "BAJAS: {} - LLAVE",
                                        enemigos_matados,
                                    )
                                }
                            };
                    }
                }
            }
        }

        while vida_jugador > 0
            && enemigos_matados
                >= siguiente_horda
        {
            let generados =
                generar_horda(
                    &mut zombies,
                    &mapa,
                    &player,
                    numero_horda,
                );

            if generados > 0 {
                mensaje =
                    format!(
                        "HORDA {} - {} ENEMIGOS",
                        numero_horda,
                        generados,
                    );
            }

            numero_horda += 1;

            siguiente_horda += 6;
        }

        if vida_jugador > 0
            && ventana
                .is_key_pressed(
                    KeyboardKey::KEY_R,
                )
            && arma_equipada
                == ArmaActual::Pistola
            && !recargando
        {
            if balas_cargador == 8 {
                mensaje =
                    "Cargador lleno"
                        .to_string();
            } else if balas_reserva <= 0 {
                mensaje =
                    "Sin balas de reserva"
                        .to_string();
            } else {
                recargando = true;

                tiempo_recarga =
                    DURACION_RECARGA;

                sonidos.recarga();

                mensaje =
                    "Recargando..."
                        .to_string();
            }
        }

        if ventana
            .is_key_pressed(
                KeyboardKey::KEY_F5,
            )
        {
            detener_sonidos_enemigos(
                &sonidos,
            );

            nivel_actual = 1;
            enemigos_matados = 0;
            numero_horda = 1;
            siguiente_horda = 4;
            tiempo_cambio_nivel = 0.0;

            mapa =
                Map::new(1);

            zombies =
                crear_zombies(
                    &mut mapa,
                );

            tyrant =
                buscar_tyrant(
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

            vida_jugador = 100;

            balas_cargador = 8;
            balas_reserva = 24;

            tiempo_disparo = 0.0;
            tiempo_hachazo = 0.0;
            tiempo_recarga = 0.0;

            recargando = false;

            mensaje =
                "NIVEL 1"
                    .to_string();

            ventana.disable_cursor();
        }

        if ventana
            .is_key_pressed(
                KeyboardKey::KEY_F11,
            )
        {
            ventana.toggle_fullscreen();
        }

        if ventana
            .is_key_pressed(
                KeyboardKey::KEY_TAB,
            )
        {
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
            &textura_subir,
            &textura_bajar,
            &textura_suelo,
            &textura_suelo2,
            &textura_techo,
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
            .unwrap();

        let pantalla_ancho =
            ventana
                .get_screen_width()
                as f32;

        let pantalla_alto =
            ventana
                .get_screen_height()
                as f32;

        let escala =
            (
                pantalla_ancho
                    / ANCHO_VENTANA
                        as f32
            )
                .min(
                    pantalla_alto
                        / ALTO_VENTANA
                            as f32,
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

        let mut dibujo =
            ventana
                .begin_drawing(
                    &thread,
                );

        dibujo
            .clear_background(
                Color::BLACK,
            );

        dibujo
            .draw_texture_pro(
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

        unsafe {
            raylib::ffi::
                BeginScissorMode(
                    offset_x
                        .round()
                        as i32,

                    offset_y
                        .round()
                        as i32,

                    ancho_render
                        .round()
                        .max(1.0)
                        as i32,

                    alto_render
                        .round()
                        .max(1.0)
                        as i32,
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

        if let Some(
            tyrant_actual,
        ) =
            tyrant.as_ref()
        {
            render_tyrant(
                &mut dibujo,
                &mapa,
                &player,
                &camera,
                tyrant_actual,
                &tyrant1,
                &tyrant2,
                &tyrant3,
                offset_x,
                offset_y,
                escala,
            );
        }

        if vida_jugador > 0 {
            dibujo
                .draw_texture_ex(
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
                        + ancho_render / 2.0;

                let mira_y =
                    offset_y
                        + alto_render / 2.0;

                dibujo.draw_circle(
                    mira_x as i32,
                    mira_y as i32,
                    3.0 * escala.max(1.0),
                    Color::RED,
                );
            }
        }

        unsafe {
            raylib::ffi::
                EndScissorMode();
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

        let vivos =
            zombies
                .iter()
                .filter(|zombie| {
                    zombie.vivo
                })
                .count();

        let texto_arcade =
            format!(
                "NIVEL {} | HORDA {} | BAJAS {} | ENEMIGOS {}",
                nivel_actual,
                numero_horda,
                enemigos_matados,
                vivos,
            );

        dibujo.draw_text(
            &texto_arcade,
            (offset_x + 15.0) as i32,
            (offset_y + 115.0) as i32,
            18,
            Color::WHITE,
        );

        damage_effect.render(
            &mut dibujo,
        );

        if vida_jugador <= 0 {
            detener_sonidos_enemigos(
                &sonidos,
            );

            let sw =
                dibujo
                    .get_screen_width();

            let sh =
                dibujo
                    .get_screen_height();

            dibujo.draw_rectangle(
                0,
                0,
                sw,
                sh,
                Color::new(
                    0,
                    0,
                    0,
                    220,
                ),
            );

            let titulo =
                "GAME OVER";

            let stats =
                format!(
                    "HORDA {} - {} BAJAS",
                    numero_horda,
                    enemigos_matados,
                );

            let reiniciar =
                "F5 - JUGAR DE NUEVO";

            let menu_texto =
                "BACKSPACE - MENU PRINCIPAL";

            let ancho_titulo =
                dibujo.measure_text(
                    titulo,
                    64,
                );

            let ancho_stats =
                dibujo.measure_text(
                    &stats,
                    26,
                );

            let ancho_reiniciar =
                dibujo.measure_text(
                    reiniciar,
                    22,
                );

            let ancho_menu =
                dibujo.measure_text(
                    menu_texto,
                    20,
                );

            dibujo.draw_text(
                titulo,
                sw / 2
                    - ancho_titulo / 2,
                sh / 2
                    - 110,
                64,
                Color::RED,
            );

            dibujo.draw_text(
                &stats,
                sw / 2
                    - ancho_stats / 2,
                sh / 2
                    - 10,
                26,
                Color::WHITE,
            );

            dibujo.draw_text(
                reiniciar,
                sw / 2
                    - ancho_reiniciar / 2,
                sh / 2
                    + 50,
                22,
                Color::LIGHTGRAY,
            );

            dibujo.draw_text(
                menu_texto,
                sw / 2
                    - ancho_menu / 2,
                sh / 2
                    + 85,
                20,
                Color::GRAY,
            );
        }
    }
}