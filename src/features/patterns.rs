use std::any::Any;
use crate::features::color::Color;
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
    Blended,
    Checkers,
    Gradient,
    Perturb,
    RadialGradient,
    Ring,
    Solid,
    Stripe,
    Test
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

        //self.pattern_at(&pattern_point)
        todo!() //need to delegate to the pattern
    }
}

pub trait PatternAt {
    fn pattern_at(&self, point: &Point) -> Color;
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