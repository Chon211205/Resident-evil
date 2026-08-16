use crate::camera::Camera;
use crate::inventory::Inventory;
use crate::map::{Map, TAMANO_CELDA};
use crate::player::Player;
use crate::puzzle::Puzzle;
use crate::raycaster::lanzar_rayo;
use crate::zombie::Zombie;

use rand::{thread_rng, Rng};
use std::f32::consts::PI;

pub enum InteractionResult {
    None,
    LlaveRecogida,
    MunicionRecogida(i32),
    CuracionRecogida(i32),
    PuertaAbierta,
    PuertaCerrada,
    SalidaNivel,
}

pub enum ShotResult {
    Miss,

    Hit {
        vida_restante: i32,
    },

    Kill,

    KillConLlave,
}

fn normalizar_angulo(
    mut angulo: f32,
) -> f32 {
    while angulo > PI {
        angulo -= 2.0 * PI;
    }

    while angulo < -PI {
        angulo += 2.0 * PI;
    }

    angulo
}

pub fn recoger_objetos_cercanos(
    mapa: &mut Map,
    player: &Player,
    inventory: &mut Inventory,
    _puzzle: &mut Puzzle,
    vida_actual: i32,
) -> InteractionResult {
    let columna =
        (
            player.x
                / TAMANO_CELDA
        )
            .floor()
            as i32;

    let fila =
        (
            player.y
                / TAMANO_CELDA
        )
            .floor()
            as i32;

    let celda =
        mapa.celda(
            fila,
            columna,
        );

    match celda {
        'K' => {
            mapa.cambiar_celda(
                fila,
                columna,
                ' ',
            );

            inventory.recoger_llave();

            InteractionResult::LlaveRecogida
        }

        'A' => {
            mapa.cambiar_celda(
                fila,
                columna,
                ' ',
            );

            InteractionResult::MunicionRecogida(
                8,
            )
        }

        'H' => {
            if vida_actual >= 100 {
                return InteractionResult::None;
            }

            mapa.cambiar_celda(
                fila,
                columna,
                ' ',
            );

            InteractionResult::CuracionRecogida(
                25,
            )
        }

        'X' => {
            InteractionResult::SalidaNivel
        }

        _ => {
            InteractionResult::None
        }
    }
}

pub fn interactuar(
    mapa: &mut Map,
    player: &Player,
    camera: &Camera,
    inventory: &mut Inventory,
    _puzzle: &mut Puzzle,
) -> InteractionResult {
    let distancias = [
        15.0,
        20.0,
        25.0,
        30.0,
        35.0,
        40.0,
    ];

    for distancia in distancias {
        let objetivo_x =
            player.x
                + camera.angle.cos()
                    * distancia;

        let objetivo_y =
            player.y
                + camera.angle.sin()
                    * distancia;

        let columna =
            (
                objetivo_x
                    / TAMANO_CELDA
            )
                .floor()
                as i32;

        let fila =
            (
                objetivo_y
                    / TAMANO_CELDA
            )
                .floor()
                as i32;

        let celda =
            mapa.celda(
                fila,
                columna,
            );

        if celda == 'D' {
            if inventory.tiene_llave() {
                mapa.cambiar_celda(
                    fila,
                    columna,
                    'O',
                );

                inventory.usar_llave();

                return InteractionResult::PuertaAbierta;
            }

            return InteractionResult::PuertaCerrada;
        }
    }

    InteractionResult::None
}

pub fn disparar(
    zombies: &mut Vec<Zombie>,
    player: &Player,
    camera: &Camera,
    mapa: &mut Map,
) -> ShotResult {
    const DANO: i32 =
        50;

    const TOLERANCIA: f32 =
        0.08;

    let mut objetivo:
        Option<(usize, f32)> =
        None;

    for (
        indice,
        zombie,
    ) in zombies
        .iter()
        .enumerate()
    {
        if !zombie.vivo {
            continue;
        }

        let dx =
            zombie.x
                - player.x;

        let dy =
            zombie.y
                - player.y;

        let distancia =
            (
                dx * dx
                    + dy * dy
            )
                .sqrt();

        if distancia <= 0.001 {
            continue;
        }

        let angulo_zombie =
            dy.atan2(
                dx,
            );

        let diferencia =
            normalizar_angulo(
                angulo_zombie
                    - camera.angle,
            );

        if diferencia.abs()
            > TOLERANCIA
        {
            continue;
        }

        let impacto_pared =
            lanzar_rayo(
                mapa,
                player.x,
                player.y,
                angulo_zombie,
            );

        if impacto_pared.distancia
            < distancia - 4.0
        {
            continue;
        }

        match objetivo {
            Some((
                _,
                distancia_actual,
            )) => {
                if distancia
                    < distancia_actual
                {
                    objetivo =
                        Some((
                            indice,
                            distancia,
                        ));
                }
            }

            None => {
                objetivo =
                    Some((
                        indice,
                        distancia,
                    ));
            }
        }
    }

    let Some((
        indice,
        _,
    )) = objetivo
    else {
        return ShotResult::Miss;
    };

    zombies[indice]
        .recibir_dano(
            DANO,
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

pub fn procesar_muerte_zombie(
    zombie: &Zombie,
    mapa: &mut Map,
) -> ShotResult {
    let columna =
        (
            zombie.x
                / TAMANO_CELDA
        )
            .floor()
            as i32;

    let fila =
        (
            zombie.y
                / TAMANO_CELDA
        )
            .floor()
            as i32;

    if zombie
        .puede_dropear_llave
    {
        mapa.cambiar_celda(
            fila,
            columna,
            'K',
        );

        return ShotResult::KillConLlave;
    }

    let mut rng =
        thread_rng();

    let probabilidad:
        f32 =
        rng.gen();

    if probabilidad < 0.35 {
        mapa.cambiar_celda(
            fila,
            columna,
            'H',
        );
    } else if probabilidad < 0.70 {
        mapa.cambiar_celda(
            fila,
            columna,
            'A',
        );
    }

    ShotResult::Kill
}