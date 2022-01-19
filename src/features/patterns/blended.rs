use std::any::Any;
use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::primitives::point::Point;

#[derive(Clone)]
pub struct BlendedPattern {
    pattern_a: Box<dyn Pattern>,
    pattern_b: Box<dyn Pattern>,
    transformation: Matrix
}

impl BlendedPattern {
    pub fn create(pattern_a: Box<dyn Pattern>, pattern_b: Box<dyn Pattern>) -> BlendedPattern {
        BlendedPattern { pattern_a, pattern_b, transformation: Matrix::identity() }
    }
}

impl Pattern for BlendedPattern {
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
        self.transformation = self.transformation.clone() * transform;
    }

    fn pattern_at(&self, point: Point) -> Color {
        let tp = self.transformation.inverse() * point;
        let color_a = self.pattern_a.pattern_at(tp) * 0.5;
        let color_b = self.pattern_b.pattern_at(tp) * 0.5;

        color_a + color_b
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::blended::BlendedPattern;
    use crate::features::patterns::Pattern;
    use crate::features::patterns::stripe::StripePattern;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;

    #[test]
    fn test_blended_pattern() {
        let pattern_a = StripePattern::create(BLACK, WHITE);
        let pattern_b = StripePattern::create(WHITE, BLACK);

        let pattern = BlendedPattern::create(Box::new(pattern_a), Box::new(pattern_b));

        let color = Color::create(0.5, 0.5, 0.5);
        assert!(pattern.pattern_at(Point::zero()).equals(color));
        assert!(pattern.pattern_at(Point::create(0.9, 0.0, 0.0)).equals(color));
        assert!(pattern.pattern_at(Point::create(1.0, 0.0, 0.0)).equals(color));
        assert!(pattern.pattern_at(Point::create(-0.1, 0.0, 0.0)).equals(color));
        assert!(pattern.pattern_at(Point::create(-1.0, 0.0, 0.0)).equals(color));
        assert!(pattern.pattern_at(Point::create(-1.1, 0.0, 0.0)).equals(color));
    }
}