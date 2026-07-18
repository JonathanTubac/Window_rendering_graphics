use raylib::prelude::*;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            pixels: vec![Color::BLACK; width * height],
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        self.pixels.fill(self.background_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.pixels[y as usize * self.width + x as usize] = self.current_color;
        }
    }

    pub fn get_color(&self, x: i32, y: i32) -> Color {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.pixels[y as usize * self.width + x as usize]
        } else {
            self.background_color
        }
    }

    pub fn render_to_file(&self, path: &str) {
        let w = self.width as u32;
        let h = self.height as u32;
        // Cada fila se rellena a múltiplo de 4 bytes (formato BMP)
        let row_size = ((24 * w + 31) / 32) * 4;
        let pixel_data_size = row_size * h;
        let file_size = 54 + pixel_data_size;

        let mut buf: Vec<u8> = Vec::with_capacity(file_size as usize);

        // File header (14 bytes)
        buf.extend_from_slice(b"BM");
        buf.extend_from_slice(&file_size.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&54u32.to_le_bytes());

        // DIB header / BITMAPINFOHEADER (40 bytes)
        buf.extend_from_slice(&40u32.to_le_bytes());
        buf.extend_from_slice(&(w as i32).to_le_bytes());
        buf.extend_from_slice(&(h as i32).to_le_bytes()); // positivo = bottom-up
        buf.extend_from_slice(&1u16.to_le_bytes());        // planos de color
        buf.extend_from_slice(&24u16.to_le_bytes());       // bits por pixel
        buf.extend_from_slice(&0u32.to_le_bytes());        // sin compresión
        buf.extend_from_slice(&pixel_data_size.to_le_bytes());
        buf.extend_from_slice(&2835i32.to_le_bytes());
        buf.extend_from_slice(&2835i32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());

        // Pixel data (de abajo hacia arriba, orden BGR)
        for row in (0..h).rev() {
            let mut written = 0u32;
            for col in 0..w {
                let c = self.pixels[row as usize * self.width + col as usize];
                buf.push(c.b);
                buf.push(c.g);
                buf.push(c.r);
                written += 3;
            }
            while written % 4 != 0 {
                buf.push(0);
                written += 1;
            }
        }

        std::fs::write(path, buf).expect("No se pudo escribir el archivo BMP");
    }

    pub fn render_to_png(&self, path: &str) {
        let h = self.height as u32;
        let img = image::ImageBuffer::from_fn(
            self.width as u32,
            h,
            |x, y| {
                // Invertir Y para que y=0 quede abajo (coordenadas matemáticas)
                let y_flipped = h - 1 - y;
                let c = self.pixels[y_flipped as usize * self.width + x as usize];
                image::Rgb([c.r, c.g, c.b])
            },
        );
        img.save(path).expect("No se pudo guardar el PNG");
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) {
        let mut image = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
        for y in 0..self.height {
            for x in 0..self.width {
                image.draw_pixel(x as i32, y as i32, self.pixels[y * self.width + x]);
            }
        }

        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &image) {
            let screen_width = window.get_screen_width() as f32;
            let screen_height = window.get_screen_height() as f32;

            let source = Rectangle::new(0.0, 0.0, self.width as f32, self.height as f32);
            let dest = Rectangle::new(0.0, 0.0, screen_width, screen_height);

            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture_pro(
                &texture,
                source,
                dest,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
    }
}
