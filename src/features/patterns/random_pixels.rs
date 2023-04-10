use rand::Rng;
use crate::features::color::Color;
use crate::features::patterns::{EmptyCreate, Pattern, PatternAt, Patterns};
use crate::features::primitives::point::Point;

#[derive(Clone, Debug)]
pub struct RandomPixelsPattern {}

impl RandomPixelsPattern {
    fn new() -> RandomPixelsPattern {
        Self {}
    }
}

impl EmptyCreate for RandomPixelsPattern {
    fn create() -> Pattern {
        let pattern = RandomPixelsPattern::new();
        Pattern::create(Box::new(Patterns::RandomPixels(pattern)))
    }
}

impl PatternAt for RandomPixelsPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let mut rng = rand::thread_rng();
        let r = (rng.gen::<f64>() * 256.0).floor();
        let g = (rng.gen::<f64>() * 256.0).floor();
        let b = (rng.gen::<f64>() * 256.0).floor();
        Color::create(r, g, b)
    }
}