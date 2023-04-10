use rand::Rng;
use crate::features::color::Color;
use crate::features::patterns::{EmptyCreate, Pattern, PatternAt, Patterns, WidthHeightCreate};
use crate::features::primitives::point::Point;

#[derive(Clone, Debug)]
pub struct RandomRectanglesPattern {
    width: f64,
    height: f64,
    rectangles: Vec<Box<Pattern>>
}

impl RandomRectanglesPattern {
    const MAX_RECTANGLES: i32 = 32;
    fn new(width: f64, height: f64) -> RandomRectanglesPattern {
        Self {
            width,
            height,
            rectangles: vec![]
        }
    }

    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
        self.height
    }

    fn random_color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let r = (rng.gen::<f64>() * 256.0).floor();
        let g = (rng.gen::<f64>() * 256.0).floor();
        let b = (rng.gen::<f64>() * 256.0).floor();
        Color::create(r, g, b)
    }
}

impl EmptyCreate for RandomRectanglesPattern {
    fn create() -> Pattern {
        let pattern = RandomRectanglesPattern::new(1040.0, 1040.0);
        Pattern::create(Box::new(Patterns::RandomRectangles(pattern)))
    }
}

impl WidthHeightCreate for RandomRectanglesPattern {
    fn create(width: f64, height: f64) -> Pattern {
        let pattern = RandomRectanglesPattern::new(width, height);
        Pattern::create(Box::new(Patterns::RandomRectangles(pattern)))
    }
}

impl PatternAt for RandomRectanglesPattern {
    fn pattern_at(&self, _: &Point) -> Color {
        Color::random_color()
    }
}