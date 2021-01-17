use crate::color::Color;

pub struct Canvas<T: Color> {
    pixels: Vec<T>,
    width: usize,
    height: usize
}

impl<T: Color> Canvas<T> {
    pub fn empty(width: usize, height: usize) -> Canvas<T> {
        let len = width * height;
        let mut pixels = Vec::with_capacity(width * height);

        for _ in 0..len {
            let color = T::white();
            pixels.push(color);
        }

        Self {
            pixels,
            width,
            height,
        }
    }

    pub fn from_file(path: &str, width: usize, height: usize) -> Canvas<T> {
        let filter_type = image::imageops::FilterType::Nearest;

        let img = image::open(path)
            .unwrap()
            .resize(width as u32, height as u32, filter_type)
            .to_bgr8();
           
        let img = img.as_raw();

        let mut pixels = Vec::new();
        let mut writes = Vec::new();
        let mut i = 0;

        while i < img.len() {

            let b: u8 = img[i + 0];
            let g: u8 = img[i + 1];
            let r: u8 = img[i + 2];

            let value = T::from_rgb(r, g, b);
            pixels.push(value);
            writes.push(0);

            i += 3;
        }

        Self {
            pixels,
            width,
            height
        }
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        let len = self.width * self.height;
        let mut pixels = vec![0xFFFFFF; len];

        for i in 0..len {
            let (r, g, b) = self.pixels[i].to_rgb();

            let r = r as u32;
            let g = g as u32;
            let b = b as u32;

            pixels[i] = (r << 16) | (g << 8) | b;
        }

        pixels
    }

    pub fn likeliness(&self, other: &Self) -> f32 {
        let len = self.pixels.len();
        let mut diff = 0.0;

        for i in 0..len {
            diff += self.pixels[i].compare(other.pixels[i]);
        }

        diff
    }

    pub fn clear(&mut self) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = T::white();
        }
    }

    pub fn plot(&mut self, x: i16, y: i16, color: T) {
        let x = x as usize;
        let y = y as usize;
        let i = x * (self.width as usize) + y;

        self.pixels[i] = color;
    }
}
