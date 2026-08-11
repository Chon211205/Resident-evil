use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::map::{Map, TAMANO_CELDA};
use crate::player::Player;

use raylib::prelude::*;
use std::f32::consts::PI;

pub const ANCHO_VENTANA: i32 = 800;
pub const ALTO_VENTANA: i32 = 600;

pub const CANTIDAD_RAYOS: i32 = ANCHO_VENTANA;
pub const FOV: f32 = PI / 3.0;

pub struct RayHit {
    pub distancia: f32,
    pub impacto_x: f32,
    pub impacto_y: f32,
}

pub fn render_3d(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
    textura_pared: &mut Image,
    textura_suelo: &mut Image,
) {
    dibujar_cielo(
        framebuffer,
        camera.vertical_offset,
    );

    dibujar_suelo_texturizado(
        framebuffer,
        player,
        camera,
        textura_suelo,
    );

    dibujar_paredes(
        framebuffer,
        mapa,
        player,
        camera,
        textura_pared,
    );
}

pub fn lanzar_rayo(
    mapa: &Map,
    inicio_x: f32,
    inicio_y: f32,
    angulo: f32,
) -> RayHit {
    let direccion_x =
        angulo.cos();

    let direccion_y =
        angulo.sin();

    let pos_mapa_x =
        inicio_x / TAMANO_CELDA;

    let pos_mapa_y =
        inicio_y / TAMANO_CELDA;

    let mut mapa_x =
        pos_mapa_x.floor() as i32;

    let mut mapa_y =
        pos_mapa_y.floor() as i32;

    let delta_dist_x =
        if direccion_x.abs() < 0.00001 {
            f32::INFINITY
        } else {
            (1.0 / direccion_x).abs()
        };

    let delta_dist_y =
        if direccion_y.abs() < 0.00001 {
            f32::INFINITY
        } else {
            (1.0 / direccion_y).abs()
        };

    let paso_x: i32;
    let paso_y: i32;

    let mut lado_dist_x: f32;
    let mut lado_dist_y: f32;

    if direccion_x < 0.0 {
        paso_x = -1;

        lado_dist_x =
            (pos_mapa_x
                - mapa_x as f32)
                * delta_dist_x;
    } else {
        paso_x = 1;

        lado_dist_x =
            (
                mapa_x as f32
                    + 1.0
                    - pos_mapa_x
            )
                * delta_dist_x;
    }

    if direccion_y < 0.0 {
        paso_y = -1;

        lado_dist_y =
            (pos_mapa_y
                - mapa_y as f32)
                * delta_dist_y;
    } else {
        paso_y = 1;

        lado_dist_y =
            (
                mapa_y as f32
                    + 1.0
                    - pos_mapa_y
            )
                * delta_dist_y;
    }

    let golpe_vertical: bool;

    loop {
        let lado_actual;

        if lado_dist_x < lado_dist_y {
            lado_dist_x +=
                delta_dist_x;

            mapa_x += paso_x;

            lado_actual = true;
        } else {
            lado_dist_y +=
                delta_dist_y;

            mapa_y += paso_y;

            lado_actual = false;
        }

        if mapa.celda(
            mapa_y,
            mapa_x,
        ) == '#'
        {
            golpe_vertical =
                lado_actual;

            break;
        }
    }

    let distancia_celdas =
        if golpe_vertical {
            lado_dist_x
                - delta_dist_x
        } else {
            lado_dist_y
                - delta_dist_y
        };

    let distancia =
        distancia_celdas
            * TAMANO_CELDA;

    let impacto_x =
        inicio_x
            + direccion_x
                * distancia;

    let impacto_y =
        inicio_y
            + direccion_y
                * distancia;

    RayHit {
        distancia,
        impacto_x,
        impacto_y,
    }
}

fn dibujar_paredes(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
    textura_pared: &mut Image,
) {
    let angulo_inicial =
        camera.angle
            - FOV / 2.0;

    let incremento_angulo =
        FOV
            / CANTIDAD_RAYOS as f32;

    let distancia_plano =
        (ANCHO_VENTANA as f32 / 2.0)
            / (FOV / 2.0).tan();

    for numero_rayo
        in 0..CANTIDAD_RAYOS
    {
        let angulo_rayo =
            angulo_inicial
                + numero_rayo as f32
                    * incremento_angulo;

        let hit =
            lanzar_rayo(
                mapa,
                player.x,
                player.y,
                angulo_rayo,
            );

        let diferencia_angulo =
            angulo_rayo
                - camera.angle;

        let distancia_corregida =
            hit.distancia
                * diferencia_angulo.cos();

        let distancia_segura =
            distancia_corregida
                .max(0.001);

        let altura_columna =
            TAMANO_CELDA
                * distancia_plano
                / distancia_segura;

        let altura_columna =
            altura_columna
                .min(
                    ALTO_VENTANA as f32
                        * 2.0,
                )
                as i32;

        let centro_pantalla =
            ALTO_VENTANA / 2
                + camera.vertical_offset;

        let inicio_y =
            centro_pantalla
                - altura_columna / 2;

        let final_y =
            centro_pantalla
                + altura_columna / 2;

        let textura_x =
            calcular_textura_x(
                hit.impacto_x,
                hit.impacto_y,
                textura_pared.width(),
            );

        dibujar_columna_texturizada(
            framebuffer,
            textura_pared,
            numero_rayo,
            inicio_y,
            final_y,
            textura_x,
            distancia_segura,
        );
    }
}

fn calcular_textura_x(
    impacto_x: f32,
    impacto_y: f32,
    ancho_textura: i32,
) -> i32 {
    let local_x =
        impacto_x
            .rem_euclid(
                TAMANO_CELDA,
            );

    let local_y =
        impacto_y
            .rem_euclid(
                TAMANO_CELDA,
            );

    let borde_x =
        local_x.min(
            TAMANO_CELDA
                - local_x,
        );

    let borde_y =
        local_y.min(
            TAMANO_CELDA
                - local_y,
        );

    let porcentaje =
        if borde_x < borde_y {
            local_y
                / TAMANO_CELDA
        } else {
            local_x
                / TAMANO_CELDA
        };

    (
        porcentaje
            * ancho_textura as f32
    )
        .floor()
        .clamp(
            0.0,
            (ancho_textura - 1)
                as f32,
        )
        as i32
}

fn dibujar_columna_texturizada(
    framebuffer: &mut Framebuffer,
    textura: &mut Image,
    pantalla_x: i32,
    inicio_y: i32,
    final_y: i32,
    textura_x: i32,
    distancia: f32,
) {
    let inicio =
        inicio_y.max(0);

    let final_posicion =
        final_y.min(
            framebuffer.height() - 1,
        );

    if inicio >
        final_posicion
    {
        return;
    }

    let altura_pared =
        final_y - inicio_y;

    if altura_pared <= 0 {
        return;
    }

    for pantalla_y
        in inicio..=final_posicion
    {
        let porcentaje_y =
            (
                pantalla_y
                    - inicio_y
            ) as f32
                / altura_pared as f32;

        let textura_y =
            (
                porcentaje_y
                    * textura.height()
                        as f32
            )
                .floor()
                .clamp(
                    0.0,
                    (textura.height() - 1)
                        as f32,
                )
                as i32;

        let color =
            textura.get_color(
                textura_x,
                textura_y,
            );

        let color =
            aplicar_oscuridad(
                color,
                distancia,
            );

        framebuffer
            .set_current_color(
                color,
            );

        framebuffer.point(
            pantalla_x,
            pantalla_y,
        );
    }
}

fn dibujar_suelo_texturizado(
    framebuffer: &mut Framebuffer,
    player: &Player,
    camera: &Camera,
    textura_suelo: &mut Image,
) {
    let horizonte =
        ALTO_VENTANA / 2
            + camera.vertical_offset;

    let direccion_x =
        camera.angle.cos();

    let direccion_y =
        camera.angle.sin();

    let plano_x =
        -camera.angle.sin()
            * (FOV / 2.0).tan();

    let plano_y =
        camera.angle.cos()
            * (FOV / 2.0).tan();

    let rayo_izquierda_x =
        direccion_x
            - plano_x;

    let rayo_izquierda_y =
        direccion_y
            - plano_y;

    let rayo_derecha_x =
        direccion_x
            + plano_x;

    let rayo_derecha_y =
        direccion_y
            + plano_y;

    let altura_camara =
        TAMANO_CELDA * 0.5;

    let inicio_y =
        horizonte.max(0);

    for y
        in inicio_y..ALTO_VENTANA
    {
        let distancia_vertical =
            y as f32
                - horizonte as f32;

        if distancia_vertical
            <= 0.0
        {
            continue;
        }

        let distancia_fila =
            altura_camara
                * ALTO_VENTANA as f32
                / distancia_vertical;

        let paso_x =
            distancia_fila
                * (
                    rayo_derecha_x
                        - rayo_izquierda_x
                )
                / ANCHO_VENTANA as f32;

        let paso_y =
            distancia_fila
                * (
                    rayo_derecha_y
                        - rayo_izquierda_y
                )
                / ANCHO_VENTANA as f32;

        let mut mundo_x =
            player.x
                + distancia_fila
                    * rayo_izquierda_x;

        let mut mundo_y =
            player.y
                + distancia_fila
                    * rayo_izquierda_y;

        for x in (0..ANCHO_VENTANA).step_by(2) {
            let textura_x =
                (
                    mundo_x
                        .rem_euclid(
                            TAMANO_CELDA,
                        )
                        / TAMANO_CELDA
                        * textura_suelo
                            .width() as f32
                )
                    .floor()
                    .clamp(
                        0.0,
                        (
                            textura_suelo
                                .width()
                                - 1
                        ) as f32,
                    )
                    as i32;

            let textura_y =
                (
                    mundo_y
                        .rem_euclid(
                            TAMANO_CELDA,
                        )
                        / TAMANO_CELDA
                        * textura_suelo
                            .height() as f32
                )
                    .floor()
                    .clamp(
                        0.0,
                        (
                            textura_suelo
                                .height()
                                - 1
                        ) as f32,
                    )
                    as i32;

            let color =
                textura_suelo
                    .get_color(
                        textura_x,
                        textura_y,
                    );

            let color =
                aplicar_oscuridad(
                    color,
                    distancia_fila,
                );

            framebuffer
                .set_current_color(
                    color,
                );

            framebuffer.point(
                x,
                y,
            );

            if x + 1 < ANCHO_VENTANA {
                framebuffer.point(
                    x + 1,
                    y,
                );
            }

            mundo_x += paso_x * 2.0;
            mundo_y += paso_y * 2.0;
        }
    }
}


fn aplicar_oscuridad(
    color: Color,
    distancia: f32,
) -> Color {
    let factor =
        (
            1.0
                - distancia
                    / 700.0
        )
            .clamp(
                0.25,
                1.0,
            );

    Color::new(
        (
            color.r as f32
                * factor
        ) as u8,
        (
            color.g as f32
                * factor
        ) as u8,
        (
            color.b as f32
                * factor
        ) as u8,
        color.a,
    )
}

fn dibujar_cielo(
    framebuffer: &mut Framebuffer,
    altura_camara: i32,
) {
    let horizonte =
        (
            ALTO_VENTANA / 2
                + altura_camara
        )
            .clamp(
                0,
                ALTO_VENTANA,
            );

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
}

fn dibujar_rectangulo(
    framebuffer: &mut Framebuffer,
    x: i32,
    y: i32,
    ancho: i32,
    alto: i32,
) {
    if ancho <= 0
        || alto <= 0
    {
        return;
    }

    for pixel_y
        in y..y + alto
    {
        for pixel_x
            in x..x + ancho
        {
            framebuffer.point(
                pixel_x,
                pixel_y,
            );
        }
    }
}