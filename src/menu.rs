use raylib::prelude::*;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum EstadoJuego {
    Menu,
    Jugando,
    Controles,
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum AccionMenu {
    Ninguna,
    Jugar,
    Controles,
    Salir,
}

pub struct Menu {
    opcion: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            opcion: 0,
        }
    }

    pub fn update(
        &mut self,
        rl: &RaylibHandle,
    ) -> AccionMenu {
        if rl.is_key_pressed(
            KeyboardKey::KEY_DOWN,
        ) {
            self.opcion =
                (self.opcion + 1) % 3;
        }

        if rl.is_key_pressed(
            KeyboardKey::KEY_UP,
        ) {
            if self.opcion == 0 {
                self.opcion = 2;
            } else {
                self.opcion -= 1;
            }
        }

        if !rl.is_key_pressed(
            KeyboardKey::KEY_ENTER,
        ) {
            return AccionMenu::Ninguna;
        }

        match self.opcion {
            0 => AccionMenu::Jugar,
            1 => AccionMenu::Controles,
            2 => AccionMenu::Salir,
            _ => AccionMenu::Ninguna,
        }
    }

    pub fn render_menu(
        &self,
        d: &mut RaylibDrawHandle,
    ) {
        let ancho =
            d.get_screen_width();

        let alto =
            d.get_screen_height();

        d.clear_background(
            Color::new(
                7,
                7,
                9,
                255,
            ),
        );

        dibujar_fondo(
            d,
            ancho,
            alto,
        );

        let titulo =
            "NIGHTMARE";

        let subtitulo =
            "SURVIVAL";

        let titulo_size =
            72;

        let ancho_titulo =
            d.measure_text(
                titulo,
                titulo_size,
            );

        d.draw_text(
            titulo,
            ancho / 2
                - ancho_titulo / 2,
            alto / 2
                - 220,
            titulo_size,
            Color::new(
                170,
                20,
                20,
                255,
            ),
        );

        let ancho_subtitulo =
            d.measure_text(
                subtitulo,
                28,
            );

        d.draw_text(
            subtitulo,
            ancho / 2
                - ancho_subtitulo / 2,
            alto / 2
                - 145,
            28,
            Color::GRAY,
        );

        let opciones = [
            "JUGAR",
            "CONTROLES",
            "SALIR",
        ];

        for (
            i,
            texto,
        ) in opciones
            .iter()
            .enumerate()
        {
            let seleccionado =
                i == self.opcion;

            let color =
                if seleccionado {
                    Color::RED
                } else {
                    Color::LIGHTGRAY
                };

            let size =
                if seleccionado {
                    36
                } else {
                    30
                };

            let texto_final =
                if seleccionado {
                    format!(
                        "> {} <",
                        texto,
                    )
                } else {
                    texto.to_string()
                };

            let ancho_texto =
                d.measure_text(
                    &texto_final,
                    size,
                );

            d.draw_text(
                &texto_final,
                ancho / 2
                    - ancho_texto / 2,
                alto / 2
                    + i as i32 * 58,
                size,
                color,
            );
        }

        let instrucciones =
            "FLECHAS PARA MOVER - ENTER PARA SELECCIONAR";

        let ancho_instrucciones =
            d.measure_text(
                instrucciones,
                16,
            );

        d.draw_text(
            instrucciones,
            ancho / 2
                - ancho_instrucciones / 2,
            alto - 45,
            16,
            Color::DARKGRAY,
        );
    }

    pub fn render_controles(
        &self,
        d: &mut RaylibDrawHandle,
    ) {
        let ancho =
            d.get_screen_width();

        let alto =
            d.get_screen_height();

        d.clear_background(
            Color::new(
                7,
                7,
                9,
                255,
            ),
        );

        dibujar_fondo(
            d,
            ancho,
            alto,
        );

        let titulo =
            "CONTROLES";

        let ancho_titulo =
            d.measure_text(
                titulo,
                50,
            );

        d.draw_text(
            titulo,
            ancho / 2
                - ancho_titulo / 2,
            55,
            50,
            Color::RED,
        );

        let controles = [
            (
                "WASD",
                "Movimiento",
            ),
            (
                "MOUSE",
                "Mover camara",
            ),
            (
                "CLICK IZQ.",
                "Disparar / Hachazo",
            ),
            (
                "CLICK DER.",
                "Apuntar / Bloquear",
            ),
            (
                "R",
                "Recargar",
            ),
            (
                "E",
                "Interactuar",
            ),
            (
                "1",
                "Pistola",
            ),
            (
                "2",
                "Hacha",
            ),
            (
                "F5",
                "Reiniciar partida",
            ),
            (
                "F11",
                "Pantalla completa",
            ),
            (
                "TAB",
                "Liberar cursor",
            ),
            (
                "BACKSPACE",
                "Volver al menu",
            ),
        ];

        for (
            i,
            (
                tecla,
                accion,
            ),
        ) in controles
            .iter()
            .enumerate()
        {
            let y =
                140
                    + i as i32 * 34;

            d.draw_text(
                tecla,
                ancho / 2
                    - 240,
                y,
                21,
                Color::WHITE,
            );

            d.draw_text(
                accion,
                ancho / 2
                    + 20,
                y,
                21,
                Color::GRAY,
            );
        }

        let texto =
            "BACKSPACE - VOLVER";

        let ancho_texto =
            d.measure_text(
                texto,
                20,
            );

        d.draw_text(
            texto,
            ancho / 2
                - ancho_texto / 2,
            alto - 45,
            20,
            Color::RED,
        );
    }
}

fn dibujar_fondo(
    d: &mut RaylibDrawHandle,
    ancho: i32,
    alto: i32,
) {
    for y in 0..alto {
        let porcentaje =
            y as f32
                / alto.max(1)
                    as f32;

        let gris =
            (
                18.0
                    * (
                        1.0
                            - porcentaje
                    )
            )
                as u8;

        d.draw_line(
            0,
            y,
            ancho,
            y,
            Color::new(
                gris,
                gris,
                gris,
                255,
            ),
        );
    }
}