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

pub trait Pattern: Any {
    fn box_clone(&self) -> Box<dyn Pattern>;
    fn as_any(&self) -> &dyn Any;
    fn transformation(&self) -> Matrix;
    fn transform(&mut self, transform: Matrix);
    fn pattern_at(&self, point: Point) -> Color;

    fn pattern_at_object(&self, object: Object, point: Point) -> Color {
        let object_point = object.transformation().inverse() * point;
        let pattern_point = self.transformation().inverse() * object_point;

        self.pattern_at(pattern_point)
    }
}

impl Clone for Box<dyn Pattern> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}