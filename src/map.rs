use std::fs;

pub const TAMANO_CELDA: f32 = 25.0;

#[derive(Clone, Copy)]
pub enum TipoSpawnZombie {
    Normal,
    ConLlave,
    Medio,
    Fuerte,
}

pub struct Map {
    celdas: Vec<Vec<char>>,
    ancho: usize,
    alto: usize,
}

impl Map {
    pub fn new() -> Self {
        let contenido =
            include_str!("../mapa_resident.txt");

        let lineas: Vec<&str> =
            contenido
                .lines()
                .map(|linea| {
                    linea.trim_end_matches('\r')
                })
                .collect();

        let alto =
            lineas.len();

        let ancho =
            lineas
                .iter()
                .map(|linea| {
                    linea.chars().count()
                })
                .max()
                .unwrap_or(0);

        let mut celdas =
            Vec::with_capacity(alto);

        for linea in lineas {
            let mut fila: Vec<char> =
                linea.chars().collect();

            while fila.len() < ancho {
                fila.push(' ');
            }

            celdas.push(fila);
        }

        Self {
            celdas,
            ancho,
            alto,
        }
    }

    pub fn ancho(&self) -> usize {
        self.ancho
    }

    pub fn alto(&self) -> usize {
        self.alto
    }

    pub fn celda(
        &self,
        fila: i32,
        columna: i32,
    ) -> char {
        if fila < 0
            || columna < 0
            || fila >= self.alto as i32
            || columna >= self.ancho as i32
        {
            return '#';
        }

        self.celdas
            [fila as usize]
            [columna as usize]
    }

    pub fn cambiar_celda(
        &mut self,
        fila: i32,
        columna: i32,
        valor: char,
    ) {
        if fila < 0
            || columna < 0
            || fila >= self.alto as i32
            || columna >= self.ancho as i32
        {
            return;
        }

        self.celdas
            [fila as usize]
            [columna as usize] =
            valor;
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

    pub fn es_pared(
        &self,
        x: f32,
        y: f32,
    ) -> bool {
        let columna =
            (x / TAMANO_CELDA)
                .floor() as i32;

        let fila =
            (y / TAMANO_CELDA)
                .floor() as i32;

        self.es_bloque_solido(
            fila,
            columna,
        )
    }

    pub fn buscar_jugador(
        &self,
    ) -> Option<(f32, f32)> {
        for fila in 0..self.alto {
            for columna in 0..self.ancho {
                if self.celdas
                    [fila]
                    [columna]
                    == 'P'
                {
                    let x =
                        columna as f32
                            * TAMANO_CELDA
                            + TAMANO_CELDA
                                / 2.0;

                    let y =
                        fila as f32
                            * TAMANO_CELDA
                            + TAMANO_CELDA
                                / 2.0;

                    return Some((
                        x,
                        y,
                    ));
                }
            }
        }

        None
    }

    pub fn extraer_zombies(
        &mut self,
    ) -> Vec<(
        f32,
        f32,
        TipoSpawnZombie,
    )> {
        let mut zombies =
            Vec::new();

        for fila in 0..self.alto {
            for columna in 0..self.ancho {
                let celda =
                    self.celdas
                        [fila]
                        [columna];

                let tipo =
                    match celda {
                        'Z' => {
                            Some(
                                TipoSpawnZombie::Normal,
                            )
                        }

                        'L' => {
                            Some(
                                TipoSpawnZombie::ConLlave,
                            )
                        }

                        'M' => {
                            Some(
                                TipoSpawnZombie::Medio,
                            )
                        }

                        'F' => {
                            Some(
                                TipoSpawnZombie::Fuerte,
                            )
                        }

                        _ => None,
                    };

                let Some(tipo) =
                    tipo
                else {
                    continue;
                };

                let x =
                    columna as f32
                        * TAMANO_CELDA
                        + TAMANO_CELDA
                            / 2.0;

                let y =
                    fila as f32
                        * TAMANO_CELDA
                        + TAMANO_CELDA
                            / 2.0;

                zombies.push((
                    x,
                    y,
                    tipo,
                ));

                self.celdas
                    [fila]
                    [columna] =
                    ' ';
            }
        }

        zombies
    }

    pub fn guardar_txt(
        &self,
        ruta: &str,
    ) {
        let contenido =
            self.celdas
                .iter()
                .map(|fila| {
                    fila
                        .iter()
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n");

        if let Err(error) =
            fs::write(
                ruta,
                contenido,
            )
        {
            eprintln!(
                "Error guardando mapa: {}",
                error,
            );
        }
    }
}