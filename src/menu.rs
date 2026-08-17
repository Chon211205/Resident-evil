use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EstadoJuego {
    Menu,
    SeleccionNivel,
    Jugando,
    Controles,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AccionMenu {
    Ninguna,
    Jugar,
    SeleccionarNivel,
    Controles,
    Salir,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NivelSeleccionado {
    Mansion,
    Laboratorio,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AccionSeleccionNivel {
    Ninguna,
    Elegir(NivelSeleccionado),
    Volver,
}

pub struct Menu {
    opcion_menu: usize,
    opcion_nivel: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            opcion_menu: 0,
            opcion_nivel: 0,
        }
    }

    pub fn update(
        &mut self,
        rl: &RaylibHandle,
    ) -> AccionMenu {
        const TOTAL_OPCIONES: usize = 4;

        if rl.is_key_pressed(
            KeyboardKey::KEY_UP,
        ) {
            if self.opcion_menu == 0 {
                self.opcion_menu =
                    TOTAL_OPCIONES - 1;
            } else {
                self.opcion_menu -= 1;
            }
        }

        if rl.is_key_pressed(
            KeyboardKey::KEY_DOWN,
        ) {
            self.opcion_menu =
                (
                    self.opcion_menu + 1
                ) % TOTAL_OPCIONES;
        }

        if rl.is_key_pressed(
            KeyboardKey::KEY_ENTER,
        ) {
            return match self.opcion_menu {
                0 => AccionMenu::Jugar,

                1 => {
                    AccionMenu::SeleccionarNivel
                }

                2 => {
                    AccionMenu::Controles
                }

                3 => {
                    AccionMenu::Salir
                }

                _ => {
                    AccionMenu::Ninguna
                }
            };
        }

        AccionMenu::Ninguna
    }

    pub fn update_seleccion_nivel(
        &mut self,
        rl: &RaylibHandle,
    ) -> AccionSeleccionNivel {
        const TOTAL_NIVELES: usize = 2;

        if rl.is_key_pressed(
            KeyboardKey::KEY_UP,
        ) {
            if self.opcion_nivel == 0 {
                self.opcion_nivel =
                    TOTAL_NIVELES - 1;
            } else {
                self.opcion_nivel -= 1;
            }
        }

        if rl.is_key_pressed(
            KeyboardKey::KEY_DOWN,
        ) {
            self.opcion_nivel =
                (
                    self.opcion_nivel + 1
                ) % TOTAL_NIVELES;
        }

        if rl.is_key_pressed(
            KeyboardKey::KEY_ENTER,
        ) {
            return match self.opcion_nivel {
                0 => {
                    AccionSeleccionNivel::Elegir(
                        NivelSeleccionado::Mansion,
                    )
                }

                1 => {
                    AccionSeleccionNivel::Elegir(
                        NivelSeleccionado::Laboratorio,
                    )
                }

                _ => {
                    AccionSeleccionNivel::Ninguna
                }
            };
        }

        if rl.is_key_pressed(
            KeyboardKey::KEY_BACKSPACE,
        ) {
            return AccionSeleccionNivel::Volver;
        }

        AccionSeleccionNivel::Ninguna
    }

    pub fn render_menu(
        &self,
        d: &mut RaylibDrawHandle,
    ) {
        let sw =
            d.get_screen_width();

        let sh =
            d.get_screen_height();

        d.clear_background(
            Color::new(
                5,
                5,
                8,
                255,
            ),
        );

        for y in 0..sh {
            let factor =
                y as f32
                    / sh.max(1) as f32;

            let intensidad =
                (
                    16.0
                        * (
                            1.0 - factor
                        )
                ) as u8;

            d.draw_line(
                0,
                y,
                sw,
                y,
                Color::new(
                    intensidad,
                    0,
                    0,
                    255,
                ),
            );
        }

        let titulo =
            "NIGHTMARE";

        let subtitulo =
            "SURVIVAL";

        let titulo_tamano =
            64;

        let subtitulo_tamano =
            26;

        let titulo_ancho =
            d.measure_text(
                titulo,
                titulo_tamano,
            );

        let subtitulo_ancho =
            d.measure_text(
                subtitulo,
                subtitulo_tamano,
            );

        d.draw_text(
            titulo,
            sw / 2
                - titulo_ancho / 2,
            sh / 2
                - 230,
            titulo_tamano,
            Color::RED,
        );

        d.draw_text(
            subtitulo,
            sw / 2
                - subtitulo_ancho / 2,
            sh / 2
                - 160,
            subtitulo_tamano,
            Color::LIGHTGRAY,
        );

        let opciones = [
            "JUGAR",
            "SELECCIONAR NIVEL",
            "CONTROLES",
            "SALIR",
        ];

        for (
            indice,
            texto,
        ) in opciones
            .iter()
            .enumerate()
        {
            let seleccionado =
                indice == self.opcion_menu;

            let tamano =
                if seleccionado {
                    32
                } else {
                    27
                };

            let color =
                if seleccionado {
                    Color::GOLD
                } else {
                    Color::WHITE
                };

            let prefijo =
                if seleccionado {
                    "> "
                } else {
                    "  "
                };

            let opcion =
                format!(
                    "{}{}",
                    prefijo,
                    texto,
                );

            let ancho =
                d.measure_text(
                    &opcion,
                    tamano,
                );

            let y =
                sh / 2
                    - 55
                    + indice as i32
                        * 55;

            d.draw_text(
                &opcion,
                sw / 2
                    - ancho / 2,
                y,
                tamano,
                color,
            );
        }

        let ayuda =
            "FLECHAS - MOVER   ENTER - SELECCIONAR";

        let ayuda_ancho =
            d.measure_text(
                ayuda,
                18,
            );

        d.draw_text(
            ayuda,
            sw / 2
                - ayuda_ancho / 2,
            sh - 55,
            18,
            Color::GRAY,
        );
    }

    pub fn render_seleccion_nivel(
        &self,
        d: &mut RaylibDrawHandle,
    ) {
        let sw =
            d.get_screen_width();

        let sh =
            d.get_screen_height();

        d.clear_background(
            Color::new(
                5,
                5,
                8,
                255,
            ),
        );

        for y in 0..sh {
            let factor =
                y as f32
                    / sh.max(1) as f32;

            let intensidad =
                (
                    18.0
                        * (
                            1.0 - factor
                        )
                ) as u8;

            d.draw_line(
                0,
                y,
                sw,
                y,
                Color::new(
                    0,
                    intensidad / 2,
                    intensidad,
                    255,
                ),
            );
        }

        let titulo =
            "SELECCIONAR NIVEL";

        let titulo_ancho =
            d.measure_text(
                titulo,
                48,
            );

        d.draw_text(
            titulo,
            sw / 2
                - titulo_ancho / 2,
            sh / 2
                - 200,
            48,
            Color::WHITE,
        );

        let niveles = [
            (
                "MANSION",
                "Sobrevive dentro de la mansion",
            ),
            (
                "LABORATORIO",
                "Sobrevive dentro del laboratorio",
            ),
        ];

        for (
            indice,
            (
                nombre,
                descripcion,
            ),
        ) in niveles
            .iter()
            .enumerate()
        {
            let seleccionado =
                indice == self.opcion_nivel;

            let y =
                sh / 2
                    - 70
                    + indice as i32
                        * 120;

            let color =
                if seleccionado {
                    Color::GOLD
                } else {
                    Color::WHITE
                };

            let texto =
                if seleccionado {
                    format!(
                        "> {}",
                        nombre,
                    )
                } else {
                    format!(
                        "  {}",
                        nombre,
                    )
                };

            let ancho =
                d.measure_text(
                    &texto,
                    32,
                );

            d.draw_text(
                &texto,
                sw / 2
                    - ancho / 2,
                y,
                32,
                color,
            );

            let descripcion_ancho =
                d.measure_text(
                    descripcion,
                    18,
                );

            d.draw_text(
                descripcion,
                sw / 2
                    - descripcion_ancho / 2,
                y + 42,
                18,
                Color::GRAY,
            );
        }

        let volver =
            "BACKSPACE - VOLVER";

        let volver_ancho =
            d.measure_text(
                volver,
                18,
            );

        d.draw_text(
            volver,
            sw / 2
                - volver_ancho / 2,
            sh - 60,
            18,
            Color::GRAY,
        );
    }

    pub fn render_controles(
        &self,
        d: &mut RaylibDrawHandle,
    ) {
        let sw =
            d.get_screen_width();

        let sh =
            d.get_screen_height();

        d.clear_background(
            Color::new(
                5,
                5,
                8,
                255,
            ),
        );

        let titulo =
            "CONTROLES";

        let titulo_ancho =
            d.measure_text(
                titulo,
                48,
            );

        d.draw_text(
            titulo,
            sw / 2
                - titulo_ancho / 2,
            60,
            48,
            Color::RED,
        );

        let controles = [
            (
                "W A S D",
                "Movimiento",
            ),
            (
                "MOUSE",
                "Camara",
            ),
            (
                "CLICK DERECHO",
                "Apuntar / Bloquear",
            ),
            (
                "CLICK IZQUIERDO",
                "Disparar / Atacar",
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
                "R",
                "Recargar",
            ),
            (
                "E",
                "Interactuar",
            ),
            (
                "TAB",
                "Liberar cursor",
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
                "BACKSPACE",
                "Volver al menu",
            ),
        ];

        let inicio_y =
            145;

        for (
            indice,
            (
                tecla,
                accion,
            ),
        ) in controles
            .iter()
            .enumerate()
        {
            let y =
                inicio_y
                    + indice as i32
                        * 34;

            d.draw_text(
                tecla,
                sw / 2
                    - 260,
                y,
                20,
                Color::GOLD,
            );

            d.draw_text(
                accion,
                sw / 2
                    - 20,
                y,
                20,
                Color::LIGHTGRAY,
            );
        }

        let volver =
            "BACKSPACE - VOLVER";

        let volver_ancho =
            d.measure_text(
                volver,
                18,
            );

        d.draw_text(
            volver,
            sw / 2
                - volver_ancho / 2,
            sh - 45,
            18,
            Color::GRAY,
        );
    }
}