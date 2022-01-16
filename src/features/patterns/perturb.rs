use std::any::Any;
use noise::{Perlin, NoiseFn};
use crate::features::color::Color;
use crate::features::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::point::Point;

#[derive(Clone)]
pub struct PerturbedPattern {
    pattern: Box<dyn Pattern>,
    transformation: Matrix
}

impl PerturbedPattern {
    pub fn create(pattern: Box<dyn Pattern>) -> PerturbedPattern {
        PerturbedPattern { pattern, transformation: Matrix::create_identity() }
    }

    fn fade(&self, t: f64) -> f64 {
        ((6.0 * t - 15.0) * t + 10.0) * t * t * t
    }

    fn lerp(&self, t:f64, a1: f64, a2: f64) -> f64 {
        a1 + t * (a2 - a1)
    }

    fn grad(&self, hash: i32, x: f64, y: f64, z:f64) -> f64 {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 { y } else { if h == 12 || h == 14 { x } else { z }};

        self.determine_value(h, 1, u) + self.determine_value(h, 2, v)
    }

    fn determine_value(&self, h: i32, mod_i: i32, return_value: f64) -> f64 {
        return if h & mod_i == 0 {
            return_value
        } else {
            -return_value
        }
    }

    fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        let noise = Perlin::new();

        noise.get([x, y, z])
    }
}

impl Pattern for PerturbedPattern {
    fn box_clone(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    fn transform(&mut self, transform: Matrix) {
        let new_transform = self.transformation.multiply(&transform);
        self.transformation = new_transform;
    }

    fn pattern_at(&self, point: Point) -> Color {
        let scale = 0.1;

        let new_x = point.value().x + self.noise(point.value().x , point.value().y, point.value().z) * scale;
        let new_y = point.value().y + self.noise(point.value().x , point.value().y, point.value().z + 1.0) * scale;
        let new_z = point.value().z + self.noise(point.value().x , point.value().y, point.value().z + 2.0) * scale;
        self.pattern.pattern_at(Point::create(new_x, new_y, new_z))
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::consts::WHITE;
    use crate::features::patterns::perturb::PerturbedPattern;
    use crate::features::patterns::solid::SolidPattern;

    #[test]
    fn test_fade() {
        let pattern = PerturbedPattern::create(Box::new(SolidPattern::create(WHITE)));

        let fade = pattern.fade(1.0);

        assert_eq!(fade, 1.0);
    }

    #[test]
    fn test_lerp() {
        let pattern = PerturbedPattern::create(Box::new(SolidPattern::create(WHITE)));

        let fade = pattern.lerp(1.0, 2.0, 3.0);

        assert_eq!(fade, 3.0);
    }

    #[test]
    fn test_grad() {
        let pattern = PerturbedPattern::create(Box::new(SolidPattern::create(WHITE)));

        let grad = pattern.grad(0, 1.0, 1.0, 1.0);

        assert_eq!(grad, 2.0);
    }
}