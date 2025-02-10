use canvas_draw::color::Color;
use linear_algebra::matrix::Matrix;
use linear_algebra::point::Point;
use linear_algebra::tuple_trait::Tuple;
use crate::features::patterns::{MultiColorCreate, OneColorCreate, Pattern, PatternAt, PatternAtWithInverse, Patterns, TwoColorCreate, TwoPatternCreate};
use crate::features::patterns::solid::SolidPattern;

#[derive(Clone, Debug)]
pub struct StripePattern {
    patterns: Vec<Pattern>
}

impl TwoColorCreate for StripePattern {
    fn create(color_a: Color, color_b: Color) -> Pattern {
        let pattern = StripePattern {
            patterns: vec![SolidPattern::create(color_a), SolidPattern::create(color_b)]
        };
        Pattern::create(Box::new(Patterns::Stripe(pattern)))
    }
}

impl TwoPatternCreate for StripePattern {
    fn create(pattern_a: Pattern, pattern_b: Pattern) -> Pattern {
        let transform = Matrix::identity();
        let pattern = StripePattern {
            patterns: vec![pattern_a, pattern_b]
        };
        Pattern {
            pattern: Box::new(Patterns::Stripe(pattern)),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl MultiColorCreate for StripePattern {
    fn create(colors: Vec<Color>) -> Pattern {
        let transform = Matrix::identity();
        let patterns: Vec<Pattern> = colors.iter()
            .map(|c| SolidPattern::create(*c))
            .collect();
        let pattern = StripePattern {
            patterns
        };
        Pattern {
            pattern: Box::new(Patterns::Stripe(pattern)),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl PatternAtWithInverse for StripePattern {
    fn pattern_at(&self, point: &Point, inverse: &Matrix) -> Color {
        let tp = *inverse * *point;

        let index = (tp.x().floor().abs() as usize) % self.patterns.len();

        self.patterns[index].pattern_at(&tp)
    }
}

#[cfg(test)]
mod tests {
    use canvas_draw::color::consts::{BLACK, WHITE};
    use linear_algebra::matrix::Matrix;
    use linear_algebra::point::Point;
    use linear_algebra::tuple_trait::Tuple;
    use crate::features::patterns::{PatternAt, TwoColorCreate};
    use crate::features::patterns::stripe::StripePattern;
    use crate::features::shapes::shape::Shape;

    #[test]
    fn test_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::create(WHITE, BLACK);

        assert_eq!(pattern.pattern_at(&Point::zero()), WHITE);
        assert_eq!(pattern.pattern_at(&Point::create(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(&Point::create(0.0, 2.0, 0.0)), WHITE);
    }

    #[test]
    fn test_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::create(WHITE, BLACK);

        assert_eq!(pattern.pattern_at(&Point::zero()), WHITE);
        assert_eq!(pattern.pattern_at(&Point::create(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.pattern_at(&Point::create(0.0, 0.0, 2.0)), WHITE);
    }

    #[test]
    fn test_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::create(WHITE, BLACK);

        assert_eq!(pattern.pattern_at(&Point::zero()), WHITE);
        assert_eq!(pattern.pattern_at(&Point::create(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(&Point::create(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.pattern_at(&Point::create(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.pattern_at(&Point::create(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.pattern_at(&Point::create(-1.1, 0.0, 0.0)), WHITE);
    }

    #[test]
    fn test_stripes_with_an_object_transformation() {
        let object = Shape::Sphere.create()
            .with_transform(Matrix::scale(2.0, 2.0, 2.0));
        let pattern = StripePattern::create(WHITE, BLACK);

        let c = pattern.pattern_at_object(&object, &Point::create(1.5, 0.0, 0.0));

        assert_eq!(c, WHITE);
    }

    #[test]
    fn test_stripes_with_a_pattern_transformation() {
        let object = Shape::Sphere.create();
        let mut pattern = StripePattern::create(WHITE, BLACK);
        pattern.transform(Matrix::scale(2.0, 2.0, 2.0));

        let c = pattern.pattern_at_object(&object, &Point::create(1.5, 0.0, 0.0));

        assert_eq!(c, WHITE);
    }

    #[test]
    fn test_stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Shape::Sphere.create()
            .with_transform(Matrix::scale(2.0, 2.0, 2.0));
        let mut pattern = StripePattern::create(WHITE, BLACK);
        pattern.transform(Matrix::translate(0.5, 0.0, 0.0));

        let c = pattern.pattern_at_object(&object, &Point::create(2.5, 0.0, 0.0));

        assert_eq!(c, WHITE);
    }
}