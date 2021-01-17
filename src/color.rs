use rand::Rng;

pub trait Color: Copy {
    fn black() -> Self;

    fn white() -> Self;

    fn from_rgb(r: u8, g: u8, b: u8) -> Self;

    fn to_rgb(self) -> (u8, u8, u8);

    fn compare(self, other: Self) -> f32;

    fn generate<R: Rng>(rng: &mut R) -> Self;

    fn mutate<R: Rng>(self, rng: &mut R) -> Self;
}

#[derive(Clone, Copy)]
pub struct Black {
    pub value: u8,
}

impl Color for Black {
    fn black() -> Self {
        return Self { value: 0 };
    }

    fn white() -> Self {
        return Self { value: 255 };
    }

    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let r = r as f32;
        let g = g as f32;
        let b = b as f32;

        let value = (r * 0.2126 + 0.7152 * g + 0.0722 * b) as u8;

        if value > 110 {
            Self { value: 255 }
        } else {
            Self { value: 0 }
        }
    }

    fn to_rgb(self) -> (u8, u8, u8) {
        (self.value, self.value, self.value)
    }

    fn compare(self, other: Self) -> f32 {
        if self.value == other.value {
            return 1.0;
        } else {
            return 0.0;
        }
    }

    fn generate<R: Rng>(_rng: &mut R) -> Self {
        Self::black()
    }

    fn mutate<R: Rng>(self, _rng: &mut R) -> Self {
        self
    }
}

#[derive(Clone, Copy)]
pub struct Grey {
    pub value: u8,
}

impl Color for Grey {
    fn black() -> Self {
        return Self { value: 0 };
    }

    fn white() -> Self {
        return Self { value: 255 };
    }

    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let r = r as f32;
        let g = g as f32;
        let b = b as f32;

        let value = (r * 0.2126 + 0.7152 * g + 0.0722 * b) as u8;

        Self { value }
    }

    fn to_rgb(self) -> (u8, u8, u8) {
        (self.value, self.value, self.value)
    }

    fn compare(self, other: Self) -> f32 {
        let a = self.value as f32;
        let b = other.value as f32;
        let d = (a - b).abs();

        1.0 - d / 255.0
    }

    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            value: rng.gen_range(0, 190),
        }
    }

    fn mutate<R: Rng>(self, rng: &mut R) -> Self {
        let value = self.value as i16;

        Self {
            value: rng.gen_range(value - 5, value + 5).max(0).min(255) as u8,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Hsv {
    h: f32,
    s: f32,
    v: f32,
}

impl Color for Hsv {
    fn black() -> Self {
        Self {
            h: 0.0,
            s: 0.0,
            v: 0.0,
        }
    }

    fn white() -> Self {
        Self {
            h: 0.0,
            s: 0.0,
            v: 1.0,
        }
    }
    
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let r = r as f32;
        let g = g as f32;
        let b = b as f32;

        let mut h;
        let s;

        let min = r.min(g).min(b) as f32;
        let v = r.max(g).max(b) as f32;
        let delta = v - min;

        if v == 0.0 {
            s = 0.0;
        } else {
            s = delta / v;
        }

        if s == 0.0 {
            h = 0.0;
        } else {
            if r == v {
                h = (g - b) / delta;
            } else if g == v {
                h = 2.0 + (b - r) / delta;
            } else {
                h = 4.0 + (r - g) / delta;
            }

            h *= 60.0;

            if h < 0.0 {
                h += 360.0;
            }
        }

        Self {
            h, s, v: v / 255.0
        }
    }

    fn to_rgb(self) -> (u8, u8, u8) {
        if self.s == 0.0 {
            let v = self.v * 255.0;
            return (v as u8, v as u8, v as u8);
        }

        let h = if self.h == 360.0 { 0.0 } else { self.h / 60.0 };
        let i = h.trunc();
        let f = h - i;

        let p = self.v * (1.0 - self.s);
        let q = self.v * (1.0 - self.s * f);
        let t = self.v * (1.0 - self.s * (1.0 - f));

        let v = (self.v * 255.0) as u8;
        let p = (p * 255.0) as u8;
        let q = (q * 255.0) as u8;
        let t = (t * 255.0) as u8;

        match i as i32 {
            0 => return (v, t, p),
            1 => return (q, v, p),
            2 => return (p, v, t),
            3 => return (p, q, v),
            4 => return (t, p, v),
            _ => return (v, p, q),
        }
    }

    fn compare(self, other: Self) -> f32 {
        let dh = f32::abs(self.h - other.h);
        let dh = f32::min(dh, 360.0 - dh) / 180.0;
        let ds = f32::abs(self.s - other.s);
        let dv = f32::abs(self.v - other.v);

        return 1.0 - (dh * dh + ds * ds + dv * dv).sqrt();
    }

    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            h: rng.gen_range(0.0, 360.0),
            s: rng.gen_range(0.0, 1.0),
            v: rng.gen_range(0.0, 1.0),
        }
    }

    fn mutate<R: Rng>(self, rng: &mut R) -> Self {
        let h = rng.gen_range(self.h - 14.0, self.h + 14.0);

        let h = {
            if h > 360.0 { 
                h - 360.0 
            } else if h < 0.0 { 
                h + 360.0 
            } else { 
                h 
            }
        };
        
        Self {
            h,
            s: rng.gen_range(self.s - 0.08, self.s + 0.08).min(1.0).max(0.0),
            v: rng.gen_range(self.v - 0.08, self.v + 0.08).min(1.0).max(0.0),
        }
    }
}