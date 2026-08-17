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
    AccionSeleccionNivel,
    EstadoJuego,
    Menu,
    NivelSeleccionado,
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

const META_NORMAL: i32 = 20;
const META_MEDIO: i32 = 15;
const META_FUERTE: i32 = 5;

fn crear_zombies(
    mapa: &mut Map,
) -> Vec<Zombie> {
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

fn contar_vivos_tipo(
    zombies: &[Zombie],
    tipo: TipoZombie,
) -> usize {
    zombies
        .iter()
        .filter(|zombie| {
            zombie.vivo
                && zombie.tipo == tipo
        })
        .count()
}

fn actualizar_bajas_por_tipo(
    normal_antes: usize,
    medio_antes: usize,
    fuerte_antes: usize,
    zombies: &[Zombie],
    bajas_normal: &mut i32,
    bajas_medio: &mut i32,
    bajas_fuerte: &mut i32,
) {
    let normal_despues =
        contar_vivos_tipo(
            zombies,
            TipoZombie::Normal,
        );

    let medio_despues =
        contar_vivos_tipo(
            zombies,
            TipoZombie::Medio,
        );

    let fuerte_despues =
        contar_vivos_tipo(
            zombies,
            TipoZombie::Fuerte,
        );

    if normal_despues < normal_antes {
        *bajas_normal +=
            (normal_antes - normal_despues)
                as i32;
    }

    if medio_despues < medio_antes {
        *bajas_medio +=
            (medio_antes - medio_despues)
                as i32;
    }

    if fuerte_despues < fuerte_antes {
        *bajas_fuerte +=
            (fuerte_antes - fuerte_despues)
                as i32;
    }
}

fn objetivo_completo(
    bajas_normal: i32,
    bajas_medio: i32,
    bajas_fuerte: i32,
) -> bool {
    bajas_normal >= META_NORMAL
        && bajas_medio >= META_MEDIO
        && bajas_fuerte >= META_FUERTE
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

        _ => {
            (
                14
                    + (numero_horda - 5)
                        .max(0)
                        as usize
            )
                .min(18)
        }
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

        if distancia < DISTANCIA_MIN_SPAWN {
            continue;
        }

        if distancia > DISTANCIA_MAX_SPAWN {
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
        1 => Zombie::new(x, y),

        2 => {
            if indice % 4 == 0 {
                Zombie::new_medio(x, y)
            } else {
                Zombie::new(x, y)
            }
        }

        3 | 4 => {
            match indice % 6 {
                0 => Zombie::new_fuerte(x, y),

                1 | 2 => {
                    Zombie::new_medio(x, y)
                }

                _ => Zombie::new(x, y),
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

                _ => Zombie::new(x, y),
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

fn cambiar_nivel_mansion(
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

fn numero_mapa_inicial(
    nivel: NivelSeleccionado,
) -> i32 {
    match nivel {
        NivelSeleccionado::Mansion => 1,
        NivelSeleccionado::Laboratorio => 3,
    }
}

fn nombre_nivel(
    nivel: NivelSeleccionado,
) -> &'static str {
    match nivel {
        NivelSeleccionado::Mansion => {
            "MANSION"
        }

        NivelSeleccionado::Laboratorio => {
            "LABORATORIO"
        }
    }
}

fn main() {
    let mut estado_juego =
        EstadoJuego::Menu;

    let mut menu =
        Menu::new();

    let mut nivel_seleccionado =
        NivelSeleccionado::Mansion;

    let mut nivel_actual =
        1;

    let mut enemigos_matados =
        0_i32;

    let mut bajas_normal =
        0_i32;

    let mut bajas_medio =
        0_i32;

    let mut bajas_fuerte =
        0_i32;

    let mut objetivo_completado =
        false;

    let mut pantalla_great =
        false;

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
        "MANSION".to_string();

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
            .unwrap();

    let tyrant2 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/tyrant2.png",
            )
            .unwrap();

    let tyrant3 =
        ventana
            .load_texture(
                &thread,
                "assets/textures/tyrant3.png",
            )
            .unwrap();

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

    while !ventana.window_should_close() {
        let delta_time =
            ventana.get_frame_time();

        if estado_juego
            == EstadoJuego::Jugando
            && vida_jugador > 0
        {
            sonidos.actualizar_musica();
        }

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
                        nivel_seleccionado =
                            NivelSeleccionado::Mansion;

                        nivel_actual =
                            numero_mapa_inicial(
                                nivel_seleccionado,
                            );

                        mapa =
                            Map::new(
                                nivel_actual,
                            );

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

                        vida_jugador = 100;

                        balas_cargador = 8;
                        balas_reserva = 24;

                        arma_equipada =
                            ArmaActual::Pistola;

                        enemigos_matados = 0;

                        bajas_normal = 0;
                        bajas_medio = 0;
                        bajas_fuerte = 0;

                        objetivo_completado =
                            false;

                        pantalla_great =
                            false;

                        numero_horda = 1;
                        siguiente_horda = 4;

                        tiempo_disparo = 0.0;
                        tiempo_hachazo = 0.0;
                        tiempo_recarga = 0.0;
                        tiempo_cambio_nivel = 0.0;

                        recargando = false;

                        mensaje =
                            nombre_nivel(
                                nivel_seleccionado,
                            )
                                .to_string();

                        estado_juego =
                            EstadoJuego::Jugando;

                        sonidos
                            .detener_musica();

                        sonidos
                            .iniciar_musica();

                        ventana
                            .disable_cursor();
                    }

                    AccionMenu::
                        SeleccionarNivel =>
                    {
                        estado_juego =
                            EstadoJuego::
                                SeleccionNivel;
                    }

                    AccionMenu::Controles => {
                        estado_juego =
                            EstadoJuego::Controles;
                    }

                    AccionMenu::Salir => {
                        sonidos
                            .detener_musica();

                        break;
                    }

                    AccionMenu::Ninguna => {}
                }

                let mut dibujo =
                    ventana.begin_drawing(
                        &thread,
                    );

                menu.render_menu(
                    &mut dibujo,
                );

                continue;
            }

            EstadoJuego::SeleccionNivel => {
                detener_sonidos_enemigos(
                    &sonidos,
                );

                ventana.enable_cursor();

                match menu
                    .update_seleccion_nivel(
                        &ventana,
                    )
                {
                    AccionSeleccionNivel::
                        Elegir(nivel) =>
                    {
                        nivel_seleccionado =
                            nivel;

                        nivel_actual =
                            numero_mapa_inicial(
                                nivel_seleccionado,
                            );

                        mapa =
                            Map::new(
                                nivel_actual,
                            );

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

                        vida_jugador = 100;

                        balas_cargador = 8;
                        balas_reserva = 24;

                        arma_equipada =
                            ArmaActual::Pistola;

                        enemigos_matados = 0;

                        bajas_normal = 0;
                        bajas_medio = 0;
                        bajas_fuerte = 0;

                        objetivo_completado =
                            false;

                        pantalla_great =
                            false;

                        numero_horda = 1;
                        siguiente_horda = 4;

                        tiempo_disparo = 0.0;
                        tiempo_hachazo = 0.0;
                        tiempo_recarga = 0.0;
                        tiempo_cambio_nivel = 0.0;

                        recargando = false;

                        mensaje =
                            nombre_nivel(
                                nivel_seleccionado,
                            )
                                .to_string();

                        sonidos
                            .detener_musica();

                        sonidos
                            .iniciar_musica();

                        estado_juego =
                            EstadoJuego::Jugando;

                        ventana
                            .disable_cursor();
                    }

                    AccionSeleccionNivel::Volver => {
                        estado_juego =
                            EstadoJuego::Menu;
                    }

                    AccionSeleccionNivel::Ninguna => {}
                }

                let mut dibujo =
                    ventana.begin_drawing(
                        &thread,
                    );

                menu.render_seleccion_nivel(
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
                    ventana.begin_drawing(
                        &thread,
                    );

                menu.render_controles(
                    &mut dibujo,
                );

                continue;
            }

            EstadoJuego::Jugando => {}
        }

        if pantalla_great {
            detener_sonidos_enemigos(
                &sonidos,
            );

            sonidos.actualizar_musica();

            ventana.enable_cursor();

            if ventana
                .is_key_pressed(
                    KeyboardKey::KEY_ENTER,
                )
            {
                pantalla_great =
                    false;

                mensaje =
                    "MODO ARCADE"
                        .to_string();

                ventana
                    .disable_cursor();

                continue;
            }

            if ventana
                .is_key_pressed(
                    KeyboardKey::
                        KEY_BACKSPACE,
                )
            {
                pantalla_great =
                    false;

                estado_juego =
                    EstadoJuego::Menu;

                sonidos
                    .detener_musica();

                ventana
                    .enable_cursor();

                continue;
            }

            let mut dibujo =
                ventana.begin_drawing(
                    &thread,
                );

            dibujo.clear_background(
                Color::new(
                    8,
                    8,
                    8,
                    255,
                ),
            );

            let sw =
                dibujo.get_screen_width();

            let sh =
                dibujo.get_screen_height();

            let titulo =
                "GREAT!";

            let subtitulo =
                "OBJETIVO COMPLETADO";

            let normal =
                format!(
                    "ZOMBIES NORMALES: {}/{}",
                    bajas_normal.min(
                        META_NORMAL,
                    ),
                    META_NORMAL,
                );

            let medio =
                format!(
                    "ZOMBIES MEDIOS: {}/{}",
                    bajas_medio.min(
                        META_MEDIO,
                    ),
                    META_MEDIO,
                );

            let fuerte =
                format!(
                    "ZOMBIES FUERTES: {}/{}",
                    bajas_fuerte.min(
                        META_FUERTE,
                    ),
                    META_FUERTE,
                );

            let arcade =
                "ENTER - SEGUIR JUGANDO";

            let volver =
                "BACKSPACE - MENU PRINCIPAL";

            let titulo_w =
                dibujo.measure_text(
                    titulo,
                    80,
                );

            dibujo.draw_text(
                titulo,
                sw / 2
                    - titulo_w / 2,
                sh / 2
                    - 190,
                80,
                Color::GOLD,
            );

            let sub_w =
                dibujo.measure_text(
                    subtitulo,
                    28,
                );

            dibujo.draw_text(
                subtitulo,
                sw / 2
                    - sub_w / 2,
                sh / 2
                    - 95,
                28,
                Color::WHITE,
            );

            let normal_w =
                dibujo.measure_text(
                    &normal,
                    24,
                );

            dibujo.draw_text(
                &normal,
                sw / 2
                    - normal_w / 2,
                sh / 2
                    - 30,
                24,
                Color::LIGHTGRAY,
            );

            let medio_w =
                dibujo.measure_text(
                    &medio,
                    24,
                );

            dibujo.draw_text(
                &medio,
                sw / 2
                    - medio_w / 2,
                sh / 2
                    + 5,
                24,
                Color::LIGHTGRAY,
            );

            let fuerte_w =
                dibujo.measure_text(
                    &fuerte,
                    24,
                );

            dibujo.draw_text(
                &fuerte,
                sw / 2
                    - fuerte_w / 2,
                sh / 2
                    + 40,
                24,
                Color::LIGHTGRAY,
            );

            let arcade_w =
                dibujo.measure_text(
                    arcade,
                    24,
                );

            dibujo.draw_text(
                arcade,
                sw / 2
                    - arcade_w / 2,
                sh / 2
                    + 115,
                24,
                Color::GREEN,
            );

            let volver_w =
                dibujo.measure_text(
                    volver,
                    20,
                );

            dibujo.draw_text(
                volver,
                sw / 2
                    - volver_w / 2,
                sh / 2
                    + 155,
                20,
                Color::GRAY,
            );

            continue;
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

            sonidos
                .detener_musica();

            estado_juego =
                EstadoJuego::Menu;

            ventana.enable_cursor();

            continue;
        }

        tiempo_cambio_nivel =
            (
                tiempo_cambio_nivel
                    - delta_time
            )
                .max(0.0);

        tiempo_disparo =
            (
                tiempo_disparo
                    - delta_time
            )
                .max(0.0);

        tiempo_hachazo =
            (
                tiempo_hachazo
                    - delta_time
            )
                .max(0.0);

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
            for zombie in &mut zombies {
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
                        vida_jugador.max(0);

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

                    damage_effect.activar();
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
            sonidos
                .detener_zombie_medio();
        }

        if vida_jugador > 0
            && hay_fuerte
        {
            sonidos.zombie_fuerte();
        } else {
            sonidos
                .detener_zombie_fuerte();
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

                if distancia_tyrant <= 450.0 {
                    sonidos.tyrant();
                } else {
                    sonidos
                        .detener_tyrant();
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
                        vida_jugador.max(0);

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

                    damage_effect.activar();
                }
            } else {
                sonidos.detener_tyrant();
            }
        } else {
            sonidos.detener_tyrant();
        }

        if vida_jugador > 0 {
            match recoger_objetos_cercanos(
                &mut mapa,
                &player,
                &mut inventory,
                &mut puzzle,
                vida_jugador,
            ) {
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
                        SubirNivel(indice) =>
                    {
                        if nivel_seleccionado
                            == NivelSeleccionado::
                                Mansion
                            && nivel_actual == 1
                            && tiempo_cambio_nivel
                                <= 0.0
                        {
                            detener_sonidos_enemigos(
                                &sonidos,
                            );

                            nivel_actual = 2;

                            cambiar_nivel_mansion(
                                2,
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

                            tiempo_disparo = 0.0;
                            tiempo_hachazo = 0.0;
                            tiempo_recarga = 0.0;
                            recargando = false;

                            mensaje =
                                "MANSION - PISO 2"
                                    .to_string();
                        }
                    }

                    InteractionResult::
                        BajarNivel(indice) =>
                    {
                        if nivel_seleccionado
                            == NivelSeleccionado::
                                Mansion
                            && nivel_actual == 2
                            && tiempo_cambio_nivel
                                <= 0.0
                        {
                            detener_sonidos_enemigos(
                                &sonidos,
                            );

                            nivel_actual = 1;

                            cambiar_nivel_mansion(
                                1,
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

                            tiempo_disparo = 0.0;
                            tiempo_hachazo = 0.0;
                            tiempo_recarga = 0.0;
                            recargando = false;

                            mensaje =
                                "MANSION - PISO 1"
                                    .to_string();
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

                            let normal_antes =
                                contar_vivos_tipo(
                                    &zombies,
                                    TipoZombie::Normal,
                                );

                            let medio_antes =
                                contar_vivos_tipo(
                                    &zombies,
                                    TipoZombie::Medio,
                                );

                            let fuerte_antes =
                                contar_vivos_tipo(
                                    &zombies,
                                    TipoZombie::Fuerte,
                                );

                            let vivos_antes =
                                zombies
                                    .iter()
                                    .filter(|z| z.vivo)
                                    .count();

                            let resultado =
                                disparar(
                                    &mut zombies,
                                    &player,
                                    &camera,
                                    &mut mapa,
                                );

                            actualizar_bajas_por_tipo(
                                normal_antes,
                                medio_antes,
                                fuerte_antes,
                                &zombies,
                                &mut bajas_normal,
                                &mut bajas_medio,
                                &mut bajas_fuerte,
                            );

                            let vivos_despues =
                                zombies
                                    .iter()
                                    .filter(|z| z.vivo)
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

                        let normal_antes =
                            contar_vivos_tipo(
                                &zombies,
                                TipoZombie::Normal,
                            );

                        let medio_antes =
                            contar_vivos_tipo(
                                &zombies,
                                TipoZombie::Medio,
                            );

                        let fuerte_antes =
                            contar_vivos_tipo(
                                &zombies,
                                TipoZombie::Fuerte,
                            );

                        let vivos_antes =
                            zombies
                                .iter()
                                .filter(|z| z.vivo)
                                .count();

                        let resultado =
                            atacar_con_hacha(
                                &mut zombies,
                                &player,
                                &camera,
                                &mut mapa,
                            );

                        actualizar_bajas_por_tipo(
                            normal_antes,
                            medio_antes,
                            fuerte_antes,
                            &zombies,
                            &mut bajas_normal,
                            &mut bajas_medio,
                            &mut bajas_fuerte,
                        );

                        let vivos_despues =
                            zombies
                                .iter()
                                .filter(|z| z.vivo)
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

                                ShotResult::Kill
                                | ShotResult::
                                    HeadshotKill =>
                                {
                                    format!(
                                        "BAJAS: {}",
                                        enemigos_matados,
                                    )
                                }

                                ShotResult::
                                    KillConLlave
                                | ShotResult::
                                    HeadshotKillConLlave =>
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
                            };
                    }
                }
            }
        }

        if !objetivo_completado
            && objetivo_completo(
                bajas_normal,
                bajas_medio,
                bajas_fuerte,
            )
        {
            objetivo_completado =
                true;

            pantalla_great =
                true;

            detener_sonidos_enemigos(
                &sonidos,
            );

            mensaje =
                "OBJETIVO COMPLETADO"
                    .to_string();

            ventana.enable_cursor();

            continue;
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

            sonidos
                .detener_musica();

            nivel_actual =
                numero_mapa_inicial(
                    nivel_seleccionado,
                );

            enemigos_matados = 0;

            bajas_normal = 0;
            bajas_medio = 0;
            bajas_fuerte = 0;

            objetivo_completado = false;
            pantalla_great = false;

            numero_horda = 1;
            siguiente_horda = 4;

            tiempo_cambio_nivel = 0.0;

            mapa =
                Map::new(
                    nivel_actual,
                );

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
                nombre_nivel(
                    nivel_seleccionado,
                )
                    .to_string();

            sonidos
                .iniciar_musica();

            ventana
                .disable_cursor();
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

        let nombre =
            nombre_nivel(
                nivel_seleccionado,
            );

        let texto_arcade =
            if nivel_seleccionado
                == NivelSeleccionado::Mansion
            {
                format!(
                    "{} | PISO {} | HORDA {} | BAJAS {} | ENEMIGOS {}",
                    nombre,
                    nivel_actual,
                    numero_horda,
                    enemigos_matados,
                    vivos,
                )
            } else {
                format!(
                    "{} | HORDA {} | BAJAS {} | ENEMIGOS {}",
                    nombre,
                    numero_horda,
                    enemigos_matados,
                    vivos,
                )
            };

        dibujo.draw_text(
            &texto_arcade,
            (offset_x + 15.0) as i32,
            (offset_y + 115.0) as i32,
            18,
            Color::WHITE,
        );

        if !objetivo_completado {
            let texto_objetivo =
                format!(
                    "OBJETIVO | NORMAL {}/{} | MEDIO {}/{} | FUERTE {}/{}",
                    bajas_normal
                        .min(META_NORMAL),
                    META_NORMAL,
                    bajas_medio
                        .min(META_MEDIO),
                    META_MEDIO,
                    bajas_fuerte
                        .min(META_FUERTE),
                    META_FUERTE,
                );

            dibujo.draw_text(
                &texto_objetivo,
                (offset_x + 15.0) as i32,
                (offset_y + 140.0) as i32,
                18,
                Color::GOLD,
            );
        } else {
            dibujo.draw_text(
                "MODO ARCADE",
                (offset_x + 15.0) as i32,
                (offset_y + 140.0) as i32,
                18,
                Color::GOLD,
            );
        }

        damage_effect.render(
            &mut dibujo,
        );

        if vida_jugador <= 0 {
            sonidos.detener_musica();

            detener_sonidos_enemigos(
                &sonidos,
            );

            let sw =
                dibujo.get_screen_width();

            let sh =
                dibujo.get_screen_height();

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
                    "{} - HORDA {} - {} BAJAS",
                    nombre,
                    numero_horda,
                    enemigos_matados,
                );

            let objetivo =
                format!(
                    "N {}/{}   M {}/{}   F {}/{}",
                    bajas_normal
                        .min(META_NORMAL),
                    META_NORMAL,
                    bajas_medio
                        .min(META_MEDIO),
                    META_MEDIO,
                    bajas_fuerte
                        .min(META_FUERTE),
                    META_FUERTE,
                );

            let reiniciar =
                "F5 - JUGAR DE NUEVO";

            let menu_texto =
                "BACKSPACE - MENU PRINCIPAL";

            let titulo_w =
                dibujo.measure_text(
                    titulo,
                    64,
                );

            dibujo.draw_text(
                titulo,
                sw / 2
                    - titulo_w / 2,
                sh / 2
                    - 130,
                64,
                Color::RED,
            );

            let stats_w =
                dibujo.measure_text(
                    &stats,
                    26,
                );

            dibujo.draw_text(
                &stats,
                sw / 2
                    - stats_w / 2,
                sh / 2
                    - 35,
                26,
                Color::WHITE,
            );

            let objetivo_w =
                dibujo.measure_text(
                    &objetivo,
                    22,
                );

            dibujo.draw_text(
                &objetivo,
                sw / 2
                    - objetivo_w / 2,
                sh / 2
                    + 5,
                22,
                Color::GOLD,
            );

            let reiniciar_w =
                dibujo.measure_text(
                    reiniciar,
                    22,
                );

            dibujo.draw_text(
                reiniciar,
                sw / 2
                    - reiniciar_w / 2,
                sh / 2
                    + 60,
                22,
                Color::LIGHTGRAY,
            );

            let menu_w =
                dibujo.measure_text(
                    menu_texto,
                    20,
                );

            dibujo.draw_text(
                menu_texto,
                sw / 2
                    - menu_w / 2,
                sh / 2
                    + 95,
                20,
                Color::GRAY,
            );
        }
    }

    sonidos.detener_musica();

    detener_sonidos_enemigos(
        &sonidos,
    );
}