use std::fs;

pub const TAMANO_CELDA: f32 = 25.0;

pub struct Map {
    pub data: Vec<String>,
}

impl Map {
    pub fn new() -> Self {
        let data = vec![
            "#########################################".to_string(),
            "#P       #             #               #".to_string(),
            "#        #             #               #".to_string(),
            "#        #             #               #".to_string(),
            "#        #             #               #".to_string(),
            "#### ########## ############### ########".to_string(),
            "#              #               #       #".to_string(),
            "#              #               #       #".to_string(),
            "#              #               #       #".to_string(),
            "#              #               #       #".to_string(),
            "####### ########## ########## ##########".to_string(),
            "#     #           #          #         #".to_string(),
            "#     #           #          #         #".to_string(),
            "#     #           #          #         #".to_string(),
            "### ######## ###### ########## #########".to_string(),
            "#           #                         ##".to_string(),
            "#           #                          #".to_string(),
            "#           #                          #".to_string(),
            "#           #                          #".to_string(),
            "###### ############## ##################".to_string(),
            "#             #                        #".to_string(),
            "#             #                        #".to_string(),
            "#             #                        #".to_string(),
            "#             #                        #".to_string(),
            "###### ########### #####################".to_string(),
            "#               #                      #".to_string(),
            "#               #                      #".to_string(),
            "#               #                      #".to_string(),
            "#               #                      #".to_string(),
            "###### ########### #####################".to_string(),
            "#             #                        #".to_string(),
            "#             #                        #".to_string(),
            "#             #                        #".to_string(),
            "#             #                        #".to_string(),
            "###### ############# ###################".to_string(),
            "#                   #                  #".to_string(),
            "#                   #                  #".to_string(),
            "#                   #                  #".to_string(),
            "#                   #                 E#".to_string(),
            "#########################################".to_string(),
        ];

        Self { data }
    }

    pub fn guardar_txt(&self, nombre: &str) {
        fs::write(
            nombre,
            self.data.join("\n"),
        )
        .expect("No se pudo guardar el mapa");

        println!("Mapa guardado en {}", nombre);
    }

    pub fn buscar_jugador(&self) -> Option<(usize, usize)> {
        for (fila, linea) in self.data.iter().enumerate() {
            for (columna, celda) in linea.chars().enumerate() {
                if celda == 'P' {
                    return Some((fila, columna));
                }
            }
        }

        None
    }

    pub fn celda(&self, fila: i32, columna: i32) -> char {
        if fila < 0 || fila >= self.data.len() as i32 {
            return '#';
        }

        let linea = &self.data[fila as usize];

        if columna < 0
            || columna >= linea.chars().count() as i32
        {
            return '#';
        }

        linea
            .chars()
            .nth(columna as usize)
            .unwrap_or('#')
    }

    pub fn celda_desde_posicion(
        &self,
        x: f32,
        y: f32,
    ) -> char {
        let columna =
            (x / TAMANO_CELDA).floor() as i32;

        let fila =
            (y / TAMANO_CELDA).floor() as i32;

        self.celda(fila, columna)
    }

    pub fn es_pared(
        &self,
        x: f32,
        y: f32,
    ) -> bool {
        self.celda_desde_posicion(x, y) == '#'
    }

    pub fn ancho(&self) -> usize {
        self.data
            .iter()
            .map(|fila| fila.chars().count())
            .max()
            .unwrap_or(0)
    }

    pub fn alto(&self) -> usize {
        self.data.len()
    }
}