#[derive(Debug, Clone, Copy)]
pub struct PerlinNoise2d {
    seed: u32,
}

impl PerlinNoise2d {
    pub const fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn sample(self, x: f32, y: f32) -> f32 {
        let x0 = x.floor() as i32;
        let y0 = y.floor() as i32;
        let tx = x - x0 as f32;
        let ty = y - y0 as f32;
        let u = fade(tx);
        let v = fade(ty);

        let n00 = gradient(self.hash(x0, y0), tx, ty);
        let n10 = gradient(self.hash(x0 + 1, y0), tx - 1.0, ty);
        let n01 = gradient(self.hash(x0, y0 + 1), tx, ty - 1.0);
        let n11 = gradient(self.hash(x0 + 1, y0 + 1), tx - 1.0, ty - 1.0);
        lerp(lerp(n00, n10, u), lerp(n01, n11, u), v).clamp(-1.0, 1.0)
    }

    fn hash(self, x: i32, y: i32) -> u32 {
        let mut value =
            self.seed ^ (x as u32).wrapping_mul(0x9e37_79b9) ^ (y as u32).wrapping_mul(0x85eb_ca6b);
        value ^= value >> 16;
        value = value.wrapping_mul(0x7feb_352d);
        value ^= value >> 15;
        value.wrapping_mul(0x846c_a68b) ^ (value >> 16)
    }
}

fn fade(value: f32) -> f32 {
    value * value * value * (value * (value * 6.0 - 15.0) + 10.0)
}

fn lerp(a: f32, b: f32, amount: f32) -> f32 {
    a + (b - a) * amount
}

fn gradient(hash: u32, x: f32, y: f32) -> f32 {
    const INV_SQRT_2: f32 = std::f32::consts::FRAC_1_SQRT_2;
    let (gx, gy) = match hash & 7 {
        0 => (1.0, 0.0),
        1 => (-1.0, 0.0),
        2 => (0.0, 1.0),
        3 => (0.0, -1.0),
        4 => (INV_SQRT_2, INV_SQRT_2),
        5 => (-INV_SQRT_2, INV_SQRT_2),
        6 => (INV_SQRT_2, -INV_SQRT_2),
        _ => (-INV_SQRT_2, -INV_SQRT_2),
    };
    gx * x + gy * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_is_deterministic_and_continuous_nearby() {
        let noise = PerlinNoise2d::new(42);
        let sample = noise.sample(2.25, -7.75);
        assert_eq!(sample, noise.sample(2.25, -7.75));
        assert!((sample - noise.sample(2.251, -7.75)).abs() < 0.01);
    }
}
