use std::any::Any;
use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone)]
pub struct StripePattern {
    pattern_a: Box<dyn Pattern>,
    pattern_b: Box<dyn Pattern>,
    transformation: Matrix
}

impl StripePattern {
    pub fn create(color_a: Color, color_b: Color) -> StripePattern {
        StripePattern {
            pattern_a: Box::new(SolidPattern::create(color_a)),
            pattern_b: Box::new(SolidPattern::create(color_b)),
            transformation: Matrix::identity()
        }
    }

    pub fn create_with_patterns(pattern_a: Box<dyn Pattern>, pattern_b: Box<dyn Pattern>) -> StripePattern {
        StripePattern{
            pattern_a,
            pattern_b,
            transformation: Matrix::identity() }
    }
}

impl Pattern for StripePattern {
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

        if tp.x().floor() % 2.0 == 0.0 {
            return self.pattern_a.pattern_at(tp);
        }

        self.pattern_b.pattern_at(tp)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::primitives::matrix::Matrix;
    use crate::features::patterns::Pattern;
    use crate::features::patterns::stripe::StripePattern;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::shapes::Shape;
    use crate::features::shapes::sphere::Sphere;

    #[test]
    fn test_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 1.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 2.0, 0.0)).equals(WHITE));
    }

    #[test]
    fn test_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 1.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 2.0)).equals(WHITE));
    }

    #[test]
    fn test_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::zero()).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.9, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(1.0, 0.0, 0.0)).equals(BLACK));
        assert!(pattern.pattern_at(Point::create(-0.1, 0.0, 0.0)).equals(BLACK));
        assert!(pattern.pattern_at(Point::create(-1.0, 0.0, 0.0)).equals(BLACK));
        assert!(pattern.pattern_at(Point::create(-1.1, 0.0, 0.0)).equals(WHITE));
    }

    #[test]
    fn test_stripes_with_an_object_transformation() {
        let mut object = Sphere::create();
        object.set_transform(Matrix::scale(2.0, 2.0, 2.0 ));
        let pattern = StripePattern::create(WHITE, BLACK);

        let c = pattern.pattern_at_object(Box::new(object), Point::create(1.5, 0.0, 0.0));

        assert!(c.equals(WHITE));
    }

    #[test]
    fn test_stripes_with_a_pattern_transformation() {
        let object = Sphere::create();
        let mut pattern = StripePattern::create(WHITE, BLACK);
        pattern.transform(Matrix::scale(2.0, 2.0, 2.0));

        let c = pattern.pattern_at_object(Box::new(object), Point::create(1.5, 0.0, 0.0));

        assert!(c.equals(WHITE));
    }

    #[test]
    fn test_stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = Sphere::create();
        object.set_transform(Matrix::scale(2.0, 2.0, 2.0 ));
        let mut pattern = StripePattern::create(WHITE, BLACK);
        pattern.transform(Matrix::translate(0.5, 0.0, 0.0));

        let c = pattern.pattern_at_object(Box::new(object), Point::create(2.5, 0.0, 0.0));

        assert!(c.equals(WHITE));
    }
}