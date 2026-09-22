use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::material::Material;
use crate::map::{
    Map,
    TAMANO_CELDA,
};
use crate::player::Player;
use crate::texture_data::TextureData;

use raylib::color::Color;

pub const ANCHO_VENTANA: i32 = 800;
pub const ALTO_VENTANA: i32 = 600;

const FOV: f32 =
    std::f32::consts::PI / 3.0;

const DISTANCIA_MAXIMA: f32 =
    2000.0;

pub struct RayHit {
    pub distancia: f32,
    pub offset_textura: f32,
    pub golpe_vertical: bool,
    pub tipo: char,
    pub impacto_x: f32,
    pub impacto_y: f32,
    pub normal_x: f32,
    pub normal_y: f32,
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

    let mut mapa_x =
        (
            origen_x
                / TAMANO_CELDA
        )
            .floor()
            as i32;

    let mut mapa_y =
        (
            origen_y
                / TAMANO_CELDA
        )
            .floor()
            as i32;

    let delta_dist_x =
        if dir_x.abs() < 0.00001 {
            f32::MAX
        } else {
            (
                TAMANO_CELDA
                    / dir_x
            )
                .abs()
        };

    let delta_dist_y =
        if dir_y.abs() < 0.00001 {
            f32::MAX
        } else {
            (
                TAMANO_CELDA
                    / dir_y
            )
                .abs()
        };

    let paso_x: i32;
    let paso_y: i32;

    let mut distancia_lateral_x: f32;
    let mut distancia_lateral_y: f32;

    if dir_x < 0.0 {
        paso_x =
            -1;

        distancia_lateral_x =
            (
                origen_x
                    - mapa_x
                        as f32
                        * TAMANO_CELDA
            )
                / dir_x
                    .abs()
                    .max(
                        0.00001,
                    );
    } else {
        paso_x =
            1;

        distancia_lateral_x =
            (
                (
                    mapa_x
                        as f32
                        + 1.0
                )
                    * TAMANO_CELDA
                    - origen_x
            )
                / dir_x
                    .abs()
                    .max(
                        0.00001,
                    );
    }

    if dir_y < 0.0 {
        paso_y =
            -1;

        distancia_lateral_y =
            (
                origen_y
                    - mapa_y
                        as f32
                        * TAMANO_CELDA
            )
                / dir_y
                    .abs()
                    .max(
                        0.00001,
                    );
    } else {
        paso_y =
            1;

        distancia_lateral_y =
            (
                (
                    mapa_y
                        as f32
                        + 1.0
                )
                    * TAMANO_CELDA
                    - origen_y
            )
                / dir_y
                    .abs()
                    .max(
                        0.00001,
                    );
    }

    let mut golpe_vertical =
        false;

    let mut distancia =
        0.0;

    let mut tipo =
        '#';

    for _ in 0..2048 {
        if distancia_lateral_x
            < distancia_lateral_y
        {
            distancia =
                distancia_lateral_x;

            distancia_lateral_x +=
                delta_dist_x;

            mapa_x +=
                paso_x;

            golpe_vertical =
                true;
        } else {
            distancia =
                distancia_lateral_y;

            distancia_lateral_y +=
                delta_dist_y;

            mapa_y +=
                paso_y;

            golpe_vertical =
                false;
        }

        if distancia
            > DISTANCIA_MAXIMA
        {
            break;
        }

        let celda =
            mapa.celda(
                mapa_y,
                mapa_x,
            );

        if mapa.es_bloque_solido(
            mapa_y,
            mapa_x,
        ) {
            tipo =
                celda;

            break;
        }
    }

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

    RayHit {
        distancia:
            distancia.max(
                0.0001,
            ),

        offset_textura,

        golpe_vertical,

        tipo,
        impacto_x,
        impacto_y,
        normal_x: if golpe_vertical { -(dir_x.signum()) } else { 0.0 },
        normal_y: if golpe_vertical { 0.0 } else { -(dir_y.signum()) },
    }
}

pub fn render_3d(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,

    textura_pared: &TextureData,
    textura_caja: &TextureData,
    textura_ventana: &TextureData,
    textura_puerta: &TextureData,

    textura_subir: &TextureData,
    textura_bajar: &TextureData,

    textura_suelo: &TextureData,
    textura_suelo2: &TextureData,

    textura_techo: &TextureData,
    panorama: Option<&TextureData>,
) {
    let limitar_suelo =
        panorama.is_some();

    if let Some(panorama) = panorama {
        render_panorama(framebuffer, camera, panorama);
    } else {
        render_techo(
            framebuffer,
            player,
            camera,
            textura_techo,
        );
    }

    render_suelo(
        framebuffer,
        mapa,
        player,
        camera,
        textura_suelo,
        textura_suelo2,
        limitar_suelo,
    );

    render_paredes(
        framebuffer,
        mapa,
        player,
        camera,

        textura_pared,
        textura_caja,
        textura_ventana,
        textura_puerta,

        textura_subir,
        textura_bajar,
        panorama,
    );
}

fn render_techo(
    framebuffer: &mut Framebuffer,
    player: &Player,
    camera: &Camera,
    textura_techo: &TextureData,
) {
    let horizonte =
        ALTO_VENTANA
            as f32
            / 2.0
            + camera
                .vertical_offset
                as f32;

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

    let rayo_izq_x =
        dir_x
            - plano_x;

    let rayo_izq_y =
        dir_y
            - plano_y;

    let rayo_der_x =
        dir_x
            + plano_x;

    let rayo_der_y =
        dir_y
            + plano_y;

    let posicion_vertical =
        TAMANO_CELDA
            * ALTO_VENTANA
                as f32
            / 2.0;

    let fin_y =
        horizonte
            .min(
                ALTO_VENTANA
                    as f32,
            )
            .max(
                0.0,
            )
            as i32;

    for y in 0..fin_y {
        let p =
            horizonte
                - y as f32;

        if p.abs()
            < 0.001
        {
            continue;
        }

        let distancia_fila =
            posicion_vertical
                / p;

        let paso_x =
            distancia_fila
                * (
                    rayo_der_x
                        - rayo_izq_x
                )
                / ANCHO_VENTANA
                    as f32;

        let paso_y =
            distancia_fila
                * (
                    rayo_der_y
                        - rayo_izq_y
                )
                / ANCHO_VENTANA
                    as f32;

        let mut mundo_x =
            player.x
                + distancia_fila
                    * rayo_izq_x;

        let mut mundo_y =
            player.y
                + distancia_fila
                    * rayo_izq_y;

        for x in 0..ANCHO_VENTANA {
            let local_x =
                mundo_x / TAMANO_CELDA;
            let local_y =
                mundo_y / TAMANO_CELDA;

            let u =
                local_x - local_x.floor();
            let v =
                local_y - local_y.floor();

            let tex_x =
                (
                    u
                        * textura_techo
                            .width
                            as f32
                )
                    .floor()
                    .clamp(
                        0.0,
                        textura_techo
                            .width
                            as f32
                            - 1.0,
                    )
                    as i32;

            let tex_y =
                (
                    v
                        * textura_techo
                            .height
                            as f32
                )
                    .floor()
                    .clamp(
                        0.0,
                        textura_techo
                            .height
                            as f32
                            - 1.0,
                    )
                    as i32;

            let mut color =
                textura_techo
                    .get_pixel(
                        tex_x,
                        tex_y,
                    );

            let oscuridad =
                (
                    0.80
                        - distancia_fila
                            / 1800.0
                )
                    .clamp(
                        0.30,
                        0.80,
                    );

            color.r =
                (
                    color.r
                        as f32
                        * oscuridad
                )
                    .clamp(
                        0.0,
                        255.0,
                    )
                    as u8;

            color.g =
                (
                    color.g
                        as f32
                        * oscuridad
                )
                    .clamp(
                        0.0,
                        255.0,
                    )
                    as u8;

            color.b =
                (
                    color.b
                        as f32
                        * oscuridad
                )
                    .clamp(
                        0.0,
                        255.0,
                    )
                    as u8;

            framebuffer
                .point_color(
                    x,
                    y,
                    color,
                );

            mundo_x +=
                paso_x;

            mundo_y +=
                paso_y;
        }
    }
}

fn render_suelo(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,
    textura_suelo: &TextureData,
    textura_suelo2: &TextureData,
    limitar_al_mapa: bool,
) {
    let horizonte =
        ALTO_VENTANA
            as f32
            / 2.0
            + camera
                .vertical_offset
                as f32;

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

    let rayo_izq_x =
        dir_x
            - plano_x;

    let rayo_izq_y =
        dir_y
            - plano_y;

    let rayo_der_x =
        dir_x
            + plano_x;

    let rayo_der_y =
        dir_y
            + plano_y;

    let posicion_vertical =
        TAMANO_CELDA
            * ALTO_VENTANA
                as f32
            / 2.0;

    let inicio_y =
        horizonte
            .max(
                0.0,
            )
            as i32;

    for y in inicio_y
        ..ALTO_VENTANA
    {
        let p =
            y as f32
                - horizonte;

        if p.abs()
            < 0.001
        {
            continue;
        }

        let distancia_fila =
            posicion_vertical
                / p;

        let paso_x =
            distancia_fila
                * (
                    rayo_der_x
                        - rayo_izq_x
                )
                / ANCHO_VENTANA
                    as f32;

        let paso_y =
            distancia_fila
                * (
                    rayo_der_y
                        - rayo_izq_y
                )
                / ANCHO_VENTANA
                    as f32;

        let mut mundo_x =
            player.x
                + distancia_fila
                    * rayo_izq_x;

        let mut mundo_y =
            player.y
                + distancia_fila
                    * rayo_izq_y;

        for x in 0..ANCHO_VENTANA {
            let columna =
                (
                    mundo_x
                        / TAMANO_CELDA
                )
                    .floor()
                    as i32;

            let fila =
                (
                    mundo_y
                        / TAMANO_CELDA
                )
                    .floor()
                    as i32;

            let celda =
                mapa.celda(
                    fila,
                    columna,
                );

            if limitar_al_mapa
                && (
                    fila < 0
                    || columna < 0
                    || fila >= mapa.alto() as i32
                    || columna >= mapa.ancho() as i32
                    || celda == 'G'
                )
            {
                mundo_x += paso_x;
                mundo_y += paso_y;
                continue;
            }

            let textura =
                if celda == 'C' {
                    textura_suelo2
                } else {
                    textura_suelo
                };

            let local_x =
                mundo_x / TAMANO_CELDA;
            let local_y =
                mundo_y / TAMANO_CELDA;

            let u =
                local_x - local_x.floor();
            let v =
                local_y - local_y.floor();

            let tex_x =
                (
                    u
                        * textura
                            .width
                            as f32
                )
                    .floor()
                    .clamp(
                        0.0,
                        textura.width
                            as f32
                            - 1.0,
                    )
                    as i32;

            let tex_y =
                (
                    v
                        * textura
                            .height
                            as f32
                )
                    .floor()
                    .clamp(
                        0.0,
                        textura.height
                            as f32
                            - 1.0,
                    )
                    as i32;

            let mut color =
                textura.get_pixel(
                    tex_x,
                    tex_y,
                );

            let sombra =
                (
                    1.0
                        - distancia_fila
                            / 1400.0
                )
                    .clamp(
                        0.35,
                        1.0,
                    );

            color.r =
                (
                    color.r
                        as f32
                        * sombra
                )
                    .clamp(
                        0.0,
                        255.0,
                    )
                    as u8;

            color.g =
                (
                    color.g
                        as f32
                        * sombra
                )
                    .clamp(
                        0.0,
                        255.0,
                    )
                    as u8;

            color.b =
                (
                    color.b
                        as f32
                        * sombra
                )
                    .clamp(
                        0.0,
                        255.0,
                    )
                    as u8;

            framebuffer
                .point_color(
                    x,
                    y,
                    color,
                );

            mundo_x +=
                paso_x;

            mundo_y +=
                paso_y;
        }
    }
}

struct Materiales<'a> {
    pared: Material<'a>,
    caja: Material<'a>,
    ventana: Material<'a>,
    puerta: Material<'a>,
    subir: Material<'a>,
    bajar: Material<'a>,
}

impl<'a> Materiales<'a> {
    fn para(&self, tipo: char) -> &Material<'a> {
        match tipo {
            'J' => &self.caja,
            'W' => &self.ventana,
            'D' => &self.puerta,
            'X' => &self.subir,
            'B' => &self.bajar,
            _ => &self.pared,
        }
    }
}

fn mezclar(a: Color, b: Color, peso_b: f32) -> Color {
    let t = peso_b.clamp(0.0, 1.0);
    Color::new(
        (a.r as f32 * (1.0 - t) + b.r as f32 * t) as u8,
        (a.g as f32 * (1.0 - t) + b.g as f32 * t) as u8,
        (a.b as f32 * (1.0 - t) + b.b as f32 * t) as u8,
        255,
    )
}

fn sumar_brillo(color: Color, brillo: f32) -> Color {
    let aumento = (brillo * 255.0).clamp(0.0, 255.0) as u8;
    Color::new(
        color.r.saturating_add(aumento),
        color.g.saturating_add(aumento),
        color.b.saturating_add(aumento),
        255,
    )
}

fn reflejar(dir: (f32, f32), normal: (f32, f32)) -> (f32, f32) {
    let producto = dir.0 * normal.0 + dir.1 * normal.1;
    (dir.0 - 2.0 * producto * normal.0, dir.1 - 2.0 * producto * normal.1)
}

/// Ley de Snell. `normal` apunta hacia el medio del que llega el rayo.
fn refractar(dir: (f32, f32), normal: (f32, f32), indice_entrada: f32, indice_salida: f32) -> Option<(f32, f32)> {
    let eta = indice_entrada / indice_salida;
    let cos_i = -(dir.0 * normal.0 + dir.1 * normal.1);
    let k = 1.0 - eta * eta * (1.0 - cos_i * cos_i);
    if k < 0.0 {
        return None;
    }
    let factor = eta * cos_i - k.sqrt();
    Some((eta * dir.0 + factor * normal.0, eta * dir.1 + factor * normal.1))
}

/// Atraviesa la celda de vidrio: aire → vidrio → aire y traza el siguiente objeto.
fn trazar_vidrio(mapa: &Map, hit: &RayHit, dir: (f32, f32), indice: f32) -> Option<(RayHit, (f32, f32))> {
    let dentro = refractar(dir, (hit.normal_x, hit.normal_y), 1.0, indice)?;
    let inicio_x = hit.impacto_x + dentro.0 * 0.01;
    let inicio_y = hit.impacto_y + dentro.1 * 0.01;
    let celda_x = (inicio_x / TAMANO_CELDA).floor();
    let celda_y = (inicio_y / TAMANO_CELDA).floor();

    let t_x = if dentro.0 > 0.00001 {
        ((celda_x + 1.0) * TAMANO_CELDA - inicio_x) / dentro.0
    } else if dentro.0 < -0.00001 {
        (celda_x * TAMANO_CELDA - inicio_x) / dentro.0
    } else { f32::INFINITY };
    let t_y = if dentro.1 > 0.00001 {
        ((celda_y + 1.0) * TAMANO_CELDA - inicio_y) / dentro.1
    } else if dentro.1 < -0.00001 {
        (celda_y * TAMANO_CELDA - inicio_y) / dentro.1
    } else { f32::INFINITY };

    let salida_vertical = t_x < t_y;
    let recorrido = t_x.min(t_y);
    if !recorrido.is_finite() || recorrido <= 0.0 {
        return None;
    }
    let normal_salida = if salida_vertical {
        (-dentro.0.signum(), 0.0)
    } else {
        (0.0, -dentro.1.signum())
    };
    let fuera = refractar(dentro, normal_salida, indice, 1.0)?;
    let x = inicio_x + dentro.0 * recorrido + fuera.0 * 0.01;
    let y = inicio_y + dentro.1 * recorrido + fuera.1 * 0.01;
    Some((lanzar_rayo(mapa, x, y, fuera.1.atan2(fuera.0)), fuera))
}

fn color_rayo_secundario(hit: &RayHit, dir: (f32, f32), v: f32, materiales: &Materiales<'_>, panorama: Option<&TextureData>) -> Color {
    if hit.tipo == 'G' {
        if let Some(cielo) = panorama {
            let angulo = dir.1.atan2(dir.0).rem_euclid(std::f32::consts::TAU);
            return cielo.get_pixel(
                (angulo / std::f32::consts::TAU * cielo.width as f32) as i32,
                (v.clamp(0.0, 1.0) * cielo.height as f32) as i32,
            );
        }
        return Color::new(12, 15, 22, 255);
    }
    let material = materiales.para(hit.tipo);
    let base = material.sample(hit.offset_textura, v);
    let sombra = (1.0 - hit.distancia / 900.0).clamp(0.28, 1.0);
    Color::new(
        (base.r as f32 * sombra) as u8,
        (base.g as f32 * sombra) as u8,
        (base.b as f32 * sombra) as u8,
        255,
    )
}

fn brillo_especular(dir: (f32, f32), normal: (f32, f32), intensidad: f32) -> f32 {
    let luz = (0.6_f32, -0.8_f32);
    let mitad = (luz.0 - dir.0, luz.1 - dir.1);
    let longitud = (mitad.0 * mitad.0 + mitad.1 * mitad.1).sqrt().max(0.0001);
    let coseno = ((normal.0 * mitad.0 + normal.1 * mitad.1) / longitud).max(0.0);
    intensidad * coseno.powf(24.0) * 0.55
}

fn render_paredes(
    framebuffer: &mut Framebuffer,
    mapa: &Map,
    player: &Player,
    camera: &Camera,

    textura_pared: &TextureData,
    textura_caja: &TextureData,
    textura_ventana: &TextureData,
    textura_puerta: &TextureData,

    textura_subir: &TextureData,
    textura_bajar: &TextureData,
    panorama: Option<&TextureData>,
) {
    let materiales = Materiales {
        pared: Material::new(textura_pared, Color::new(235, 230, 220, 255), 0.08, 0.0, 0.02, 1.0),
        caja: Material::new(textura_caja, Color::new(215, 225, 240, 255), 0.80, 0.0, 0.52, 1.0),
        ventana: Material::new(textura_ventana, Color::new(185, 215, 245, 255), 0.75, 0.72, 0.12, 1.5),
        puerta: Material::new(textura_puerta, Color::new(225, 205, 180, 255), 0.20, 0.0, 0.04, 1.0),
        subir: Material::new(textura_subir, Color::new(220, 230, 235, 255), 0.35, 0.0, 0.08, 1.0),
        bajar: Material::new(textura_bajar, Color::new(205, 215, 225, 255), 0.30, 0.0, 0.06, 1.0),
    };
    for columna_pantalla
        in 0..ANCHO_VENTANA
    {
        let porcentaje =
            columna_pantalla
                as f32
                / ANCHO_VENTANA
                    as f32;

        let angulo_rayo =
            camera.angle
                - FOV
                    / 2.0
                + porcentaje
                    * FOV;

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
            (
                hit.distancia
                    * diferencia_angulo
                        .cos()
            )
                .max(
                    0.001,
                );

        let altura_pared =
            (
                TAMANO_CELDA
                    * ALTO_VENTANA
                        as f32
                    / distancia_corregida
            )
                .max(
                    1.0,
                );

        let centro =
            ALTO_VENTANA
                as f32
                / 2.0
                + camera
                    .vertical_offset
                    as f32;

        let inicio =
            centro
                - altura_pared
                    / 2.0;

        let fin =
            centro
                + altura_pared
                    / 2.0;

        if hit.tipo == 'G' {
            continue;
        }

        dibujar_columna_pared(
            framebuffer,
            columna_pantalla,
            inicio,
            fin,
            &hit,
            &materiales,
            mapa,
            (angulo_rayo.cos(), angulo_rayo.sin()),
            panorama,
            distancia_corregida,
        );
    }
}

fn dibujar_columna_pared(
    framebuffer: &mut Framebuffer,
    x: i32,
    inicio: f32,
    fin: f32,
    hit: &RayHit,
    materiales: &Materiales<'_>,
    mapa: &Map,
    direccion: (f32, f32),
    panorama: Option<&TextureData>,
    distancia: f32,
) {
    let altura =
        fin
            - inicio;

    if altura <= 0.0 {
        return;
    }

    let inicio_dibujo =
        inicio
            .max(
                0.0,
            )
            as i32;

    let fin_dibujo =
        fin
            .min(
                ALTO_VENTANA
                    as f32
                    - 1.0,
            )
            as i32;

    let sombra_distancia =
        (
            1.0
                - distancia
                    / 900.0
        )
            .clamp(
                0.28,
                1.0,
            );

    let sombra_lado =
        if hit.golpe_vertical {
            1.0
        } else {
            0.82
        };

    let sombra =
        sombra_distancia
            * sombra_lado;

    let material = materiales.para(hit.tipo);
    // Los rayos secundarios dependen de la columna, no de cada píxel vertical.
    let transmitido = if material.transparency > 0.0 {
        trazar_vidrio(mapa, hit, direccion, material.refractive_index)
    } else {
        None
    };
    let reflejado = if material.reflectivity > 0.10 {
        let dir = reflejar(direccion, (hit.normal_x, hit.normal_y));
        Some((
            lanzar_rayo(
                mapa,
                hit.impacto_x + dir.0 * 0.01,
                hit.impacto_y + dir.1 * 0.01,
                dir.1.atan2(dir.0),
            ),
            dir,
        ))
    } else {
        None
    };

    for y in inicio_dibujo
        ..=fin_dibujo
    {
        let porcentaje_y =
            (
                y as f32
                    - inicio
            )
                / altura;

        let mut color = material.sample(hit.offset_textura, porcentaje_y);

        if let Some((secondary_hit, secondary_dir)) = &transmitido {
            let fondo = color_rayo_secundario(secondary_hit, *secondary_dir, porcentaje_y, materiales, panorama);
            color = mezclar(color, fondo, material.transparency);
        }

        if let Some((secondary_hit, secondary_dir)) = &reflejado {
            let reflejo = color_rayo_secundario(secondary_hit, *secondary_dir, porcentaje_y, materiales, panorama);
            color = mezclar(color, reflejo, material.reflectivity);
        }

        let brillo = brillo_especular(direccion, (hit.normal_x, hit.normal_y), material.specular);
        color = sumar_brillo(color, brillo);

        color.r =
            (
                color.r
                    as f32
                    * sombra
            )
                .clamp(
                    0.0,
                    255.0,
                )
                as u8;

        color.g =
            (
                color.g
                    as f32
                    * sombra
            )
                .clamp(
                    0.0,
                    255.0,
                )
                as u8;

        color.b =
            (
                color.b
                    as f32
                    * sombra
            )
                .clamp(
                    0.0,
                    255.0,
                )
                as u8;

        framebuffer
            .point_color(
                x,
                y,
                color,
            );
    }
}

fn render_panorama(
    framebuffer: &mut Framebuffer,
    camera: &Camera,
    panorama: &TextureData,
) {
    for y in 0..ALTO_VENTANA {
        let textura_y =
            ((y as f32 / ALTO_VENTANA as f32)
                * panorama.height as f32)
                .clamp(0.0, panorama.height as f32 - 1.0)
                as i32;

        for x in 0..ANCHO_VENTANA {
            let desplazamiento =
                (x as f32 / ANCHO_VENTANA as f32 - 0.5) * FOV;
            let angulo =
                (camera.angle + desplazamiento)
                    .rem_euclid(std::f32::consts::PI * 2.0);
            let textura_x =
                (angulo / (std::f32::consts::PI * 2.0)
                    * panorama.width as f32) as i32
                    % panorama.width;

            framebuffer.point_color(
                x,
                y,
                panorama.get_pixel(textura_x, textura_y),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{reflejar, refractar};

    #[test]
    fn reflejo_invierte_componente_perpendicular() {
        let salida = reflejar((0.6, 0.8), (-1.0, 0.0));
        assert!((salida.0 + 0.6).abs() < 0.0001);
        assert!((salida.1 - 0.8).abs() < 0.0001);
    }

    #[test]
    fn refraccion_hacia_vidrio_se_acerca_a_la_normal() {
        let entrada = (0.8, 0.6);
        let salida = refractar(entrada, (-1.0, 0.0), 1.0, 1.5).unwrap();
        assert!(salida.1.abs() < entrada.1.abs());
        assert!((salida.0 * salida.0 + salida.1 * salida.1 - 1.0).abs() < 0.0001);
    }
}
