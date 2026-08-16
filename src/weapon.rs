use crate::camera::Camera;
use crate::interaction::{
    procesar_muerte_zombie,
    ShotResult,
};
use crate::map::Map;
use crate::player::Player;
use crate::raycaster::lanzar_rayo;
use crate::zombie::Zombie;

use std::f32::consts::PI;

#[derive(PartialEq, Clone, Copy)]
pub enum ArmaActual {
    Pistola,
    Hacha,
}

pub fn atacar_con_hacha(
    zombies: &mut [Zombie],
    player: &Player,
    camera: &Camera,
    mapa: &mut Map,
) -> ShotResult {
    let alcance = 30.0;
    let tolerancia = 0.30;

    let mut objetivo:
        Option<(usize, f32)> =
        None;

    for (indice, zombie)
        in zombies.iter().enumerate()
    {
        if !zombie.vivo {
            continue;
        }

        let dx =
            zombie.x - player.x;

        let dy =
            zombie.y - player.y;

        let distancia =
            (dx * dx + dy * dy)
                .sqrt();

        if distancia > alcance {
            continue;
        }

        let angulo_zombie =
            dy.atan2(dx);

        let diferencia =
            normalizar_diferencia(
                angulo_zombie
                    - camera.angle,
            );

        if diferencia.abs()
            > tolerancia
        {
            continue;
        }

        let hit_pared =
            lanzar_rayo(
                mapa,
                player.x,
                player.y,
                angulo_zombie,
            );

        if hit_pared.distancia
            < distancia - 3.0
        {
            continue;
        }

        match objetivo {
            None => {
                objetivo =
                    Some((
                        indice,
                        distancia,
                    ));
            }

            Some((
                _,
                mejor_distancia,
            )) => {
                if distancia
                    < mejor_distancia
                {
                    objetivo =
                        Some((
                            indice,
                            distancia,
                        ));
                }
            }
        }
    }

    let Some((indice, _)) =
        objetivo
    else {
        return ShotResult::Miss;
    };

    zombies[indice]
        .recibir_dano(
            75,
        );

    if zombies[indice].vivo {
        return ShotResult::Hit {
            vida_restante:
                zombies[indice].vida,
        };
    }

    procesar_muerte_zombie(
        &zombies[indice],
        mapa,
    )
}

pub fn puede_bloquear_ataque(
    zombie: &Zombie,
    player: &Player,
    camera: &Camera,
) -> bool {
    if !zombie.vivo {
        return false;
    }

    let dx =
        zombie.x - player.x;

    let dy =
        zombie.y - player.y;

    let angulo_zombie =
        dy.atan2(dx);

    let diferencia =
        normalizar_diferencia(
            angulo_zombie
                - camera.angle,
        );

    let angulo_bloqueo =
        0.65;

    diferencia.abs()
        <= angulo_bloqueo
}

fn normalizar_diferencia(
    mut angulo: f32,
) -> f32 {
    while angulo > PI {
        angulo -=
            2.0 * PI;
    }

    while angulo < -PI {
        angulo +=
            2.0 * PI;
    }

    angulo
}