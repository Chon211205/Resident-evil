use crate::camera::Camera;
use crate::inventory::Inventory;
use crate::map::{Map, TAMANO_CELDA};
use crate::player::Player;
use crate::puzzle::Puzzle;

pub enum InteractionResult {
    None,
    LlaveRecogida,
    PuertaAbierta,
    PuertaCerrada,
}

pub fn interactuar(
    mapa: &mut Map,
    player: &Player,
    camera: &Camera,
    inventory: &mut Inventory,
    puzzle: &mut Puzzle,
) -> InteractionResult {
    let distancia_interaccion =
        TAMANO_CELDA * 1.5;

    let frente_x =
        player.x
            + camera.angle.cos()
                * distancia_interaccion;

    let frente_y =
        player.y
            + camera.angle.sin()
                * distancia_interaccion;

    let columna =
        (frente_x / TAMANO_CELDA)
            .floor() as i32;

    let fila =
        (frente_y / TAMANO_CELDA)
            .floor() as i32;

    let objeto =
        mapa.celda(
            fila,
            columna,
        );

    match objeto {
        'D' => {
            if inventory.tiene_llave() {
                inventory.usar_llave();
                puzzle.abrir_puerta();

                mapa.cambiar_celda(
                    fila,
                    columna,
                    'O',
                );

                InteractionResult::PuertaAbierta
            } else {
                InteractionResult::PuertaCerrada
            }
        }

        _ => InteractionResult::None,
    }
}

pub fn recoger_objetos_cercanos(
    mapa: &mut Map,
    player: &Player,
    inventory: &mut Inventory,
    puzzle: &mut Puzzle,
) -> InteractionResult {
    let columna =
        (player.x / TAMANO_CELDA)
            .floor() as i32;

    let fila =
        (player.y / TAMANO_CELDA)
            .floor() as i32;

    let objeto =
        mapa.celda(
            fila,
            columna,
        );

    match objeto {
        'K' => {
            inventory.recoger_llave();
            puzzle.recoger_llave();

            mapa.cambiar_celda(
                fila,
                columna,
                ' ',
            );

            InteractionResult::LlaveRecogida
        }

        _ => InteractionResult::None,
    }
}