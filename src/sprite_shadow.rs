//! Sombras de contacto dibujadas bajo los sprites del primer piso.
use crate::{camera::Camera, licker::{EstadoLicker, Licker}, map::{Map, TAMANO_CELDA}, player::Player, raycaster::lanzar_rayo, tyrant::Tyrant, zombie::Zombie};
use raylib::prelude::*;

fn draw_shadow(
    drawing: &mut RaylibDrawHandle,
    map: &Map,
    player: &Player,
    camera: &Camera,
    world_x: f32,
    world_y: f32,
    world_radius: f32,
    offset_x: f32,
    offset_y: f32,
    scale: f32,
) {
    let dx = world_x - player.x;
    let dy = world_y - player.y;
    let distance = (dx * dx + dy * dy).sqrt();
    if distance <= 1.0 { return; }
    let angle = dy.atan2(dx);
    if lanzar_rayo(map, player.x, player.y, angle).distancia < distance - world_radius {
        return;
    }
    let Some((screen_x, screen_y, depth)) = camera.project_ground(dx, dy) else { return; };
    if !(-80.0..=880.0).contains(&screen_x) || !(-30.0..=650.0).contains(&screen_y) {
        return;
    }
    let radius = (world_radius * Camera::focal_length() / depth * scale).clamp(2.0, 70.0);
    let x = (offset_x + screen_x * scale).round() as i32;
    let y = (offset_y + screen_y * scale).round() as i32;
    // Varias elipses translúcidas evitan un borde negro duro.
    for (size, opacity) in [(1.5, 20), (1.15, 28), (0.78, 42)] {
        drawing.draw_ellipse(x, y, radius * size, radius * size * 0.27, Color::new(0, 0, 0, opacity));
    }
}

pub fn render_sprite_shadows(
    drawing: &mut RaylibDrawHandle,
    map: &Map,
    player: &Player,
    camera: &Camera,
    zombies: &[Zombie],
    lickers: &[Licker],
    tyrant: Option<&Tyrant>,
    nemesis: Option<&Tyrant>,
    offset_x: f32,
    offset_y: f32,
    scale: f32,
) {
    if !camera.use_3d_projection { return; }
    for row in 0..map.alto() {
        for col in 0..map.ancho() {
            if matches!(map.celda(row as i32, col as i32), 'K' | 'A' | 'H' | 'Q' | 'V' | 'I' | 'E') {
                draw_shadow(drawing, map, player, camera,
                    (col as f32 + 0.5) * TAMANO_CELDA,
                    (row as f32 + 0.5) * TAMANO_CELDA,
                    2.8, offset_x, offset_y, scale);
            }
        }
    }
    for zombie in zombies.iter().filter(|zombie| zombie.vivo) {
        draw_shadow(drawing, map, player, camera, zombie.x, zombie.y, 4.2, offset_x, offset_y, scale);
    }
    for licker in lickers.iter().filter(|licker| licker.vivo && licker.estado == EstadoLicker::Suelo) {
        draw_shadow(drawing, map, player, camera, licker.x, licker.y, 3.5, offset_x, offset_y, scale);
    }
    for boss in [tyrant, nemesis].into_iter().flatten() {
        draw_shadow(drawing, map, player, camera, boss.x, boss.y, 5.0, offset_x, offset_y, scale);
    }
}
