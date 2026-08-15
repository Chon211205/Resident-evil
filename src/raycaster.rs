use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::map::{
    Map,
    TAMANO_CELDA,
};
use crate::player::Player;
use crate::texture_data::TextureData;

use raylib::prelude::*;

use std::f32::consts::PI;

pub const ANCHO_VENTANA: i32 =
    800;

pub const ALTO_VENTANA: i32 =
    600;

pub const FOV: f32 =
    PI / 3.0;

pub struct RayHit {
    pub distancia: f32,
    pub offset_textura: f32,
    pub golpe_vertical: bool,
    pub tipo: char,
}

pub fn lanzar_rayo(
    mapa: &Map,
    origen_x: f32,
    origen_y: f32,
    angulo: f32,
) -> RayHit {
    let dir_x =
        angulo.cos();

    let dir_y =
        angulo.sin();

    let posicion_mapa_x =
        origen_x / TAMANO_CELDA;

    let posicion_mapa_y =
        origen_y / TAMANO_CELDA;

    let mut mapa_x =
        posicion_mapa_x
            .floor() as i32;

    let mut mapa_y =
        posicion_mapa_y
            .floor() as i32;

    let delta_dist_x =
        if dir_x.abs()
            < 0.000001
        {
            f32::INFINITY
        } else {
            (1.0 / dir_x)
                .abs()
        };

    let delta_dist_y =
        if dir_y.abs()
            < 0.000001
        {
            f32::INFINITY
        } else {
            (1.0 / dir_y)
                .abs()
        };

    let paso_x: i32;

    let paso_y: i32;

    let mut distancia_lado_x: f32;

    let mut distancia_lado_y: f32;

    if dir_x < 0.0 {
        paso_x =
            -1;

        distancia_lado_x =
            (
                posicion_mapa_x
                    - mapa_x as f32
            )
                * delta_dist_x;
    } else {
        paso_x =
            1;

        distancia_lado_x =
            (
                mapa_x as f32
                    + 1.0
                    - posicion_mapa_x
            )
                * delta_dist_x;
    }

    if dir_y < 0.0 {
        paso_y =
            -1;

        distancia_lado_y =
            (
                posicion_mapa_y
                    - mapa_y as f32
            )
                * delta_dist_y;
    } else {
        paso_y =
            1;

        distancia_lado_y =
            (
                mapa_y as f32
                    + 1.0
                    - posicion_mapa_y
            )
                * delta_dist_y;
    }

    let mut golpe_vertical;

    loop {
        if distancia_lado_x
            < distancia_lado_y
        {
            distancia_lado_x +=
                delta_dist_x;

            mapa_x +=
                paso_x;

            golpe_vertical =
                true;
        } else {
            distancia_lado_y +=
                delta_dist_y;

            mapa_y +=
                paso_y;

            golpe_vertical =
                false;
        }

        if mapa.es_bloque_solido(
            mapa_y,
            mapa_x,
        ) {
            break;
        }
    }

    let distancia_celdas =
        if golpe_vertical {
            if dir_x.abs()
                < 0.000001
            {
                0.0
            } else {
                (
                    mapa_x as f32
                        - posicion_mapa_x
                        + (
                            1 - paso_x
                        ) as f32
                            / 2.0
                ) / dir_x
            }
        } else {
            if dir_y.abs()
                < 0.000001
            {
                0.0
            } else {
                (
                    mapa_y as f32
                        - posicion_mapa_y
                        + (
                            1 - paso_y
                        ) as f32
                            / 2.0
                ) / dir_y
            }
        };

    let distancia =
        distancia_celdas.abs()
            * TAMANO_CELDA;

    let impacto_x =
        origen_x
            + dir_x
                * distancia;

    let impacto_y =
        origen_y
            + dir_y
                * distancia;

    let mut offset_textura =
        if golpe_vertical {
            impacto_y
                / TAMANO_CELDA
        } else {
            impacto_x
                / TAMANO_CELDA
        };

    offset_textura -=
        offset_textura.floor();

    if golpe_vertical
        && dir_x > 0.0
    {
        offset_textura =
            1.0
                - offset_textura;
    }

    if !golpe_vertical
        && dir_y < 0.0
    {
        offset_textura =
            1.0
                - offset_textura;
    }

    let tipo =
        mapa.celda(
            mapa_y,
            mapa_x,
        );

    RayHit {
        distancia,
        offset_textura,
        golpe_vertical,
        tipo,
    }
}

pub fn render_3d(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
    textura_pared: &TextureData,
    textura_puerta: &TextureData,
    textura_suelo: &TextureData,
) {
    render_suelo(
        framebuffer,
        player,
        camera,
        textura_suelo,
    );

    render_paredes(
        framebuffer,
        mapa,
        player,
        camera,
        textura_pared,
        textura_puerta,
    );
}

fn render_paredes(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
    textura_pared: &TextureData,
    textura_puerta: &TextureData,
) {
    let distancia_plano =
        (
            ANCHO_VENTANA as f32
                / 2.0
        )
            / (
                FOV / 2.0
            )
                .tan();

    for columna
        in 0..ANCHO_VENTANA
    {
        let porcentaje =
            columna as f32
                / ANCHO_VENTANA
                    as f32;

        let angulo_rayo =
            camera.angle
                - FOV / 2.0
                + porcentaje
                    * FOV;

        let hit =
            lanzar_rayo(
                mapa,
                player.x,
                player.y,
                angulo_rayo,
            );

        let distancia_corregida =
            hit.distancia
                * (
                    angulo_rayo
                        - camera.angle
                )
                    .cos();

        // Evita que la altura tienda a infinito
        // cuando estamos demasiado cerca.
        let distancia_segura =
            distancia_corregida
                .max(6.0);

        let mut altura_pared =
            TAMANO_CELDA
                * distancia_plano
                / distancia_segura;

        // Protección extra ante valores absurdos.
        let altura_maxima =
            ALTO_VENTANA as f32
                * 2.5;

        if altura_pared
            > altura_maxima
        {
            altura_pared =
                altura_maxima;
        }

        let centro_vertical =
            ALTO_VENTANA as f32
                / 2.0
                + camera.vertical_offset
                    as f32;

        let inicio_pared =
            centro_vertical
                - altura_pared
                    / 2.0;

        let fin_pared =
            centro_vertical
                + altura_pared
                    / 2.0;

        let textura =
            if hit.tipo == 'D' {
                textura_puerta
            } else {
                textura_pared
            };

        dibujar_columna_pared(
            framebuffer,
            columna,
            inicio_pared,
            fin_pared,
            hit.offset_textura,
            hit.distancia,
            hit.golpe_vertical,
            textura,
        );
    }
}

fn dibujar_columna_pared(
    framebuffer: &mut Framebuffer,
    x: i32,
    inicio: f32,
    fin: f32,
    offset_textura: f32,
    distancia: f32,
    golpe_vertical: bool,
    textura: &TextureData,
) {
    let altura =
        fin - inicio;

    if altura <= 0.0 {
        return;
    }

    let tex_x =
        (
            offset_textura
                * textura.width
                    as f32
        ) as i32;

    let tex_x =
        tex_x.clamp(
            0,
            textura.width - 1,
        );

    let inicio_pantalla =
        inicio
            .floor()
            .max(0.0)
            as i32;

    let fin_pantalla =
        fin
            .ceil()
            .min(
                ALTO_VENTANA
                    as f32
                    - 1.0,
            )
            as i32;

    let oscuridad =
        calcular_oscuridad(
            distancia,
        );

    for y
        in inicio_pantalla
            ..=fin_pantalla
    {
        let porcentaje_y =
            (
                y as f32
                    - inicio
            ) / altura;

        let tex_y =
            (
                porcentaje_y
                    * textura.height
                        as f32
            ) as i32;

        let tex_y =
            tex_y.clamp(
                0,
                textura.height - 1,
            );

        let mut color =
            textura.get_pixel(
                tex_x,
                tex_y,
            );

        if golpe_vertical {
            color.r =
                (
                    color.r as f32
                        * 0.85
                ) as u8;

            color.g =
                (
                    color.g as f32
                        * 0.85
                ) as u8;

            color.b =
                (
                    color.b as f32
                        * 0.85
                ) as u8;
        }

        color.r =
            (
                color.r as f32
                    * oscuridad
            ) as u8;

        color.g =
            (
                color.g as f32
                    * oscuridad
            ) as u8;

        color.b =
            (
                color.b as f32
                    * oscuridad
            ) as u8;

        framebuffer.point_color(
            x,
            y,
            color,
        );
    }
}

fn render_suelo(
    framebuffer: &mut Framebuffer,
    player: &Player,
    camera: &Camera,
    textura_suelo: &TextureData,
) {
    let centro_vertical =
        ALTO_VENTANA as f32
            / 2.0
            + camera.vertical_offset
                as f32;

    let distancia_plano =
        (
            ANCHO_VENTANA as f32
                / 2.0
        )
            / (
                FOV / 2.0
            )
                .tan();

    let dir_x =
        camera.angle.cos();

    let dir_y =
        camera.angle.sin();

    let plano_x =
        -dir_y
            * (
                FOV / 2.0
            )
                .tan();

    let plano_y =
        dir_x
            * (
                FOV / 2.0
            )
                .tan();

    let rayo_izquierdo_x =
        dir_x - plano_x;

    let rayo_izquierdo_y =
        dir_y - plano_y;

    let rayo_derecho_x =
        dir_x + plano_x;

    let rayo_derecho_y =
        dir_y + plano_y;

    let altura_camara =
        TAMANO_CELDA
            / 2.0;

    let inicio_y =
        (
            centro_vertical
                + 1.0
        )
            .max(0.0)
            as i32;

    for y
        in (
            inicio_y
                ..ALTO_VENTANA
        )
            .step_by(2)
    {
        let diferencia_y =
            y as f32
                - centro_vertical;

        if diferencia_y.abs()
            < 0.001
        {
            continue;
        }

        let distancia =
            altura_camara
                * distancia_plano
                / diferencia_y;

        let paso_x =
            distancia
                * (
                    rayo_derecho_x
                        - rayo_izquierdo_x
                )
                / ANCHO_VENTANA
                    as f32;

        let paso_y =
            distancia
                * (
                    rayo_derecho_y
                        - rayo_izquierdo_y
                )
                / ANCHO_VENTANA
                    as f32;

        let mut mundo_x =
            player.x
                + distancia
                    * rayo_izquierdo_x;

        let mut mundo_y =
            player.y
                + distancia
                    * rayo_izquierdo_y;

        let oscuridad =
            calcular_oscuridad(
                distancia,
            );

        for x
            in 0..ANCHO_VENTANA
        {
            let tex_x =
                (
                    mundo_x
                        .rem_euclid(
                            TAMANO_CELDA,
                        )
                        / TAMANO_CELDA
                        * textura_suelo.width
                            as f32
                ) as i32;

            let tex_y =
                (
                    mundo_y
                        .rem_euclid(
                            TAMANO_CELDA,
                        )
                        / TAMANO_CELDA
                        * textura_suelo.height
                            as f32
                ) as i32;

            let tex_x =
                tex_x.clamp(
                    0,
                    textura_suelo.width - 1,
                );

            let tex_y =
                tex_y.clamp(
                    0,
                    textura_suelo.height - 1,
                );

            let mut color =
                textura_suelo
                    .get_pixel(
                        tex_x,
                        tex_y,
                    );

            color.r =
                (
                    color.r as f32
                        * oscuridad
                ) as u8;

            color.g =
                (
                    color.g as f32
                        * oscuridad
                ) as u8;

            color.b =
                (
                    color.b as f32
                        * oscuridad
                ) as u8;

            framebuffer.point_color(
                x,
                y,
                color,
            );

            if y + 1
                < ALTO_VENTANA
            {
                framebuffer.point_color(
                    x,
                    y + 1,
                    color,
                );
            }

            mundo_x +=
                paso_x;

            mundo_y +=
                paso_y;
        }
    }
}

fn calcular_oscuridad(
    distancia: f32,
) -> f32 {
    let oscuridad =
        1.0
            - distancia
                / 700.0;

    oscuridad.clamp(
        0.25,
        1.0,
    )
}