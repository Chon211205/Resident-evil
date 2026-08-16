use crate::inventory::Inventory;
use raylib::prelude::*;

pub fn render_hud(
    dibujo: &mut RaylibDrawHandle,
    vida_jugador: i32,
    balas_cargador: i32,
    balas_reserva: i32,
    inventory: &Inventory,
    mensaje: &str,
    offset_x: f32,
    offset_y: f32,
    ancho_render: f32,
    alto_render: f32,
    escala: f32,
) {
    let margen =
        18.0 * escala;

    let tamano_texto =
        (18.0 * escala)
            .max(14.0) as i32;

    let tamano_mensaje =
        (16.0 * escala)
            .max(13.0) as i32;

    let x_izquierda =
        offset_x
            + margen;

    let y_inferior =
        offset_y
            + alto_render
            - 35.0 * escala;

    dibujo.draw_text(
        &format!(
            "Vida: {}",
            vida_jugador,
        ),
        x_izquierda as i32,
        y_inferior as i32,
        tamano_texto,
        Color::GREEN,
    );

    dibujo.draw_text(
        &format!(
            "Municion: {}/{}",
            balas_cargador,
            balas_reserva,
        ),
        (
            x_izquierda
                + 150.0 * escala
        ) as i32,
        y_inferior as i32,
        tamano_texto,
        Color::WHITE,
    );

    if inventory.tiene_llave() {
        dibujo.draw_text(
            "Llave",
            (
                x_izquierda
                    + 360.0 * escala
            ) as i32,
            y_inferior as i32,
            tamano_texto,
            Color::YELLOW,
        );
    }

    if !mensaje.is_empty() {
        let ancho_mensaje =
            dibujo.measure_text(
                mensaje,
                tamano_mensaje,
            );

        let x_mensaje =
            offset_x
                + ancho_render / 2.0
                - ancho_mensaje as f32 / 2.0;

        let y_mensaje =
            offset_y
                + alto_render
                - 65.0 * escala;

        dibujo.draw_text(
            mensaje,
            x_mensaje as i32,
            y_mensaje as i32,
            tamano_mensaje,
            Color::WHITE,
        );
    }

    let fps =
        dibujo.get_fps();

    let texto_fps =
        format!(
            "FPS: {}",
            fps,
        );

    let tamano_fps =
        (18.0 * escala)
            .max(14.0) as i32;

    let ancho_fps =
        dibujo.measure_text(
            &texto_fps,
            tamano_fps,
        );

    dibujo.draw_text(
        &texto_fps,
        (
            offset_x
                + ancho_render
                - ancho_fps as f32
                - margen
        ) as i32,
        (
            offset_y
                + margen
        ) as i32,
        tamano_fps,
        Color::GREEN,
    );
}