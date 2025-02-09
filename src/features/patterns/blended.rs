use canvas_draw::color::Color;
use crate::features::primitives::matrix::Matrix;
use crate::features::patterns::{Pattern, PatternAt, PatternAtWithInverse, Patterns, TwoPatternCreate};
use crate::features::primitives::point::Point;

#[derive(Clone, Debug)]
pub struct BlendedPattern {
    pattern_a: Box<Pattern>,
    pattern_b: Box<Pattern>
}

impl TwoPatternCreate for BlendedPattern {
    fn create(pattern_a: Pattern, pattern_b: Pattern) -> Pattern {
        let pattern = BlendedPattern {
            pattern_a: Box::new(pattern_a),
            pattern_b: Box::new(pattern_b)
        };
        Pattern::create(Box::new(Patterns::Blended(pattern)))
    }
}

impl PatternAtWithInverse for BlendedPattern {
    fn pattern_at(&self, point: &Point, inverse: &Matrix) -> Color {
        let tp = inverse.clone() * *point;
        let color_a = self.pattern_a.pattern_at(&tp) * 0.5;
        let color_b = self.pattern_b.pattern_at(&tp) * 0.5;

        color_a + color_b
    }
}

#[cfg(test)]
mod tests {
    use canvas_draw::color::Color;
    use canvas_draw::color::consts::{BLACK, WHITE};
    use crate::features::patterns::blended::BlendedPattern;
    use crate::features::patterns::{TwoPatternCreate, PatternAt, OneColorCreate};
    use crate::features::patterns::solid::SolidPattern;
    use crate::features::patterns::stripe::StripePattern;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;

    #[test]
    fn test_blended_pattern() {
        let pattern_a = StripePattern::create(SolidPattern::create(BLACK), SolidPattern::create(WHITE));
        let pattern_b = StripePattern::create(SolidPattern::create(WHITE), SolidPattern::create(BLACK));

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