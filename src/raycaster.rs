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
) {
    dibujar_fondo(
        framebuffer,
        camera.vertical_offset,
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

    let mut distancia = 0.0;

    loop {
        distancia += 0.5;

        let rayo_x =
            inicio_x
                + direccion_x
                    * distancia;

        let rayo_y =
            inicio_y
                + direccion_y
                    * distancia;

        if mapa.es_pared(
            rayo_x,
            rayo_y,
        ) {
            return RayHit {
                distancia,
                impacto_x: rayo_x,
                impacto_y: rayo_y,
            };
        }
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

    for numero_rayo in 0..CANTIDAD_RAYOS {
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
            distancia_corregida.max(1.0);

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

    let distancia_borde_x =
        local_x.min(
            TAMANO_CELDA - local_x,
        );

    let distancia_borde_y =
        local_y.min(
            TAMANO_CELDA - local_y,
        );

    let porcentaje =
        if distancia_borde_x
            < distancia_borde_y
        {
            local_y / TAMANO_CELDA
        } else {
            local_x / TAMANO_CELDA
        };

    let textura_x =
        porcentaje
            * ancho_textura as f32;

    textura_x
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

    if inicio > final_posicion {
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
            (pantalla_y - inicio_y)
                as f32
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

        let mut color =
            textura.get_color(
                textura_x,
                textura_y,
            );

        color =
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

fn aplicar_oscuridad(
    color: Color,
    distancia: f32,
) -> Color {
    let factor =
        (1.0
            - distancia / 700.0)
            .clamp(
                0.25,
                1.0,
            );

    Color::new(
        (color.r as f32 * factor)
            as u8,
        (color.g as f32 * factor)
            as u8,
        (color.b as f32 * factor)
            as u8,
        color.a,
    )
}

fn dibujar_fondo(
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

    framebuffer.set_current_color(
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

    framebuffer.set_current_color(
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