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

pub fn render_3d(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
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
    );
}

pub fn lanzar_rayo(
    mapa: &Map,
    inicio_x: f32,
    inicio_y: f32,
    angulo: f32,
) -> f32 {
    let direccion_x = angulo.cos();
    let direccion_y = angulo.sin();

    let mut distancia = 0.0;

    loop {
        distancia += 0.5;

        let rayo_x =
            inicio_x + direccion_x * distancia;

        let rayo_y =
            inicio_y + direccion_y * distancia;

        if mapa.es_pared(
            rayo_x,
            rayo_y,
        ) {
            return distancia;
        }
    }
}

fn dibujar_paredes(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
) {
    let angulo_inicial =
        camera.angle - FOV / 2.0;

    let incremento_angulo =
        FOV / CANTIDAD_RAYOS as f32;

    let distancia_plano =
        (ANCHO_VENTANA as f32 / 2.0)
            / (FOV / 2.0).tan();

    for numero_rayo in 0..CANTIDAD_RAYOS {
        let angulo_rayo =
            angulo_inicial
                + numero_rayo as f32
                    * incremento_angulo;

        let distancia =
            lanzar_rayo(
                mapa,
                player.x,
                player.y,
                angulo_rayo,
            );

        let diferencia_angulo =
            angulo_rayo - camera.angle;

        let distancia_corregida =
            distancia
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

        let intensidad =
            calcular_intensidad(
                distancia_segura,
            );

        framebuffer.set_current_color(
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