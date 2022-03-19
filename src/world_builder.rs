use core::f64::consts::{FRAC_PI_2, PI};
use crate::features::color::Color;
use crate::features::color::consts::WHITE;
use crate::features::light::PointLight;
use crate::features::material::Material;
use crate::features::patterns::{OneColorCreate, TwoPatternCreate};
use crate::features::patterns::checkers::CheckerPattern;
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::shapes::shape::{Object, Shape};
use crate::features::world::World;

pub fn create_default_world_with_group(group: Object) -> World {
    let light_source = PointLight::create(WHITE, Point::create(-5.0, 25.0, -15.0));
    let objects = vec![group];
    World::create_world(light_source, objects)
}