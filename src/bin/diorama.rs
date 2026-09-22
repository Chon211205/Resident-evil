//! Prototipo independiente: `cargo run --release --bin diorama`.
//! Habitación de prueba con paredes de madera y una ventana refractante.

#[path = "../framebuffer.rs"]
mod framebuffer;
#[path = "../texture_data.rs"]
mod texture_data;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use texture_data::TextureData;

const WIDTH: i32 = 320;
const HEIGHT: i32 = 180;
const ROOM_WIDTH: f32 = 10.0;
const ROOM_DEPTH: f32 = 8.0;
const ROOM_HEIGHT: f32 = 3.0;
const EPS: f32 = 0.001;
const MAX_DEPTH: u8 = 2;

#[derive(Clone, Copy, Debug, Default)]
struct Vec3 { x: f32, y: f32, z: f32 }

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    fn dot(self, b: Self) -> f32 { self.x * b.x + self.y * b.y + self.z * b.z }
    fn cross(self, b: Self) -> Self {
        Self::new(self.y * b.z - self.z * b.y, self.z * b.x - self.x * b.z, self.x * b.y - self.y * b.x)
    }
    fn length(self) -> f32 { self.dot(self).sqrt() }
    fn unit(self) -> Self { self * (1.0 / self.length().max(0.000001)) }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, b: Self) -> Self { Self::new(self.x + b.x, self.y + b.y, self.z + b.z) }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, b: Self) -> Self { Self::new(self.x - b.x, self.y - b.y, self.z - b.z) }
}
impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, t: f32) -> Self { Self::new(self.x * t, self.y * t, self.z * t) }
}

#[derive(Clone, Copy)]
struct Ray { origin: Vec3, dir: Vec3 }

#[derive(Clone, Copy, PartialEq, Eq)]
enum Surface { Wall, Glass, Metal, Door, Floor }

struct Block { min: Vec3, max: Vec3, surface: Surface }

/// Pirámide de texturas: cada nivel promedia cuatro texeles del anterior.
struct FilteredTexture { levels: Vec<TextureData> }

impl FilteredTexture {
    fn new(base: TextureData) -> Self {
        let mut levels = vec![base];
        while levels.last().is_some_and(|level| level.width > 1 || level.height > 1) {
            let previous = levels.last().unwrap();
            let width = (previous.width / 2).max(1);
            let height = (previous.height / 2).max(1);
            let mut pixels = Vec::with_capacity((width * height) as usize);
            for y in 0..height {
                for x in 0..width {
                    let a = previous.get_pixel(x * 2, y * 2);
                    let b = previous.get_pixel(x * 2 + 1, y * 2);
                    let c = previous.get_pixel(x * 2, y * 2 + 1);
                    let d = previous.get_pixel(x * 2 + 1, y * 2 + 1);
                    pixels.push(Color::new(
                        ((a.r as u16 + b.r as u16 + c.r as u16 + d.r as u16) / 4) as u8,
                        ((a.g as u16 + b.g as u16 + c.g as u16 + d.g as u16) / 4) as u8,
                        ((a.b as u16 + b.b as u16 + c.b as u16 + d.b as u16) / 4) as u8,
                        255,
                    ));
                }
            }
            levels.push(TextureData { width, height, pixels });
        }
        Self { levels }
    }

    fn sample_level(&self, u: f32, v: f32, level: usize) -> Vec3 {
        let texture = &self.levels[level];
        let fx = u.rem_euclid(1.0) * texture.width as f32 - 0.5;
        let fy = v.rem_euclid(1.0) * texture.height as f32 - 0.5;
        let x0 = fx.floor() as i32;
        let y0 = fy.floor() as i32;
        let tx = fx - x0 as f32;
        let ty = fy - y0 as f32;
        let pixel = |x: i32, y: i32| {
            let color = texture.get_pixel(x.rem_euclid(texture.width), y.rem_euclid(texture.height));
            Vec3::new(color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0)
        };
        let top = pixel(x0, y0) * (1.0 - tx) + pixel(x0 + 1, y0) * tx;
        let bottom = pixel(x0, y0 + 1) * (1.0 - tx) + pixel(x0 + 1, y0 + 1) * tx;
        top * (1.0 - ty) + bottom * ty
    }

    fn sample(&self, u: f32, v: f32, lod: f32) -> Vec3 {
        let level = lod.clamp(0.0, (self.levels.len() - 1) as f32);
        let lower = level.floor() as usize;
        let upper = (lower + 1).min(self.levels.len() - 1);
        let blend = level - lower as f32;
        self.sample_level(u, v, lower) * (1.0 - blend)
            + self.sample_level(u, v, upper) * blend
    }

    fn lod_for_hit(&self, hit: Hit, ray: Ray) -> f32 {
        let projected_pixel = 2.0 * hit.t * (55.0_f32.to_radians() * 0.5).tan() / HEIGHT as f32;
        let angle = ray.dir.dot(hit.normal).abs().max(0.20);
        let repeat_scale = if hit.surface == Surface::Floor { 0.5 } else { 1.0 };
        let texels_per_pixel = projected_pixel / angle * repeat_scale * self.levels[0].width.max(self.levels[0].height) as f32;
        (texels_per_pixel.max(1.0).log2() - 0.5).max(0.0)
    }
}

struct Material<'a> {
    texture: &'a FilteredTexture,
    normal_map: Option<&'a FilteredTexture>,
    roughness_map: Option<&'a FilteredTexture>,
    albedo: Vec3,
    specular: f32,
    transparency: f32,
    reflectivity: f32,
    ior: f32,
}

struct Materials<'a> {
    wall: Material<'a>,
    glass: Material<'a>,
    metal: Material<'a>,
    door: Material<'a>,
    floor: Material<'a>,
    sky: &'a FilteredTexture,
}

impl Materials<'_> {
    fn get(&self, surface: Surface) -> &Material<'_> {
        match surface {
            Surface::Wall => &self.wall,
            Surface::Glass => &self.glass,
            Surface::Metal => &self.metal,
            Surface::Door => &self.door,
            Surface::Floor => &self.floor,
        }
    }
}

#[derive(Clone, Copy)]
struct Hit {
    t: f32,
    point: Vec3,
    normal: Vec3,
    surface: Surface,
    block_index: Option<usize>,
}

fn scene() -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut add = |min: Vec3, max: Vec3, surface: Surface| {
        blocks.push(Block { min, max, surface });
    };

    // Base y cuatro paredes. El techo abierto permite mirar dentro al orbitar.
    add(Vec3::new(0.0, -0.3, 0.0), Vec3::new(ROOM_WIDTH, 0.0, ROOM_DEPTH), Surface::Wall);
    add(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.25, ROOM_HEIGHT, ROOM_DEPTH), Surface::Wall);
    add(Vec3::new(ROOM_WIDTH - 0.25, 0.0, 0.0), Vec3::new(ROOM_WIDTH, ROOM_HEIGHT, ROOM_DEPTH), Surface::Wall);
    add(Vec3::new(0.0, 0.0, ROOM_DEPTH - 0.25), Vec3::new(ROOM_WIDTH, ROOM_HEIGHT, ROOM_DEPTH), Surface::Wall);

    // La pared frontal deja un hueco real entre el antepecho y el dintel.
    add(Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, ROOM_HEIGHT, 0.25), Surface::Wall);
    add(Vec3::new(7.0, 0.0, 0.0), Vec3::new(ROOM_WIDTH, ROOM_HEIGHT, 0.25), Surface::Wall);
    add(Vec3::new(3.0, 0.0, 0.0), Vec3::new(7.0, 1.0, 0.25), Surface::Wall);
    add(Vec3::new(3.0, 2.2, 0.0), Vec3::new(7.0, ROOM_HEIGHT, 0.25), Surface::Wall);

    // Marco metálico delante de una lámina fina de vidrio.
    add(Vec3::new(3.0, 1.0, 0.03), Vec3::new(3.12, 2.2, 0.22), Surface::Metal);
    add(Vec3::new(6.88, 1.0, 0.03), Vec3::new(7.0, 2.2, 0.22), Surface::Metal);
    add(Vec3::new(3.12, 1.0, 0.03), Vec3::new(6.88, 1.12, 0.22), Surface::Metal);
    add(Vec3::new(3.12, 2.08, 0.03), Vec3::new(6.88, 2.2, 0.22), Surface::Metal);
    add(Vec3::new(4.96, 1.12, 0.03), Vec3::new(5.04, 2.08, 0.22), Surface::Metal);
    add(Vec3::new(3.12, 1.12, 0.12), Vec3::new(6.88, 2.08, 0.16), Surface::Glass);

    // Objetos sencillos dentro de la habitación para verlos a través del vidrio.
    add(Vec3::new(7.9, 0.0, 7.52), Vec3::new(9.2, 2.35, 7.74), Surface::Door);
    add(Vec3::new(2.0, 0.0, 4.0), Vec3::new(2.9, 0.9, 4.9), Surface::Metal);
    add(Vec3::new(4.5, 0.0, 3.3), Vec3::new(5.5, 1.0, 4.3), Surface::Metal);
    add(Vec3::new(6.5, 0.0, 4.7), Vec3::new(7.4, 0.9, 5.6), Surface::Metal);
    blocks
}

fn intersect_box(ray: Ray, block: &Block, index: usize) -> Option<Hit> {
    let mut near = f32::NEG_INFINITY;
    let mut far = f32::INFINITY;
    let mut near_normal = Vec3::default();
    for axis in 0..3 {
        let (o, d, lo, hi, negative, positive) = match axis {
            0 => (ray.origin.x, ray.dir.x, block.min.x, block.max.x, Vec3::new(-1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
            1 => (ray.origin.y, ray.dir.y, block.min.y, block.max.y, Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            _ => (ray.origin.z, ray.dir.z, block.min.z, block.max.z, Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0)),
        };
        if d.abs() < 0.000001 {
            if o < lo || o > hi { return None; }
            continue;
        }
        let (t0, t1, n0) = if d > 0.0 {
            ((lo - o) / d, (hi - o) / d, negative)
        } else {
            ((hi - o) / d, (lo - o) / d, positive)
        };
        if t0 > near { near = t0; near_normal = n0; }
        if t1 < far { far = t1; }
        if near > far { return None; }
    }
    if far <= EPS || near <= EPS { return None; }
    Some(Hit {
        t: near,
        point: ray.origin + ray.dir * near,
        normal: near_normal,
        surface: block.surface,
        block_index: Some(index),
    })
}

fn intersect_floor(ray: Ray) -> Option<Hit> {
    if ray.dir.y >= -0.000001 { return None; }
    let t = -ray.origin.y / ray.dir.y;
    let point = ray.origin + ray.dir * t;
    if t <= EPS || point.x < 0.0 || point.x >= ROOM_WIDTH || point.z < 0.0 || point.z >= ROOM_DEPTH {
        return None;
    }
    Some(Hit { t, point, normal: Vec3::new(0.0, 1.0, 0.0), surface: Surface::Floor, block_index: None })
}

fn closest_hit(ray: Ray, blocks: &[Block]) -> Option<Hit> {
    let mut closest = intersect_floor(ray);
    for (index, block) in blocks.iter().enumerate() {
        if let Some(hit) = intersect_box(ray, block, index) {
            if closest.as_ref().is_none_or(|old| hit.t < old.t) { closest = Some(hit); }
        }
    }
    closest
}

fn reflect(dir: Vec3, normal: Vec3) -> Vec3 { (dir - normal * (2.0 * dir.dot(normal))).unit() }

fn refract(dir: Vec3, normal: Vec3, from: f32, to: f32) -> Option<Vec3> {
    let eta = from / to;
    let cos_i = -dir.dot(normal);
    let k = 1.0 - eta * eta * (1.0 - cos_i * cos_i);
    if k < 0.0 { None } else { Some((dir * eta + normal * (eta * cos_i - k.sqrt())).unit()) }
}

fn sky(dir: Vec3, texture: &FilteredTexture) -> Vec3 {
    let u = dir.z.atan2(dir.x).rem_euclid(std::f32::consts::TAU) / std::f32::consts::TAU;
    let v = 0.5 - dir.y.clamp(-1.0, 1.0).asin() / std::f32::consts::PI;
    let base = &texture.levels[0];
    let fov = 55.0_f32.to_radians();
    let horizontal = fov / std::f32::consts::TAU * base.width as f32 / WIDTH as f32;
    let vertical = fov / std::f32::consts::PI * base.height as f32 / HEIGHT as f32;
    let lod = horizontal.max(vertical).max(1.0).log2();
    texture.sample(u, v.clamp(0.0, 0.9999), lod)
}

fn surface_uv(hit: Hit) -> (f32, f32) {
    if hit.surface == Surface::Floor { return (hit.point.x * 0.5, hit.point.z * 0.5); }
    let height_scale = if hit.surface == Surface::Door { 2.35 } else { 1.0 };
    if hit.normal.x.abs() > 0.5 { (hit.point.z, 1.0 - hit.point.y / height_scale) }
    else if hit.normal.z.abs() > 0.5 { (hit.point.x, 1.0 - hit.point.y / height_scale) }
    else { (hit.point.x, hit.point.z) }
}

fn mapped_normal(hit: Hit, normal_texture: &FilteredTexture, u: f32, v: f32, lod: f32) -> Vec3 {
    let encoded = normal_texture.sample(u, v, lod);
    let tangent_space = Vec3::new(encoded.x * 2.0 - 1.0, encoded.y * 2.0 - 1.0, encoded.z * 2.0 - 1.0);
    let tangent = if hit.normal.x.abs() > 0.5 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let bitangent = if hit.surface == Surface::Floor {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    (tangent * tangent_space.x + bitangent * tangent_space.y + hit.normal * tangent_space.z).unit()
}

fn trace(ray: Ray, blocks: &[Block], materials: &Materials<'_>, depth: u8) -> Vec3 {
    let Some(hit) = closest_hit(ray, blocks) else { return sky(ray.dir, materials.sky); };
    let mat = materials.get(hit.surface);
    let (u, v) = surface_uv(hit);
    let lod = mat.texture.lod_for_hit(hit, ray);
    let texel = mat.texture.sample(u, v, lod);
    let shading_normal = mat.normal_map.map_or(hit.normal, |map| mapped_normal(hit, map, u, v, lod));
    let roughness = mat.roughness_map.map_or(0.55, |map| map.sample(u, v, lod).x).clamp(0.04, 1.0);
    let light = Vec3::new(0.45, 0.85, -0.30).unit();
    let diffuse = shading_normal.dot(light).max(0.0);
    let half = (light - ray.dir).unit();
    let shininess = 8.0 + (1.0 - roughness) * 100.0;
    let spec = shading_normal.dot(half).max(0.0).powf(shininess) * mat.specular * (1.0 - roughness);
    let mut color = Vec3::new(
        texel.x * mat.albedo.x,
        texel.y * mat.albedo.y,
        texel.z * mat.albedo.z,
    ) * (0.42 + 0.58 * diffuse) + Vec3::new(spec, spec, spec) * 0.6;

    if depth < MAX_DEPTH && mat.transparency > 0.0 {
        if let Some(inside) = refract(ray.dir, hit.normal, 1.0, mat.ior) {
            // Se intersecta el mismo bloque desde dentro para hallar su cara de salida.
            if let Some(index) = hit.block_index {
                let inside_ray = Ray { origin: hit.point + inside * EPS, dir: inside };
                if let Some(exit) = intersect_exit(inside_ray, &blocks[index]) {
                    if let Some(outside) = refract(inside, exit.1 * -1.0, mat.ior, 1.0) {
                        let next = Ray { origin: exit.0 + outside * EPS, dir: outside };
                        let transmitted = trace(next, blocks, materials, depth + 1);
                        color = color * (1.0 - mat.transparency) + transmitted * mat.transparency;
                    }
                }
            }
        }
    }
    if depth < MAX_DEPTH && mat.reflectivity >= 0.08 {
        let reflected = reflect(ray.dir, shading_normal);
        let next = Ray { origin: hit.point + reflected * EPS, dir: reflected };
        let reflected_color = trace(next, blocks, materials, depth + 1);
        color = color * (1.0 - mat.reflectivity) + reflected_color * mat.reflectivity;
    }
    color
}

fn intersect_exit(ray: Ray, block: &Block) -> Option<(Vec3, Vec3)> {
    let mut best = f32::INFINITY;
    let mut normal = Vec3::default();
    let candidates = [
        (block.min.x, ray.origin.x, ray.dir.x, Vec3::new(-1.0, 0.0, 0.0)),
        (block.max.x, ray.origin.x, ray.dir.x, Vec3::new(1.0, 0.0, 0.0)),
        (block.min.y, ray.origin.y, ray.dir.y, Vec3::new(0.0, -1.0, 0.0)),
        (block.max.y, ray.origin.y, ray.dir.y, Vec3::new(0.0, 1.0, 0.0)),
        (block.min.z, ray.origin.z, ray.dir.z, Vec3::new(0.0, 0.0, -1.0)),
        (block.max.z, ray.origin.z, ray.dir.z, Vec3::new(0.0, 0.0, 1.0)),
    ];
    for (plane, origin, dir, candidate_normal) in candidates {
        if dir.abs() < 0.000001 { continue; }
        let t = (plane - origin) / dir;
        if t <= EPS * 0.5 || t >= best { continue; }
        let point = ray.origin + ray.dir * t;
        if point.x >= block.min.x - EPS && point.x <= block.max.x + EPS
            && point.y >= block.min.y - EPS && point.y <= block.max.y + EPS
            && point.z >= block.min.z - EPS && point.z <= block.max.z + EPS {
            best = t;
            normal = candidate_normal;
        }
    }
    if best.is_finite() { Some((ray.origin + ray.dir * best, normal)) } else { None }
}

fn to_color(c: Vec3) -> Color {
    Color::new(
        ((c.x * 1.15).clamp(0.0, 1.0).powf(0.8) * 255.0) as u8,
        ((c.y * 1.15).clamp(0.0, 1.0).powf(0.8) * 255.0) as u8,
        ((c.z * 1.15).clamp(0.0, 1.0).powf(0.8) * 255.0) as u8,
        255,
    )
}

fn render(buffer: &mut Framebuffer, blocks: &[Block], materials: &Materials<'_>, target: Vec3, yaw: f32, pitch: f32, distance: f32) {
    let camera = target + Vec3::new(yaw.cos() * pitch.cos(), pitch.sin(), yaw.sin() * pitch.cos()) * distance;
    let forward = (target - camera).unit();
    let right = forward.cross(Vec3::new(0.0, 1.0, 0.0)).unit();
    let up = right.cross(forward).unit();
    let aspect = WIDTH as f32 / HEIGHT as f32;
    let tan_fov = (55.0_f32.to_radians() * 0.5).tan();
    for y in 0..HEIGHT {
        let sy = (1.0 - 2.0 * (y as f32 + 0.5) / HEIGHT as f32) * tan_fov;
        for x in 0..WIDTH {
            let sx = (2.0 * (x as f32 + 0.5) / WIDTH as f32 - 1.0) * aspect * tan_fov;
            let dir = (forward + right * sx + up * sy).unit();
            let color = trace(Ray { origin: camera, dir }, blocks, materials, 0);
            buffer.point_color(x, y, to_color(color));
        }
    }
}

fn load(path: &str) -> FilteredTexture {
    let mut image = Image::load_image(path).expect("No se pudo cargar una textura del diorama");
    FilteredTexture::new(TextureData::from_image(&mut image))
}

fn main() {
    let glass = load("assets/textures/window.png");
    let metal = load("assets/textures/metalcrate.png");
    let wood = load("assets/textures/wood035/Wood035_2K-PNG_Color.png");
    let wood_normal = load("assets/textures/wood035/Wood035_2K-PNG_NormalGL.png");
    let wood_roughness = load("assets/textures/wood035/Wood035_2K-PNG_Roughness.png");
    let floor = load("assets/textures/wood051/Wood051_2K-PNG_Color.png");
    let floor_normal = load("assets/textures/wood051/Wood051_2K-PNG_NormalGL.png");
    let floor_roughness = load("assets/textures/wood051/Wood051_2K-PNG_Roughness.png");
    let city = load("assets/textures/citynight.png");
    let materials = Materials {
        wall: Material { texture: &wood, normal_map: Some(&wood_normal), roughness_map: Some(&wood_roughness), albedo: Vec3::new(0.86, 0.82, 0.76), specular: 0.16, transparency: 0.0, reflectivity: 0.02, ior: 1.0 },
        glass: Material { texture: &glass, normal_map: None, roughness_map: None, albedo: Vec3::new(0.65, 0.80, 1.0), specular: 0.80, transparency: 0.75, reflectivity: 0.10, ior: 1.5 },
        metal: Material { texture: &metal, normal_map: None, roughness_map: None, albedo: Vec3::new(0.80, 0.88, 1.0), specular: 0.95, transparency: 0.0, reflectivity: 0.60, ior: 1.0 },
        door: Material { texture: &wood, normal_map: Some(&wood_normal), roughness_map: Some(&wood_roughness), albedo: Vec3::new(1.0, 1.0, 1.0), specular: 0.45, transparency: 0.0, reflectivity: 0.04, ior: 1.0 },
        floor: Material { texture: &floor, normal_map: Some(&floor_normal), roughness_map: Some(&floor_roughness), albedo: Vec3::new(0.96, 0.94, 0.90), specular: 0.32, transparency: 0.0, reflectivity: 0.04, ior: 1.0 },
        sky: &city,
    };
    let blocks = scene();
    let mut buffer = Framebuffer::new(WIDTH, HEIGHT);
    let snapshot_door = std::env::args().any(|arg| arg == "--snapshot-door");
    let snapshot_window = std::env::args().any(|arg| arg == "--snapshot-window");
    let snapshot_floor = std::env::args().any(|arg| arg == "--snapshot-floor");
    if snapshot_door || snapshot_window || snapshot_floor || std::env::args().any(|arg| arg == "--snapshot") {
        let start = std::time::Instant::now();
        let (target, pitch, distance, filename) = if snapshot_door {
            (Vec3::new(8.55, 1.15, 7.55), 0.04, 3.0, "diorama-wood-preview.png")
        } else if snapshot_window {
            (Vec3::new(5.0, 1.6, 0.12), 0.08, 4.0, "diorama-window-preview.png")
        } else if snapshot_floor {
            (Vec3::new(5.0, 0.2, 4.0), 1.10, 8.0, "diorama-floor-preview.png")
        } else {
            (Vec3::new(5.0, 1.4, 4.0), 0.28, 12.0, "diorama-preview.png")
        };
        let yaw = if snapshot_window || snapshot_door { -1.57 } else { -0.98 };
        render(&mut buffer, &blocks, &materials, target, yaw, pitch, distance);
        buffer.image().export_image(filename);
        println!("Imagen guardada en {filename}; tiempo: {:?}", start.elapsed());
        return;
    }
    let (mut window, thread) = raylib::init().size(WIDTH * 3, HEIGHT * 3).title("NOHC - prueba diorama 3D con ray tracing").build();
    window.set_target_fps(60);
    let mut texture = window.load_texture_from_image(&thread, buffer.image()).unwrap();
    texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR);
    let mut yaw = -0.98_f32;
    let mut pitch = 0.28_f32;
    let mut distance = 12.0_f32;
    while !window.window_should_close() {
        let dt = window.get_frame_time();
        if window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let delta = window.get_mouse_delta();
            yaw -= delta.x * 0.008;
            pitch = (pitch + delta.y * 0.008).clamp(0.12, 1.35);
        }
        if window.is_key_down(KeyboardKey::KEY_LEFT) { yaw -= dt; }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) { yaw += dt; }
        if window.is_key_down(KeyboardKey::KEY_UP) { pitch = (pitch + dt).clamp(0.12, 1.35); }
        if window.is_key_down(KeyboardKey::KEY_DOWN) { pitch = (pitch - dt).clamp(0.12, 1.35); }
        distance = (distance - window.get_mouse_wheel_move() * 1.5).clamp(8.0, 50.0);
        render(&mut buffer, &blocks, &materials, Vec3::new(5.0, 1.4, 4.0), yaw, pitch, distance);
        texture.update_texture(buffer.pixels()).unwrap();
        let mut draw = window.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        draw.draw_texture_ex(&texture, Vector2::new(0.0, 0.0), 0.0, 3.0, Color::WHITE);
        draw.draw_rectangle(0, 0, WIDTH * 3, 27, Color::new(0, 0, 0, 190));
        draw.draw_text("Arrastrar: rotar   Rueda: zoom   Flechas: rotar", 10, 7, 16, Color::WHITE);
        draw.draw_fps(WIDTH * 3 - 90, 7);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rayo_encuentra_bloque_tridimensional() {
        let block = Block { min: Vec3::new(0.0, 0.0, 0.0), max: Vec3::new(1.0, 1.0, 1.0), surface: Surface::Metal };
        let ray = Ray { origin: Vec3::new(-2.0, 0.5, 0.5), dir: Vec3::new(1.0, 0.0, 0.0) };
        let hit = intersect_box(ray, &block, 0).unwrap();
        assert!((hit.t - 2.0).abs() < 0.0001);
        assert_eq!(hit.normal.x, -1.0);
    }

    #[test]
    fn vidrio_desvia_rayo_hacia_normal() {
        let incoming = Vec3::new(0.8, -0.6, 0.0);
        let outgoing = refract(incoming, Vec3::new(0.0, 1.0, 0.0), 1.0, 1.5).unwrap();
        assert!(outgoing.x.abs() < incoming.x.abs());
    }

    #[test]
    fn escena_incluye_vidrio_metal_puerta_y_pared() {
        let blocks = scene();
        for surface in [Surface::Wall, Surface::Glass, Surface::Metal, Surface::Door] {
            assert!(blocks.iter().any(|block| block.surface == surface));
        }
    }

    #[test]
    fn ventana_tiene_vidrio_y_antepecho_solido() {
        let blocks = scene();
        let direction = Vec3::new(0.0, 0.0, 1.0);
        let through_window = closest_hit(Ray { origin: Vec3::new(4.0, 1.6, -2.0), dir: direction }, &blocks).unwrap();
        assert!(through_window.surface == Surface::Glass);
        let through_sill = closest_hit(Ray { origin: Vec3::new(4.0, 0.5, -2.0), dir: direction }, &blocks).unwrap();
        assert!(through_sill.surface == Surface::Wall);
    }

    #[test]
    fn mipmap_promedia_detalle_de_alta_frecuencia() {
        let texture = TextureData {
            width: 2,
            height: 2,
            pixels: vec![Color::BLACK, Color::WHITE, Color::WHITE, Color::BLACK],
        };
        let filtered = FilteredTexture::new(texture);
        let average = filtered.sample(0.3, 0.7, 1.0);
        assert!((average.x - 0.5).abs() < 0.01);
        assert!((average.y - 0.5).abs() < 0.01);
    }
}
