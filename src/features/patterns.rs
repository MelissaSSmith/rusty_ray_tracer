use std::any::Any;
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
    pattern: Patterns,
    transformation: Matrix,
    inverse_transformation: Matrix
}

impl Pattern {
    pub fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    pub fn inverse_transformation(&self) -> Matrix {
        self.inverse_transformation.clone()
    }

    pub fn pattern(&self) -> Patterns {
        self.pattern.clone()
    }

    pub fn transform(&mut self, transform: Matrix) {
        self.transformation = self.transformation.clone() * transform;
        self.inverse_transformation = self.transformation.inverse();
    }

    pub fn pattern_at_object(&self, object: &Object, point: &Point) -> Color {
        let object_point = object.inverse_transformation() * *point;
        let pattern_point = self.inverse_transformation() * object_point;

        match &self.pattern {
            Patterns::Blended(p) => { p.pattern_at(&pattern_point) }
            Patterns::Checkers(p) => { p.pattern_at(&pattern_point) }
            Patterns::Gradient(p) => { p.pattern_at(&pattern_point) }
            Patterns::Perturb(p) => { p.pattern_at(&pattern_point) }
            Patterns::RadialGradient(p) => { p.pattern_at(&pattern_point) }
            Patterns::Ring(p) => { p.pattern_at(&pattern_point) }
            Patterns::Solid(p) => { p.pattern_at(&pattern_point) }
            Patterns::Stripe(p) => { p.pattern_at(&pattern_point) }
            Patterns::Test(p) => { p.pattern_at(&pattern_point) }
        }
    }
}

pub trait PatternAt {
    fn pattern_at(&self, point: &Point) -> Color;
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
    fn create(pattern: Patterns) -> Pattern;
}

pub trait TwoPatternCreate {
    fn create(pattern_a: Patterns, pattern_b: Patterns) -> Pattern;
}

pub trait OnePatternWithScale {
    fn create(pattern: Patterns, scale: Option<f64>) -> Pattern;
}

pub trait PatternTrait: Any { //todo: phase out
    fn box_clone(&self) -> Box<dyn PatternTrait>;
    fn as_any(&self) -> &dyn Any;
    fn transformation(&self) -> Matrix;
    fn inverse_transformation(&self) -> Matrix;
    fn transform(&mut self, transform: Matrix);
    fn pattern_at(&self, point: &Point) -> Color;

    fn pattern_at_object(&self, object: &Object, point: &Point) -> Color {
        let object_point = object.inverse_transformation() * *point;
        let pattern_point = self.inverse_transformation() * object_point;

        self.pattern_at(&pattern_point)
    }
}

impl Clone for Box<dyn PatternTrait> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}