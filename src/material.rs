use crate::texture_data::TextureData;
use raylib::color::Color;

/// Parámetros usados por el trazador de rayos del escenario.
pub struct Material<'a> {
    pub texture: &'a TextureData,
    pub albedo: Color,
    pub specular: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub refractive_index: f32,
}

impl<'a> Material<'a> {
    pub fn new(
        texture: &'a TextureData,
        albedo: Color,
        specular: f32,
        transparency: f32,
        reflectivity: f32,
        refractive_index: f32,
    ) -> Self {
        Self {
            texture,
            albedo,
            specular,
            transparency,
            reflectivity,
            refractive_index,
        }
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let x = (u * self.texture.width as f32) as i32;
        let y = (v * self.texture.height as f32) as i32;
        let texel = self.texture.get_pixel(x, y);
        Color::new(
            ((texel.r as u16 * self.albedo.r as u16) / 255) as u8,
            ((texel.g as u16 * self.albedo.g as u16) / 255) as u8,
            ((texel.b as u16 * self.albedo.b as u16) / 255) as u8,
            255,
        )
    }
}
