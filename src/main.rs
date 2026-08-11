mod framebuffer;
mod map;

use framebuffer::Framebuffer;
use map::{Map, TAMANO_CELDA};
use raylib::prelude::*;
use std::f32::consts::PI;

const ANCHO_VENTANA: i32 = 800;
const ALTO_VENTANA: i32 = 600;

const CANTIDAD_RAYOS: i32 = ANCHO_VENTANA;
const FOV: f32 = PI / 3.0;

const VELOCIDAD_ROTACION: f32 = 2.0;
const VELOCIDAD_MOVIMIENTO: f32 = 100.0;

#[derive(Clone, Copy, PartialEq)]
enum Vista {
    Mapa2D,
    Vista3D,
}

fn main() {
    // =========================
    // MAPA
    // =========================

    let mapa = Map::new();

    mapa.guardar_txt("mapa_resident.txt");

    let (fila_inicial, columna_inicial) =
        mapa.buscar_jugador()
            .expect("No se encontró el jugador P");

    let posicion_inicial_x =
        columna_inicial as f32 * TAMANO_CELDA
            + TAMANO_CELDA / 2.0;

    let posicion_inicial_y =
        fila_inicial as f32 * TAMANO_CELDA
            + TAMANO_CELDA / 2.0;

    // =========================
    // JUGADOR
    // =========================

    let mut jugador_x = posicion_inicial_x;
    let mut jugador_y = posicion_inicial_y;

    let mut angulo_jugador = 0.0_f32;

    // Después lo usaremos con I y K.
    let mut altura_camara = 0_i32;

    let mut vista_actual = Vista::Vista3D;

    // =========================
    // FRAMEBUFFER
    // =========================

    let mut framebuffer =
        Framebuffer::new(
            ANCHO_VENTANA,
            ALTO_VENTANA,
        );

    framebuffer.set_background_color(
        Color::BLACK,
    );

    // =========================
    // VENTANA
    // =========================

    let (mut ventana, thread) =
        raylib::init()
            .size(
                ANCHO_VENTANA,
                ALTO_VENTANA + 55,
            )
            .title("Survival Horror Raycasting")
            .build();

    ventana.set_target_fps(60);

    // =========================
    // GAME LOOP
    // =========================

    while !ventana.window_should_close() {
        let delta_time =
            ventana.get_frame_time();

        procesar_eventos(
            &ventana,
            &mapa,
            &mut jugador_x,
            &mut jugador_y,
            &mut angulo_jugador,
            &mut altura_camara,
            &mut vista_actual,
            posicion_inicial_x,
            posicion_inicial_y,
            delta_time,
        );

        framebuffer.clear();

        match vista_actual {
            Vista::Vista3D => {
                dibujar_fondo_3d(
                    &mut framebuffer,
                    altura_camara,
                );

                dibujar_vista_3d(
                    &mut framebuffer,
                    &mapa,
                    jugador_x,
                    jugador_y,
                    angulo_jugador,
                    altura_camara,
                );
            }

            Vista::Mapa2D => {
                dibujar_mapa_2d(
                    &mut framebuffer,
                    &mapa,
                    jugador_x,
                    jugador_y,
                    angulo_jugador,
                );
            }
        }

        let textura = ventana
            .load_texture_from_image(
                &thread,
                framebuffer.image(),
            )
            .expect(
                "No se pudo crear la textura",
            );

        let mut dibujo =
            ventana.begin_drawing(&thread);

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
                angulo_jugador.to_degrees(),
            ),
            10,
            ALTO_VENTANA + 5,
            18,
            Color::WHITE,
        );

        dibujo.draw_text(
            "WASD: mover | J/L: girar camara | I/K: arriba/abajo | M: mapa",
            10,
            ALTO_VENTANA + 30,
            14,
            Color::LIGHTGRAY,
        );
    }
}

// ============================================================
// EVENTOS
// ============================================================

fn procesar_eventos(
    ventana: &RaylibHandle,
    mapa: &Map,
    jugador_x: &mut f32,
    jugador_y: &mut f32,
    angulo: &mut f32,
    altura_camara: &mut i32,
    vista: &mut Vista,
    inicio_x: f32,
    inicio_y: f32,
    delta_time: f32,
) {
    // =========================
    // CÁMARA - J I K L
    // =========================

    if ventana.is_key_down(
        KeyboardKey::KEY_J,
    ) {
        *angulo -=
            VELOCIDAD_ROTACION
                * delta_time;
    }

    if ventana.is_key_down(
        KeyboardKey::KEY_L,
    ) {
        *angulo +=
            VELOCIDAD_ROTACION
                * delta_time;
    }

    // Mirar hacia arriba
    if ventana.is_key_down(
        KeyboardKey::KEY_I,
    ) {
        *altura_camara -= 3;
    }

    // Mirar hacia abajo
    if ventana.is_key_down(
        KeyboardKey::KEY_K,
    ) {
        *altura_camara += 3;
    }

    *altura_camara =
        (*altura_camara).clamp(
            -150,
            150,
        );

    *angulo =
        normalizar_angulo(*angulo);

    // =========================
    // MOVIMIENTO - WASD
    // =========================

    let velocidad =
        VELOCIDAD_MOVIMIENTO
            * delta_time;

    // W = adelante
    if ventana.is_key_down(
        KeyboardKey::KEY_W,
    ) {
        mover_jugador(
            mapa,
            jugador_x,
            jugador_y,
            *angulo,
            velocidad,
        );
    }

    // S = atrás
    if ventana.is_key_down(
        KeyboardKey::KEY_S,
    ) {
        mover_jugador(
            mapa,
            jugador_x,
            jugador_y,
            *angulo,
            -velocidad,
        );
    }

    // A = izquierda
    if ventana.is_key_down(
        KeyboardKey::KEY_A,
    ) {
        mover_jugador(
            mapa,
            jugador_x,
            jugador_y,
            *angulo - PI / 2.0,
            velocidad,
        );
    }

    // D = derecha
    if ventana.is_key_down(
        KeyboardKey::KEY_D,
    ) {
        mover_jugador(
            mapa,
            jugador_x,
            jugador_y,
            *angulo + PI / 2.0,
            velocidad,
        );
    }

    // M = cambiar vista
    if ventana.is_key_pressed(
        KeyboardKey::KEY_M,
    ) {
        *vista =
            match *vista {
                Vista::Vista3D =>
                    Vista::Mapa2D,

                Vista::Mapa2D =>
                    Vista::Vista3D,
            };
    }

    // R = reiniciar
    if ventana.is_key_pressed(
        KeyboardKey::KEY_R,
    ) {
        *jugador_x = inicio_x;
        *jugador_y = inicio_y;

        *angulo = 0.0;
        *altura_camara = 0;
    }
}

// ============================================================
// MOVIMIENTO
// ============================================================

fn mover_jugador(
    mapa: &Map,
    jugador_x: &mut f32,
    jugador_y: &mut f32,
    angulo: f32,
    movimiento: f32,
) {
    let nuevo_x =
        *jugador_x
            + angulo.cos()
                * movimiento;

    let nuevo_y =
        *jugador_y
            + angulo.sin()
                * movimiento;

    // X y Y separados para deslizarse
    // por las paredes.
    if !mapa.es_pared(
        nuevo_x,
        *jugador_y,
    ) {
        *jugador_x = nuevo_x;
    }

    if !mapa.es_pared(
        *jugador_x,
        nuevo_y,
    ) {
        *jugador_y = nuevo_y;
    }
}

// ============================================================
// RAYCASTING
// ============================================================

fn dibujar_vista_3d(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    jugador_x: f32,
    jugador_y: f32,
    angulo_jugador: f32,
    altura_camara: i32,
) {
    let angulo_inicial =
        angulo_jugador
            - FOV / 2.0;

    let incremento_angulo =
        FOV
            / CANTIDAD_RAYOS
                as f32;

    let distancia_plano =
        (ANCHO_VENTANA as f32
            / 2.0)
            / (FOV / 2.0).tan();

    for numero_rayo
        in 0..CANTIDAD_RAYOS
    {
        let angulo_rayo =
            angulo_inicial
                + numero_rayo as f32
                    * incremento_angulo;

        let distancia =
            lanzar_rayo(
                mapa,
                jugador_x,
                jugador_y,
                angulo_rayo,
            );

        // Corrección del ojo de pez.
        let diferencia =
            angulo_rayo
                - angulo_jugador;

        let distancia_corregida =
            distancia
                * diferencia.cos();

        let distancia_segura =
            distancia_corregida
                .max(1.0);

        // Altura de pared.
        let altura_columna =
            TAMANO_CELDA
                * distancia_plano
                / distancia_segura;

        let altura_columna =
            altura_columna
                .min(
                    ALTO_VENTANA
                        as f32
                        * 2.0,
                )
                as i32;

        // Centro de la cámara.
        let mitad_pantalla =
            ALTO_VENTANA / 2
                + altura_camara;

        // Mitad para arriba y mitad
        // para abajo.
        let inicio_y =
            mitad_pantalla
                - altura_columna / 2;

        let final_y =
            mitad_pantalla
                + altura_columna / 2;

        let intensidad =
            calcular_intensidad(
                distancia_segura,
            );

        framebuffer
            .set_current_color(
                Color::new(
                    intensidad,
                    intensidad,
                    intensidad,
                    255,
                ),
            );

        dibujar_columna(
            framebuffer,
            numero_rayo,
            inicio_y,
            final_y,
        );
    }
}

fn lanzar_rayo(
    mapa: &Map,
    jugador_x: f32,
    jugador_y: f32,
    angulo: f32,
) -> f32 {
    let direccion_x =
        angulo.cos();

    let direccion_y =
        angulo.sin();

    let mut distancia = 0.0;

    loop {
        distancia += 0.5;

        let rayo_x =
            jugador_x
                + direccion_x
                    * distancia;

        let rayo_y =
            jugador_y
                + direccion_y
                    * distancia;

        if mapa.es_pared(
            rayo_x,
            rayo_y,
        ) {
            return distancia;
        }
    }
}

// ============================================================
// MAPA 2D
// ============================================================

fn dibujar_mapa_2d(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    jugador_x: f32,
    jugador_y: f32,
    angulo_jugador: f32,
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

    // Dibujar celdas.
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

    // =========================
    // POSICIÓN DEL JUGADOR
    // =========================

    let jugador_mapa_x =
        offset_x
            + jugador_x
                / TAMANO_CELDA
                * escala;

    let jugador_mapa_y =
        offset_y
            + jugador_y
                / TAMANO_CELDA
                * escala;

    // =========================
    // RAYOS DEL FOV
    // =========================

    let cantidad_rayitos = 40;

    let angulo_inicial =
        angulo_jugador
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
                jugador_x,
                jugador_y,
                angulo_rayo,
            );

        let choque_x =
            jugador_x
                + angulo_rayo.cos()
                    * distancia;

        let choque_y =
            jugador_y
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
            jugador_mapa_x as i32,
            jugador_mapa_y as i32,
            choque_mapa_x as i32,
            choque_mapa_y as i32,
            7.0,
        );
    }

    // =========================
    // JUGADOR
    // =========================

    framebuffer
        .set_current_color(
            Color::YELLOW,
        );

    framebuffer.point_with_size(
        jugador_mapa_x as i32,
        jugador_mapa_y as i32,
        6,
    );

    // Dirección de la cámara.
    let direccion_x =
        jugador_mapa_x
            + angulo_jugador.cos()
                * 20.0;

    let direccion_y =
        jugador_mapa_y
            + angulo_jugador.sin()
                * 20.0;

    framebuffer
        .set_current_color(
            Color::GREEN,
        );

    framebuffer.dotted_line(
        jugador_mapa_x as i32,
        jugador_mapa_y as i32,
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
            / mapa.ancho()
                as f32;

    let escala_y =
        ALTO_VENTANA as f32
            / mapa.alto()
                as f32;

    escala_x.min(escala_y)
        * 0.9
}

// ============================================================
// FONDO 3D
// ============================================================

fn dibujar_fondo_3d(
    framebuffer: &mut Framebuffer,
    altura_camara: i32,
) {
    let horizonte =
        (ALTO_VENTANA / 2
            + altura_camara)
            .clamp(
                0,
                ALTO_VENTANA,
            );

    // Techo
    framebuffer
        .set_current_color(
            Color::new(
                10,
                10,
                15,
                255,
            ),
        );

    dibujar_rectangulo(
        framebuffer,
        0,
        0,
        ANCHO_VENTANA,
        horizonte,
    );

    // Suelo
    framebuffer
        .set_current_color(
            Color::new(
                35,
                35,
                35,
                255,
            ),
        );

    dibujar_rectangulo(
        framebuffer,
        0,
        horizonte,
        ANCHO_VENTANA,
        ALTO_VENTANA - horizonte,
    );
}

// ============================================================
// DIBUJADO
// ============================================================

fn dibujar_columna(
    framebuffer: &mut Framebuffer,
    x: i32,
    inicio_y: i32,
    final_y: i32,
) {
    let inicio =
        inicio_y.max(0);

    let final_posicion =
        final_y.min(
            framebuffer.height() - 1,
        );

    if inicio > final_posicion {
        return;
    }

    for y in inicio..=final_posicion {
        framebuffer.point(
            x,
            y,
        );
    }
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

fn calcular_intensidad(
    distancia: f32,
) -> u8 {
    let intensidad =
        210.0
            - distancia * 0.55;

    intensidad
        .clamp(
            35.0,
            210.0,
        )
        as u8
}

fn normalizar_angulo(
    mut angulo: f32,
) -> f32 {
    let vuelta =
        2.0 * PI;

    while angulo < 0.0 {
        angulo += vuelta;
    }

    while angulo >= vuelta {
        angulo -= vuelta;
    }

    angulo
}