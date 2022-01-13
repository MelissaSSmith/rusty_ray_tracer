use std::any::Any;
use crate::features::color::Color;
use crate::features::matrix::Matrix;
use crate::features::patterns::Pattern;
use crate::features::point::Point;
use crate::features::shapes::Shape;

#[derive(Clone, PartialEq)]
pub struct StripePattern {
    color_a: Color,
    color_b: Color,
    transformation: Matrix
}

impl StripePattern {
    pub fn create(color_a: Color, color_b: Color) -> StripePattern {
        StripePattern {
            color_a,
            color_b,
            transformation: Matrix::create_identity()
        }
    }
}

impl Pattern for StripePattern {
    fn equals(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn box_clone(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_pattern_transformation(&mut self, transform: Matrix) {
        self.transformation = transform;
    }

    fn pattern_at(&self, point: Point) -> Color {
        if point.value().x.floor() % 2.0 == 0.0 {
            return self.color_a;
        }

        self.color_b
    }

    fn pattern_at_object(&self, object: Box<dyn Shape>, point: Point) -> Color {
        let object_point = object.transformation().inverse().multiply_point(point);
        let pattern_point = self.transformation.inverse().multiply_point(object_point);

        self.pattern_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::matrix::Matrix;
    use crate::features::patterns::Pattern;
    use crate::features::patterns::stripe::StripePattern;
    use crate::features::point::Point;
    use crate::features::shapes::Shape;
    use crate::features::shapes::sphere::Sphere;

    #[test]
    fn test_create_stripe_pattern() {
        let pattern = StripePattern::create(WHITE, BLACK);

        assert!(pattern.color_a.equals(WHITE));
        assert!(pattern.color_b.equals(BLACK));
    }

    #[test]
    fn test_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 1.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 2.0, 0.0)).equals(WHITE));
    }

    #[test]
    fn test_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 1.0)).equals(WHITE));
        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 2.0)).equals(WHITE));
    }

    #[test]
    fn test_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::create(WHITE, BLACK);

        assert!(pattern.pattern_at(Point::create(0.0, 0.0, 0.0)).equals(WHITE));
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
        pattern.set_pattern_transformation(Matrix::scale(2.0, 2.0, 2.0));

        let c = pattern.pattern_at_object(Box::new(object), Point::create(1.5, 0.0, 0.0));

        assert!(c.equals(WHITE));
    }

    #[test]
    fn test_stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = Sphere::create();
        object.set_transform(Matrix::scale(2.0, 2.0, 2.0 ));
        let mut pattern = StripePattern::create(WHITE, BLACK);
        pattern.set_pattern_transformation(Matrix::translate(0.5, 0.0, 0.0));

        let c = pattern.pattern_at_object(Box::new(object), Point::create(2.5, 0.0, 0.0));

        assert!(c.equals(WHITE));
    }
}