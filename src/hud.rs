use crate::inventory::Inventory;

use raylib::prelude::*;

pub fn render_hud(
    dibujo: &mut RaylibDrawHandle,
    vida_jugador: i32,
    municion: i32,
    inventory: &Inventory,
    mensaje: &str,
) {
    dibujar_vida(
        dibujo,
        vida_jugador,
    );

    dibujar_llave(
        dibujo,
        inventory,
    );

    dibujar_municion(
        dibujo,
        municion,
    );

    dibujar_mensaje(
        dibujo,
        mensaje,
    );

    dibujar_fps(
        dibujo,
    );

    if vida_jugador <= 0 {
        dibujar_muerte(
            dibujo,
        );
    }
}

fn dibujar_vida(
    dibujo: &mut RaylibDrawHandle,
    vida: i32,
) {
    let texto =
        format!(
            "Vida: {}",
            vida,
        );

    let color =
        if vida > 60 {
            Color::GREEN
        } else if vida > 30 {
            Color::YELLOW
        } else {
            Color::RED
        };

    dibujo.draw_text(
        &texto,
        10,
        65,
        20,
        color,
    );
}

fn dibujar_llave(
    dibujo: &mut RaylibDrawHandle,
    inventory: &Inventory,
) {
    if inventory.tiene_llave() {
        dibujo.draw_text(
            "Llave: SI",
            10,
            40,
            18,
            Color::YELLOW,
        );
    } else {
        dibujo.draw_text(
            "Llave: NO",
            10,
            40,
            18,
            Color::GRAY,
        );
    }
}

fn dibujar_municion(
    dibujo: &mut RaylibDrawHandle,
    municion: i32,
) {
    let texto =
        format!(
            "Municion: {}/24",
            municion,
        );

    let color =
        if municion > 5 {
            Color::WHITE
        } else {
            Color::RED
        };

    dibujo.draw_text(
        &texto,
        10,
        90,
        20,
        color,
    );
}

fn dibujar_mensaje(
    dibujo: &mut RaylibDrawHandle,
    mensaje: &str,
) {
    if mensaje.is_empty() {
        return;
    }

    dibujo.draw_rectangle(
        20,
        dibujo.get_screen_height() - 70,
        440,
        40,
        Color::new(
            0,
            0,
            0,
            180,
        ),
    );

    dibujo.draw_text(
        mensaje,
        30,
        dibujo.get_screen_height() - 60,
        20,
        Color::WHITE,
    );
}

fn dibujar_fps(
    dibujo: &mut RaylibDrawHandle,
) {
    let texto =
        format!(
            "FPS: {}",
            dibujo.get_fps(),
        );

    dibujo.draw_text(
        &texto,
        dibujo.get_screen_width() - 100,
        10,
        20,
        Color::GREEN,
    );
}

fn dibujar_muerte(
    dibujo: &mut RaylibDrawHandle,
) {
    let texto =
        "HAS MUERTO";

    let ancho_texto =
        dibujo.measure_text(
            texto,
            50,
        );

    dibujo.draw_rectangle(
        0,
        0,
        dibujo.get_screen_width(),
        dibujo.get_screen_height(),
        Color::new(
            0,
            0,
            0,
            170,
        ),
    );

    dibujo.draw_text(
        texto,
        dibujo.get_screen_width() / 2
            - ancho_texto / 2,
        dibujo.get_screen_height() / 2
            - 25,
        50,
        Color::RED,
    );

    dibujo.draw_text(
        "Presiona R para reiniciar",
        dibujo.get_screen_width() / 2
            - 120,
        dibujo.get_screen_height() / 2
            + 40,
        20,
        Color::WHITE,
    );
}