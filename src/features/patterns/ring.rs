use std::any::Any;
use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone)]
pub struct RingPattern {
    pattern_a: Box<dyn Pattern>,
    pattern_b: Box<dyn Pattern>,
    transformation: Matrix,
    inverse_transformation: Matrix
}

impl RingPattern {
    pub fn create(color_a: Color, color_b: Color) -> RingPattern {
        let transform = Matrix::identity();
        RingPattern {
            pattern_a: Box::new(SolidPattern::create(color_a)),
            pattern_b: Box::new(SolidPattern::create(color_b)),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }

    pub fn create_with_patterns(pattern_a: Box<dyn Pattern>, pattern_b: Box<dyn Pattern>) -> RingPattern {
        let transform = Matrix::identity();
        RingPattern{
            pattern_a,
            pattern_b,
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
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

    fn inverse_transformation(&self) -> Matrix {
        self.inverse_transformation.clone()
    }

    fn transform(&mut self, transform: Matrix) {
        self.transformation = self.transformation.clone() * transform;
        self.inverse_transformation = self.transformation.inverse();
    }

    fn pattern_at(&self, point: &Point) -> Color {
        let tp = self.inverse_transformation() * *point;
        let value = (tp.x().powi(2) + tp.z().powi(2)).sqrt();
        if value.floor() % 2.0 == 0.0 {
            return self.pattern_a.pattern_at(&tp);
        }

        self.pattern_b.pattern_at(&tp)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::Pattern;
    use crate::features::patterns::ring::RingPattern;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;

    #[test]
    fn test_ring_should_extend_both_x_and_z() {
        let pattern = RingPattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(&Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(&Point::create(1.0, 0.0, 0.0)).equals(BLACK));
        assert!(pattern.pattern_at(&Point::create(0.0, 0.0, 1.0)).equals(BLACK));
        assert!(pattern.pattern_at(&Point::create(0.708, 0.0, 0.708)).equals(BLACK));
    }
}