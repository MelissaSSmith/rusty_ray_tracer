use crate::features::color::Color;
use crate::features::patterns::blended::BlendedPattern;
use crate::features::patterns::checkers::CheckerPattern;
use crate::features::patterns::gradient::GradientPattern;
use crate::features::patterns::perturb::PerturbedPattern;
use crate::features::patterns::radial_gradient::RadialGradientPattern;
use crate::features::patterns::ring::RingPattern;
use crate::features::patterns::solid::SolidPattern;
use crate::features::patterns::stripe::StripePattern;
use crate::features::patterns::test::TestPattern;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::shapes::shape::Object;

pub mod stripe;
pub mod gradient;
pub mod ring;
pub mod checkers;
pub mod radial_gradient;
pub mod blended;
pub mod solid;
pub mod perturb;
pub mod test;

#[derive(Clone)]
pub enum Patterns {
    Blended(BlendedPattern),
    Checkers(CheckerPattern),
    Gradient(GradientPattern),
    Perturb(PerturbedPattern),
    RadialGradient(RadialGradientPattern),
    Ring(RingPattern),
    Solid(SolidPattern),
    Stripe(StripePattern),
    Test(TestPattern)
}

#[derive(Clone)]
pub struct Pattern {
    pattern: Box<Patterns>,
    transformation: Matrix,
    inverse_transformation: Matrix
}

impl Pattern {
    pub fn create(pattern: Box<Patterns>) -> Pattern {
        let transform = Matrix::identity();
        Pattern {
            pattern,
            transformation: transform,
            inverse_transformation: transform.inverse()
        }
    }

    fn call_pattern_at(&self, pattern_point: & Point) -> Color {
        let pattern = self.pattern.as_ref();
        match &pattern {
            Patterns::Blended(p) => { p.pattern_at(&pattern_point, &self.inverse_transformation) }
            Patterns::Checkers(p) => { p.pattern_at(&pattern_point, &self.inverse_transformation) }
            Patterns::Gradient(p) => { p.pattern_at(&pattern_point, &self.inverse_transformation) }
            Patterns::Perturb(p) => { p.pattern_at(&pattern_point) }
            Patterns::RadialGradient(p) => { p.pattern_at(&pattern_point, &self.inverse_transformation) }
            Patterns::Ring(p) => { p.pattern_at(&pattern_point, &self.inverse_transformation) }
            Patterns::Solid(p) => { p.pattern_at(&pattern_point) }
            Patterns::Stripe(p) => { p.pattern_at(&pattern_point, &self.inverse_transformation) }
            Patterns::Test(p) => { p.pattern_at(&pattern_point) }
        }
    }

    pub fn equals(&self, other_pattern: &Pattern) -> bool {
        self.transformation.equals(other_pattern.clone().transformation) &&
            self.inverse_transformation.equals(other_pattern.clone().inverse_transformation) //todo: add patterns
    }

    pub fn transformation(&self) -> Matrix {
        self.transformation
    }

    pub fn inverse_transformation(&self) -> Matrix {
        self.inverse_transformation
    }

    pub fn pattern(&self) -> Patterns {
        self.pattern.as_ref().clone()
    }

    pub fn transform(&mut self, transform: Matrix) {
        self.transformation = self.transformation * transform;
        self.inverse_transformation = self.transformation.inverse();
    }

    pub fn with_transform(self, transform: Matrix) -> Pattern {
        Pattern {
            transformation: transform,
            inverse_transformation: transform.inverse(),
            ..self
        }
    }

    pub fn pattern_at_object(&self, object: &Object, point: &Point) -> Color {
        let object_point = object.inverse_transformation() * *point;
        let pattern_point = self.inverse_transformation() * object_point;

        self.call_pattern_at(&pattern_point)
    }
}

impl PatternAt for Pattern {
    fn pattern_at(&self, point: &Point) -> Color {
        Pattern::call_pattern_at(self, point)
    }
}

pub trait PatternAt {
    fn pattern_at(&self, point: &Point) -> Color;
}

pub trait PatternAtWithInverse {
    fn pattern_at(&self, point: &Point, inverse: &Matrix) -> Color;
}

pub trait EmptyCreate {
    fn create() -> Pattern;
}

pub trait OneColorCreate {
    fn create(color: Color) -> Pattern;
}

pub trait TwoColorCreate {
    fn create(color_a: Color, color_b: Color) -> Pattern;
}

pub trait OnePatternCreate {
    fn create(pattern: Pattern) -> Pattern;
}

pub trait TwoPatternCreate {
    fn create(pattern_a: Pattern, pattern_b: Pattern) -> Pattern;
}

pub trait OnePatternWithScale {
    fn create(pattern: Pattern, scale: Option<f64>) -> Pattern;
}