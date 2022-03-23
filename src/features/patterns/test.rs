use crate::features::color::Color;
use crate::features::patterns::{EmptyCreate, Pattern, PatternAt, Patterns};
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;

#[derive(Clone, Debug)]
pub struct TestPattern {}

impl EmptyCreate for TestPattern {
    fn create() -> Pattern {
        Pattern::create(Box::new(Patterns::Test(TestPattern {})))
    }
}

impl PatternAt for TestPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        Color::create(point.x(), point.y(), point.z())
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::patterns::EmptyCreate;
    use crate::features::patterns::test::TestPattern;
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;
    use crate::features::shapes::shape::Shape;

    #[test]
    fn test_default_pattern_transformation() {
        let pattern = TestPattern::create();

        assert_eq!(pattern.transformation, Matrix::identity());
    }

    #[test]
    fn test_assigning_a_transformation() {
        let mut pattern = TestPattern::create();
        pattern.transform(Matrix::translate(1.0, 2.0, 3.0));

        assert_eq!(pattern.transformation, Matrix::translate(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_pattern_with_an_object_transformation() {
        let shape = Shape::Sphere.create()
            .with_transform(Matrix::scale(2.0, 2.0, 2.0));
        let pattern = TestPattern::create();

        let color = pattern.pattern_at_object(&shape, &Point::create(2.0, 3.0, 4.0));

        assert!(color.equals(Color::create(1.0, 1.5, 2.0)));
    }

    #[test]
    fn test_pattern_with_a_pattern_transformation() {
        let shape = Shape::Sphere.create();
        let mut pattern = TestPattern::create();
        pattern.transform(Matrix::scale(2.0, 2.0, 2.0));

        let color = pattern.pattern_at_object(&shape, &Point::create(2.0, 3.0, 4.0));

        assert!(color.equals(Color::create(1.0, 1.5, 2.0)));
    }

    #[test]
    fn test_pattern_with_both_object_and_pattern_transformation() {
        let shape = Shape::Sphere.create()
            .with_transform(Matrix::scale(2.0, 2.0, 2.0));
        let mut pattern = TestPattern::create();
        pattern.transform(Matrix::translate(0.5, 1.0, 1.5));

        let color = pattern.pattern_at_object(&shape, &Point::create(2.5, 3.0, 3.5));

        assert!(color.equals(Color::create(0.75, 0.5, 0.25)));
    }
}