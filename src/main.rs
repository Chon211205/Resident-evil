mod framebuffer;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::f32::consts::PI;
use std::fs;

const ANCHO_VENTANA: i32 = 800;
const ALTO_VENTANA: i32 = 600;

const TAMANO_CELDA: f32 = 25.0;

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
    let mapa = [

    "#########################################",
    "#P       #             #               #",
    "#        #             #               #",
    "#   1    #      2      #       3       #",
    "#        #             #               #",
    "#### ########## ############### ########",
    "#              #               #       #",
    "#              #               #       #",
    "#              #               #   5   #",
    "#              #               #       #",
    "####### ########## ########## ##########",
    "#     #           #          #         #",
    "#  4  #           #          #    6    #",
    "#     #           #          #    S    #",
    "### ######## ###### ########## #########",
    "#           #                         #",
    "#           #                         #",
    "#     7     #           8             #",
    "#           #                         #",
    "###### ############## #################",
    "#             #                       #",
    "#             #                       #",
    "#      9      #          L            #",
    "#       I     #                     A #",
    "###### ########### ####################",
    "#               #                     #",
    "#               #                     #",
    "#               #          Z          #",
    "#       K       #                     #",
    "###### ########### ####################",
    "#             #                       #",
    "#             #                       #",
    "#      G      #          C            #",
    "#      H      #                 Z     #",
    "###### ############# ##################",
    "#                   #                 #",
    "#                   #                 #",
    "#                   #        B        #",
    "#          Z        #        B       E#",
    "#########################################",
    ];

    fs::write(
        "mapa_resident.txt",
        mapa.join("\n"),
    )
    .expect("No se pudo crear mapa_resident.txt");

    let (fila_inicial, columna_inicial) =
        buscar_jugador(&mapa)
            .expect("No se encontró el jugador P");

    let posicion_inicial_x =
        columna_inicial as f32 * TAMANO_CELDA
            + TAMANO_CELDA / 2.0;

    let posicion_inicial_y =
        fila_inicial as f32 * TAMANO_CELDA
            + TAMANO_CELDA / 2.0;

    let mut jugador_x = posicion_inicial_x;
    let mut jugador_y = posicion_inicial_y;

    let mut angulo_jugador = 0.0_f32;

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

        procesar_eventos(
            &ventana,
            &mapa,
            &mut jugador_x,
            &mut jugador_y,
            &mut angulo_jugador,
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
                );

                dibujar_vista_3d(
                    &mut framebuffer,
                    &mapa,
                    jugador_x,
                    jugador_y,
                    angulo_jugador,
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
                angulo_jugador
                    .to_degrees(),
            ),
            10,
            ALTO_VENTANA + 5,
            18,
            Color::WHITE,
        );

        dibujo.draw_text(
            "W/S: mover | A/D: girar | M: mapa | R: reiniciar",
            10,
            ALTO_VENTANA + 30,
            15,
            Color::LIGHTGRAY,
        );
    }
}

fn procesar_eventos(
    ventana: &RaylibHandle,
    mapa: &[&str],
    jugador_x: &mut f32,
    jugador_y: &mut f32,
    angulo: &mut f32,
    vista: &mut Vista,
    inicio_x: f32,
    inicio_y: f32,
    delta_time: f32,
) {
    if ventana.is_key_down(
        KeyboardKey::KEY_A,
    ) || ventana.is_key_down(
        KeyboardKey::KEY_LEFT,
    ) {
        *angulo -=
            VELOCIDAD_ROTACION
                * delta_time;
    }

    if ventana.is_key_down(
        KeyboardKey::KEY_D,
    ) || ventana.is_key_down(
        KeyboardKey::KEY_RIGHT,
    ) {
        *angulo +=
            VELOCIDAD_ROTACION
                * delta_time;
    }

    *angulo =
        normalizar_angulo(*angulo);

    let mut movimiento = 0.0;

    if ventana.is_key_down(
        KeyboardKey::KEY_W,
    ) || ventana.is_key_down(
        KeyboardKey::KEY_UP,
    ) {
        movimiento +=
            VELOCIDAD_MOVIMIENTO
                * delta_time;
    }

    if ventana.is_key_down(
        KeyboardKey::KEY_S,
    ) || ventana.is_key_down(
        KeyboardKey::KEY_DOWN,
    ) {
        movimiento -=
            VELOCIDAD_MOVIMIENTO
                * delta_time;
    }

    if movimiento != 0.0 {
        mover_jugador(
            mapa,
            jugador_x,
            jugador_y,
            *angulo,
            movimiento,
        );
    }

    if ventana.is_key_pressed(
        KeyboardKey::KEY_M,
    ) {
        *vista = match *vista {
            Vista::Vista3D =>
                Vista::Mapa2D,

            Vista::Mapa2D =>
                Vista::Vista3D,
        };
    }

    if ventana.is_key_pressed(
        KeyboardKey::KEY_R,
    ) {
        *jugador_x = inicio_x;
        *jugador_y = inicio_y;
        *angulo = 0.0;
    }
}

fn mover_jugador(
    mapa: &[&str],
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

    if !es_pared(
        mapa,
        nuevo_x,
        *jugador_y,
    ) {
        *jugador_x = nuevo_x;
    }

    if !es_pared(
        mapa,
        *jugador_x,
        nuevo_y,
    ) {
        *jugador_y = nuevo_y;
    }
}

fn es_pared(
    mapa: &[&str],
    x: f32,
    y: f32,
) -> bool {
    let columna =
        (x / TAMANO_CELDA)
            .floor() as i32;

    let fila =
        (y / TAMANO_CELDA)
            .floor() as i32;

    if fila < 0
        || fila >= mapa.len() as i32
    {
        return true;
    }

    let linea =
        mapa[fila as usize];

    if columna < 0
        || columna
            >= linea.chars().count()
                as i32
    {
        return true;
    }

    let celda = linea
        .chars()
        .nth(columna as usize)
        .unwrap_or('#');

    celda == '#'
}

fn dibujar_vista_3d(
    framebuffer: &mut Framebuffer,
    mapa: &[&str],
    jugador_x: f32,
    jugador_y: f32,
    angulo_jugador: f32,
) {
    let angulo_inicial =
        angulo_jugador
            - FOV / 2.0;

    let incremento =
        FOV / CANTIDAD_RAYOS
            as f32;

    let distancia_plano =
        (ANCHO_VENTANA as f32
            / 2.0)
            / (FOV / 2.0).tan();

    for rayo in 0..CANTIDAD_RAYOS {
        let angulo_rayo =
            angulo_inicial
                + rayo as f32
                    * incremento;

        let distancia =
            lanzar_rayo(
                mapa,
                jugador_x,
                jugador_y,
                angulo_rayo,
            );

        let diferencia =
            angulo_rayo
                - angulo_jugador;

        let distancia_corregida =
            distancia
                * diferencia.cos();

        let distancia_segura =
            distancia_corregida
                .max(1.0);

        let altura =
            TAMANO_CELDA
                * distancia_plano
                / distancia_segura;

        let altura =
            altura
                .min(
                    ALTO_VENTANA
                        as f32,
                ) as i32;

        let mitad =
            ALTO_VENTANA / 2;

        let inicio_y =
            mitad - altura / 2;

        let final_y =
            mitad + altura / 2;

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
            rayo,
            inicio_y,
            final_y,
        );
    }
}

fn lanzar_rayo(
    mapa: &[&str],
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

        if es_pared(
            mapa,
            rayo_x,
            rayo_y,
        ) {
            return distancia;
        }
    }
}

fn dibujar_mapa_2d(
    framebuffer: &mut Framebuffer,
    mapa: &[&str],
    jugador_x: f32,
    jugador_y: f32,
    angulo: f32,
) {
    let escala =
        calcular_escala_mapa(mapa);

    let columnas =
        mapa
            .iter()
            .map(
                |linea|
                    linea
                        .chars()
                        .count(),
            )
            .max()
            .unwrap_or(1)
            as f32;

    let filas =
        mapa.len() as f32;

    let ancho_mapa =
        columnas * escala;

    let alto_mapa =
        filas * escala;

    let offset_x =
        (ANCHO_VENTANA as f32
            - ancho_mapa)
            / 2.0;

    let offset_y =
        (ALTO_VENTANA as f32
            - alto_mapa)
            / 2.0;

    for (fila, linea) in
        mapa.iter().enumerate()
    {
        for (columna, celda) in
            linea.chars().enumerate()
        {
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

                'Z' => {
                    framebuffer
                        .set_current_color(
                            Color::GREEN,
                        );

                    framebuffer
                        .point_with_size(
                            x + escala as i32
                                / 2,
                            y + escala as i32
                                / 2,
                            5,
                        );
                }

                'I' => {
                    framebuffer
                        .set_current_color(
                            Color::GOLD,
                        );

                    framebuffer
                        .point_with_size(
                            x + escala as i32
                                / 2,
                            y + escala as i32
                                / 2,
                            3,
                        );
                }

                'H' => {
                    framebuffer
                        .set_current_color(
                            Color::LIME,
                        );

                    dibujar_rectangulo(
                        framebuffer,
                        x + escala as i32
                            / 3,
                        y + escala as i32
                            / 3,
                        escala as i32
                            / 3,
                        escala as i32
                            / 3,
                    );
                }

                'A' => {
                    framebuffer
                        .set_current_color(
                            Color::ORANGE,
                        );

                    framebuffer
                        .point_with_size(
                            x + escala as i32
                                / 2,
                            y + escala as i32
                                / 2,
                            3,
                        );
                }

                'K' => {
                    framebuffer
                        .set_current_color(
                            Color::YELLOW,
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

                'S' => {
                    framebuffer
                        .set_current_color(
                            Color::SKYBLUE,
                        );

                    dibujar_rectangulo(
                        framebuffer,
                        x + 3,
                        y + 3,
                        escala as i32 - 6,
                        escala as i32 - 6,
                    );
                }

                'B' => {
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
                            7,
                        );
                }

                'E' => {
                    framebuffer
                        .set_current_color(
                            Color::PURPLE,
                        );

                    dibujar_rectangulo(
                        framebuffer,
                        x + 2,
                        y + 2,
                        escala as i32 - 4,
                        escala as i32 - 4,
                    );
                }

                _ => {}
            }
        }
    }

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

    // Rayos del FOV
    let cantidad_rayitos = 40;

    let angulo_inicial =
        angulo - FOV / 2.0;

    for i in 0..cantidad_rayitos {
        let angulo_rayo =
            angulo_inicial
                + FOV
                    * i as f32
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

    // Jugador
    framebuffer
        .set_current_color(
            Color::YELLOW,
        );

    framebuffer.point_with_size(
        jugador_mapa_x as i32,
        jugador_mapa_y as i32,
        6,
    );

    // Dirección
    let direccion_x =
        jugador_mapa_x
            + angulo.cos() * 20.0;

    let direccion_y =
        jugador_mapa_y
            + angulo.sin() * 20.0;

    framebuffer
        .set_current_color(
            Color::WHITE,
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
    mapa: &[&str],
) -> f32 {
    let columnas =
        mapa
            .iter()
            .map(
                |linea|
                    linea
                        .chars()
                        .count(),
            )
            .max()
            .unwrap_or(1)
            as f32;

    let filas =
        mapa.len() as f32;

    let escala_x =
        ANCHO_VENTANA as f32
            / columnas;

    let escala_y =
        ALTO_VENTANA as f32
            / filas;

    escala_x.min(escala_y)
        * 0.9
}

fn dibujar_fondo_3d(
    framebuffer: &mut Framebuffer,
) {
    // Techo oscuro
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
        ALTO_VENTANA / 2,
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
        ALTO_VENTANA / 2,
        ANCHO_VENTANA,
        ALTO_VENTANA / 2,
    );
}

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

    for y in inicio..=final_posicion {
        framebuffer.point(x, y);
    }
}

fn dibujar_rectangulo(
    framebuffer: &mut Framebuffer,
    x: i32,
    y: i32,
    ancho: i32,
    alto: i32,
) {
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
        .clamp(35.0, 210.0)
        as u8
}

fn buscar_jugador(
    mapa: &[&str],
) -> Option<(usize, usize)> {
    for (fila, linea) in
        mapa.iter().enumerate()
    {
        for (columna, celda) in
            linea.chars().enumerate()
        {
            if celda == 'P' {
                return Some((
                    fila,
                    columna,
                ));
            }
        }
    }

    None
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