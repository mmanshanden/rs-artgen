use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use crate::{color::Color};
use crate::primitive::Primitive;
use crate::canvas::Canvas;

pub struct Config {
    pub min_x: i16,
    pub max_x: i16,
    pub min_y: i16,
    pub max_y: i16,
}

pub struct Anneal<T: Primitive, U: Color> {
    pub canvas: Canvas<U>,
    primitives: Vec<T>,
    colors: Vec<U>,
    fitness: f32,
    rng: SmallRng,
}

impl<T: Primitive, U: Color> Anneal<T, U> {
    pub fn new(count: usize, config: &Config, target_width: usize, target_height: usize) -> Self {
        let mut rng = SmallRng::from_entropy();

        let mut primis = Vec::with_capacity(count);
        let mut colors = Vec::with_capacity(count);

        for _ in 0..count {
            let p = T::generate(&config, &mut rng);
            let c = U::generate(&mut rng);

            primis.push(p);
            colors.push(c);
        }

        Self {
            primitives: primis,
            colors,
            canvas: Canvas::empty(target_width, target_height),
            fitness: f32::NEG_INFINITY,
            rng,
        }
    }

    pub fn iterate(&mut self, config: &Config, target: &Canvas<U>) {
        let len = self.primitives.len();
        let i = self.rng.gen_range(0, len);

        let primitive = self.primitives[i];
        let color = self.colors[i];

        if self.rng.gen_bool(0.5) {
            self.primitives[i].mutate(&config, &mut self.rng);
            self.colors[i].mutate(&mut self.rng);
        } else {
            self.primitives[i] = T::generate(&config, &mut self.rng);
            self.colors[i] = U::generate(&mut self.rng);
        }

        self.canvas.clear();

        for i in 0..self.primitives.len() {
            self.primitives[i].draw(self.colors[i], &mut self.canvas);
        }

        let fitness = target.likeliness(&self.canvas);

        if fitness > self.fitness  {
            self.fitness = fitness;
        } else {
            self.primitives[i] = primitive;
            self.colors[i] = color;
        }
    }
}
