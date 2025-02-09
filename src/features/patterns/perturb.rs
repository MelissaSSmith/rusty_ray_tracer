use noise::{Perlin, NoiseFn};
use canvas_draw::color::Color;
use crate::features::patterns::{OnePatternWithScale, Pattern, PatternAt, Patterns};
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone, Debug)]
pub struct PerturbedPattern {
    pattern: Box<Pattern>,
    scale: f64
}

impl PerturbedPattern {
    fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        let noise = Perlin::new(0);

        noise.get([x, y, z])
    }
}

impl OnePatternWithScale for PerturbedPattern {
    fn create(pattern: Pattern, scale: Option<f64>) -> Pattern {
        let s = match scale {
            None => { 1.0 }
            Some(s) => { s }
        };
        let pattern = PerturbedPattern {
            pattern: Box::new(pattern),
            scale: s
        };
        Pattern::create(Box::new(Patterns::Perturb(pattern)))
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