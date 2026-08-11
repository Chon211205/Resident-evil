use raylib::ffi;
use raylib::prelude::*;

pub struct Framebuffer {
    image: Image,
    width: i32,
    height: i32,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::BLACK;

        let image = unsafe {
            Image::from_raw(ffi::GenImageColor(
                width,
                height,
                background_color.into(),
            ))
        };

        Self {
            image,
            width,
            height,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn pixels(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.image.data as *const u8,
                (self.width * self.height * 4) as usize,
            )
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        self.image
            .clear_background(
                self.background_color,
            );
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if !self.is_inside(x, y) {
            return;
        }

        self.image.draw_pixel(
            x,
            y,
            self.current_color,
        );
    }

    pub fn point_with_size(
        &mut self,
        centro_x: i32,
        centro_y: i32,
        radio: i32,
    ) {
        for y in -radio..=radio {
            for x in -radio..=radio {
                if x * x + y * y <= radio * radio {
                    self.point(
                        centro_x + x,
                        centro_y + y,
                    );
                }
            }
        }
    }

    pub fn dotted_line(
        &mut self,
        inicio_x: i32,
        inicio_y: i32,
        final_x: i32,
        final_y: i32,
        separacion: f32,
    ) {
        let diferencia_x = final_x - inicio_x;
        let diferencia_y = final_y - inicio_y;

        let distancia = (
            (
                diferencia_x * diferencia_x
                    + diferencia_y * diferencia_y
            ) as f32
        )
            .sqrt();

        if distancia == 0.0 {
            self.point_with_size(
                inicio_x,
                inicio_y,
                1,
            );

            return;
        }

        let direccion_x =
            diferencia_x as f32 / distancia;

        let direccion_y =
            diferencia_y as f32 / distancia;

        let mut recorrido = 0.0;

        while recorrido <= distancia {
            let x = inicio_x as f32
                + direccion_x * recorrido;

            let y = inicio_y as f32
                + direccion_y * recorrido;

            self.point_with_size(
                x.round() as i32,
                y.round() as i32,
                1,
            );

            recorrido += separacion;
        }
    }

    pub fn render_to_file(&self, file_name: &str) {
        self.image.export_image(file_name);

        println!(
            "Imagen guardada correctamente como '{}'.",
            file_name
        );
    }

    fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0
            && x < self.width
            && y >= 0
            && y < self.height
    }
}