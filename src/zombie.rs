use crate::map::Map;
use crate::player::Player;
use crate::raycaster::lanzar_rayo;

pub struct Zombie {
    pub x: f32,
    pub y: f32,

    pub vida: i32,
    pub velocidad: f32,
    pub vivo: bool,

    pub persiguiendo: bool,
    pub tiempo_animacion: f32,

    tiempo_ultimo_ataque: f32,
}

impl Zombie {
    pub fn new(
        x: f32,
        y: f32,
    ) -> Self {
        Self {
            x,
            y,

            vida: 100,
            velocidad: 22.0,
            vivo: true,

            persiguiendo: false,
            tiempo_animacion: 0.0,

            tiempo_ultimo_ataque: 0.0,
        }
    }

    pub fn update(
        &mut self,
        player: &Player,
        mapa: &Map,
        delta_time: f32,
    ) -> i32 {
        if !self.vivo {
            self.persiguiendo = false;
            return 0;
        }

        self.tiempo_ultimo_ataque +=
            delta_time;

        let dx =
            player.x - self.x;

        let dy =
            player.y - self.y;

        let distancia =
            (dx * dx + dy * dy)
                .sqrt();

        let distancia_deteccion =
            250.0;

        let distancia_ataque =
            22.0;

        // Si está demasiado lejos,
        // se queda quieto.
        if distancia
            > distancia_deteccion
        {
            self.persiguiendo = false;
            self.tiempo_animacion = 0.0;

            return 0;
        }

        // Si hay una pared o puerta
        // entre el zombie y el jugador,
        // no puede verlo.
        if !self.puede_ver_jugador(
            player,
            mapa,
            distancia,
        ) {
            self.persiguiendo = false;
            self.tiempo_animacion = 0.0;

            return 0;
        }

        // Si llegó aquí,
        // el zombie puede ver al jugador.
        self.persiguiendo = true;

        self.tiempo_animacion +=
            delta_time;

        // Si está suficientemente cerca,
        // ataca.
        if distancia
            <= distancia_ataque
        {
            if self.tiempo_ultimo_ataque
                >= 1.0
            {
                self.tiempo_ultimo_ataque =
                    0.0;

                return 10;
            }

            return 0;
        }

        if distancia <= 0.001 {
            return 0;
        }

        let dir_x =
            dx / distancia;

        let dir_y =
            dy / distancia;

        let nuevo_x =
            self.x
                + dir_x
                    * self.velocidad
                    * delta_time;

        let nuevo_y =
            self.y
                + dir_y
                    * self.velocidad
                    * delta_time;

        // Movimiento separado en X/Y
        // para que pueda deslizarse
        // por las paredes.
        if !mapa.es_pared(
            nuevo_x,
            self.y,
        ) {
            self.x =
                nuevo_x;
        }

        if !mapa.es_pared(
            self.x,
            nuevo_y,
        ) {
            self.y =
                nuevo_y;
        }

        0
    }

    fn puede_ver_jugador(
        &self,
        player: &Player,
        mapa: &Map,
        distancia_jugador: f32,
    ) -> bool {
        let angulo =
            (player.y - self.y)
                .atan2(
                    player.x - self.x,
                );

        let hit =
            lanzar_rayo(
                mapa,
                self.x,
                self.y,
                angulo,
            );

        hit.distancia
            >= distancia_jugador - 5.0
    }

    pub fn recibir_dano(
        &mut self,
        dano: i32,
    ) {
        if !self.vivo {
            return;
        }

        self.vida -=
            dano;

        if self.vida <= 0 {
            self.vida = 0;
            self.vivo = false;
            self.persiguiendo = false;
        }
    }
}