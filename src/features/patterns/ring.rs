use std::any::Any;
use crate::features::color::Color;
use crate::features::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::patterns::solid::SolidPattern;
use crate::features::point::Point;

#[derive(Clone)]
pub struct RingPattern {
    pattern_a: Box<dyn Pattern>,
    pattern_b: Box<dyn Pattern>,
    transformation: Matrix
}

impl RingPattern {
    pub fn create(color_a: Color, color_b: Color) -> RingPattern {
        RingPattern {
            pattern_a: Box::new(SolidPattern::create(color_a)),
            pattern_b: Box::new(SolidPattern::create(color_b)),
            transformation: Matrix::create_identity()
        }
    }

    pub fn create_with_patterns(pattern_a: Box<dyn Pattern>, pattern_b: Box<dyn Pattern>) -> RingPattern {
        RingPattern{
            pattern_a,
            pattern_b,
            transformation: Matrix::create_identity() }
    }
}

impl Pattern for RingPattern {
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
        let tp = self.transformation.inverse().multiply_point(point);
        let value = (tp.value().x.powi(2) + tp.value().z.powi(2)).sqrt();
        if value.floor() % 2.0 == 0.0 {
            return self.pattern_a.pattern_at(tp);
        }

        self.pattern_b.pattern_at(tp)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::Pattern;
    use crate::features::patterns::ring::RingPattern;
    use crate::features::point::Point;

    #[test]
    fn test_ring_should_extend_both_x_and_z() {
        let pattern = RingPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(1.0, 0.0, 0.0)).equals(BLACK));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 1.0)).equals(BLACK));
        assert!(pattern.pattern_at(Point::create(0.708, 0.0, 0.708)).equals(BLACK));
    }
}