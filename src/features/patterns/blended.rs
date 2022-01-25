use crate::features::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::{Pattern, PatternAt, Patterns, TwoPatternCreate};
use crate::features::primitives::point::Point;

#[derive(Clone)]
pub struct BlendedPattern {
    pattern_a: Patterns,
    pattern_b: Patterns
}

impl TwoPatternCreate for BlendedPattern {
    fn create(pattern_a: Patterns, pattern_b: Patterns) -> Pattern {
        let transform = Matrix::identity();
        let pattern = BlendedPattern {
            pattern_a,
            pattern_b
        };
        Pattern {
            pattern: Patterns::Blended(pattern),
            transformation: transform.clone(),
            inverse_transformation: transform.inverse()
        }
    }
}

impl PatternAt for BlendedPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let tp = self.inverse_transformation() * *point;
        let color_a = self.pattern_a.pattern_at(&tp) * 0.5;
        let color_b = self.pattern_b.pattern_at(&tp) * 0.5;

        color_a + color_b
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::color::consts::{BLACK, WHITE};
    use crate::features::patterns::blended::BlendedPattern;
    use crate::features::patterns::{TwoPatternCreate, TwoColorCreate, Patterns};
    use crate::features::patterns::stripe::StripePattern;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;

    #[test]
    fn test_blended_pattern() {
        let pattern_a = Patterns::Stripe(StripePattern::create(BLACK, WHITE));
        let pattern_b = Patterns::Stripe(StripePattern::create(WHITE, BLACK));

        let pattern = BlendedPattern::create(pattern_a, pattern_b);

        let color = Color::create(0.5, 0.5, 0.5);
        assert!(pattern.pattern_at(&Point::zero()).equals(color));
        assert!(pattern.pattern_at(&Point::create(0.9, 0.0, 0.0)).equals(color));
        assert!(pattern.pattern_at(&Point::create(1.0, 0.0, 0.0)).equals(color));
        assert!(pattern.pattern_at(&Point::create(-0.1, 0.0, 0.0)).equals(color));
        assert!(pattern.pattern_at(&Point::create(-1.0, 0.0, 0.0)).equals(color));
        assert!(pattern.pattern_at(&Point::create(-1.1, 0.0, 0.0)).equals(color));
    }
}