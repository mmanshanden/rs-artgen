use rand::prelude::Rng;

use crate::color::Color;
use crate::canvas::Canvas;
use crate::anneal::Config;

pub trait Primitive: Copy {
    fn generate<R: Rng>(config: &Config, rand: &mut R) -> Self;

    fn mutate<R: Rng>(self, config: &Config, rand: &mut R) -> Self;
    
    fn draw<T: Color>(&self, color: T, canvas: &mut Canvas<T>);
}

#[derive(Clone, Copy)]
pub struct Point {
    x: i16,
    y: i16,
}

impl Primitive for Point {
    fn generate<R: Rng>(config: &Config, rand: &mut R) -> Self {
        Self { 
            x: rand.gen_range(config.min_y, config.max_x),
            y: rand.gen_range(config.min_y, config.max_y),
        }
    }

    fn mutate<R: Rng>(self, config: &Config, rand: &mut R) -> Self {
        Self {
            x: rand.gen_range(self.x - 5, self.x + 5).min(config.max_x).max(config.min_x),
            y: rand.gen_range(self.y - 5, self.y + 5).min(config.max_x).max(config.min_y),
        }
    }

    fn draw<T: Color>(&self, color: T, canvas: &mut Canvas<T>) {
        canvas.plot(self.x, self.y, color);
    }
}

#[derive(Clone, Copy)]
pub struct Splat {
    x: i16,
    y: i16,
}

impl Primitive for Splat {
    fn generate<R: Rng>(config: &Config, rand: &mut R) -> Self {
        Self { 
            x: rand.gen_range(config.min_y + 2, config.max_x - 2),
            y: rand.gen_range(config.min_y + 2, config.max_y - 2),
        }
    }

    fn mutate<R: Rng>(self, config: &Config, rand: &mut R) -> Self {
        Self {
            x: rand.gen_range(self.x - 5, self.x + 5).min(config.max_x - 2).max(config.min_x + 2),
            y: rand.gen_range(self.y - 5, self.y + 5).min(config.max_x - 2).max(config.min_y + 2),
        }
    }

    fn draw<T: Color>(&self, color: T, canvas: &mut Canvas<T>) {
        canvas.plot(self.x, self.y, color);

        for x in (self.x - 2)..(self.x + 2) {
            for y in (self.y - 2)..(self.y + 2) {
                canvas.plot(x, y, color)
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Line {
    a: Point,
    b: Point,
}

impl Primitive for Line {
    fn generate<R: Rng>(config: &Config, rand: &mut R) -> Self {
        Self {
            a: Point::generate(config, rand),
            b: Point::generate(config, rand),
        }
    }

    fn mutate<R: Rng>(self, config: &Config, rand: &mut R) -> Self {
        Self {
            a: self.a.mutate(config, rand),
            b: self.b.mutate(config, rand),
        }
    }

    fn draw<T: Color>(&self, color: T, canvas: &mut Canvas<T>) {
        let dx = i16::abs(self.b.x - self.a.x);
        let dy = -i16::abs(self.b.y - self.a.y);

        let sx = if self.a.x < self.b.x { 1 } else { -1 };
        let sy = if self.a.y < self.b.y { 1 } else { -1 };

        let mut err = dx + dy;

        let mut x = self.a.x;
        let mut y = self.a.y;

        while x != self.b.x && y != self.b.y {
            canvas.plot(x, y, color);
            let e2 = err * 2;

            if e2 >= dy {
                err += dy;
                x += sx;
            } 

            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}





