use std::fs;

pub const TAMANO_CELDA: f32 = 25.0;

pub struct Map {
    pub data: Vec<Vec<char>>,
}

impl Map {
    pub fn new() -> Self {
        let filas = vec![
            "#########################################",
            "#P        #             #               #",
            "#         #             #       Z       #",
            "#         #         Z   #               #",
            "#         #             #               #",
            "##### #########   #############   #######",
            "#         #             #               #",
            "#   K     #       Z     #    Z      Z   #",
            "#         D             #       Z       #",
            "#         #             #               #",
            "###########   ###################   #####",
            "#         #             #               #",
            "#         #             #               #",
            "#         #                     Z       #",
            "#         ###########D###################",
            "#                                       #",
            "#             #                         #",
            "#             #                         #",
            "#             #                         #",
            "#####   #####################   #########",
            "#             #                         #",
            "#      Z      #                         #",
            "#             #                         #",
            "#                                       #",
            "#########   #####################   #####",
            "#             #                         #",
            "#             #           Z             #",
            "#             #                         #",
            "#             #                         #",
            "#####   #####################   #########",
            "#             #                         #",
            "#             #                         #",
            "#             #                         #",
            "#                                       #",
            "#########################################",
            "#                                       #",
            "#                 Z                     #",
            "#                                       #",
            "#                                      E#",
            "#########################################",
        ];

        let data =
            filas
                .iter()
                .map(|fila| {
                    fila.chars()
                        .collect::<Vec<char>>()
                })
                .collect();

        Self { data }
    }

    pub fn guardar_txt(
        &self,
        nombre: &str,
    ) {
        let contenido =
            self.data
                .iter()
                .map(|fila| {
                    fila.iter()
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n");

        fs::write(
            nombre,
            contenido,
        )
        .expect(
            "No se pudo guardar el mapa",
        );

        println!(
            "Mapa guardado en {}",
            nombre,
        );
    }

    pub fn buscar_jugador(
        &self,
    ) -> Option<(usize, usize)> {
        for (fila, linea)
            in self.data.iter().enumerate()
        {
            for (columna, celda)
                in linea.iter().enumerate()
            {
                if *celda == 'P' {
                    return Some(
                        (
                            fila,
                            columna,
                        ),
                    );
                }
            }
        }

        None
    }

    pub fn extraer_zombies(
        &mut self,
    ) -> Vec<(f32, f32)> {
        let mut posiciones =
            Vec::new();

        for fila in 0..self.data.len() {
            for columna in 0..self.data[fila].len() {
                if self.data[fila][columna] == 'Z' {
                    let x =
                        columna as f32
                            * TAMANO_CELDA
                            + TAMANO_CELDA / 2.0;

                    let y =
                        fila as f32
                            * TAMANO_CELDA
                            + TAMANO_CELDA / 2.0;

                    posiciones.push(
                        (
                            x,
                            y,
                        ),
                    );

                    println!(
                        "Zombie encontrado: fila={}, columna={}, x={}, y={}",
                        fila,
                        columna,
                        x,
                        y,
                    );

                    self.data[fila][columna] =
                        ' ';
                }
            }
        }

        println!(
            "Total zombies encontrados: {}",
            posiciones.len(),
        );

        posiciones
    }

    pub fn celda(
        &self,
        fila: i32,
        columna: i32,
    ) -> char {
        if fila < 0
            || fila >= self.data.len() as i32
        {
            return '#';
        }

        let fila_actual =
            &self.data[fila as usize];

        if columna < 0
            || columna >= fila_actual.len() as i32
        {
            return '#';
        }

        fila_actual[
            columna as usize
        ]
    }

    pub fn cambiar_celda(
        &mut self,
        fila: i32,
        columna: i32,
        nueva_celda: char,
    ) {
        if fila < 0
            || fila >= self.data.len() as i32
        {
            return;
        }

        if columna < 0
            || columna
                >= self.data[fila as usize].len() as i32
        {
            return;
        }

        self.data[fila as usize][columna as usize] =
            nueva_celda;
    }

    pub fn celda_desde_posicion(
        &self,
        x: f32,
        y: f32,
    ) -> char {
        let columna =
            (x / TAMANO_CELDA)
                .floor() as i32;

        let fila =
            (y / TAMANO_CELDA)
                .floor() as i32;

        self.celda(
            fila,
            columna,
        )
    }

    pub fn es_pared(
        &self,
        x: f32,
        y: f32,
    ) -> bool {
        let celda =
            self.celda_desde_posicion(
                x,
                y,
            );

        matches!(
            celda,
            '#' | 'D'
        )
    }

    pub fn es_bloque_solido(
        &self,
        fila: i32,
        columna: i32,
    ) -> bool {
        matches!(
            self.celda(
                fila,
                columna,
            ),
            '#' | 'D'
        )
    }

    pub fn ancho(
        &self,
    ) -> usize {
        self.data
            .iter()
            .map(|fila| fila.len())
            .max()
            .unwrap_or(0)
    }

    pub fn alto(
        &self,
    ) -> usize {
        self.data.len()
    }
}