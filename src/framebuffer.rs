use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn from_float(r: f32, g: f32, b: f32) -> Self {
        Color {
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color({}, {}, {})", self.r, self.g, self.b)
    }
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            background_color: Color::new(0, 0, 0),
            current_color: Color::new(255, 255, 255),
        }
    }

    pub fn clear(&mut self) {
        let color = self.background_color.to_u32();
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            
            if depth < self.zbuffer[index] {
                self.buffer[index] = self.current_color.to_u32();
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn point_with_color(&mut self, x: usize, y: usize, depth: f32, color: Color) {
        // Validar que las coordenadas estén en rango
        if x >= self.width || y >= self.height {
            return; // Píxel fuera de rango
        }
        
        // Validar que depth sea un número válido
        if !depth.is_finite() {
            return; // Depth inválido
        }
        
        let index = y * self.width + x;
        
        // Verificar que el índice no exceda el buffer
        if index >= self.buffer.len() {
            return; // Índice fuera de buffer
        }
        
        if depth < self.zbuffer[index] {
            self.buffer[index] = color.to_u32();
            self.zbuffer[index] = depth;
        }
    }
}
