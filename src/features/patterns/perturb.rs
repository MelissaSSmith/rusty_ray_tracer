use noise::{Perlin, NoiseFn};
use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::{OnePatternWithScale, Pattern, PatternAt, Patterns};
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone)]
pub struct PerturbedPattern {
    pattern: Box<Pattern>,
    scale: f64
}

impl PerturbedPattern {
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

impl OnePatternWithScale for PerturbedPattern {
    fn create(pattern: Pattern, scale: Option<f64>) -> Pattern {
        let transform = Matrix::identity();
        let s = match scale {
            None => { 1.0 }
            Some(s) => { s }
        };
        let pattern = PerturbedPattern {
            pattern: Box::new(pattern),
            scale: s
        };
        Pattern {
            pattern: Box::new(Patterns::Perturb(pattern)),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl PatternAt for PerturbedPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let new_x = point.x() + (self.noise(point.x() , point.y() + 0.1, point.z()) * self.scale);
        let new_y = point.y() + (self.noise(point.x() , point.y() + 0.2, point.z() + 1.0) * self.scale);
        let new_z = point.z() + (self.noise(point.x() , point.y() + 0.3, point.z() + 2.0) * self.scale);
        self.pattern.pattern_at(&Point::create(new_x, new_y, new_z))
    }
}