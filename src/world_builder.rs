use std::f64::consts::FRAC_PI_2;
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
    let floor = Shape::Plane.create()
        .with_material(
            Material::create()
                .with_reflective(0.0)
                .with_pattern(CheckerPattern::create(
                    SolidPattern::create(WHITE),
                    SolidPattern::create(Color::create(0.5, 0.5, 0.5))
                ))
        );

    let left_wall = Shape::Plane.create()
        .with_material(
            Material::create()
                .with_reflective(0.0)
                .with_pattern(CheckerPattern::create(
                    SolidPattern::create(WHITE),
                    SolidPattern::create(Color::create(0.5, 0.5, 0.5))
                ))
        )
        .with_transform(
            Matrix::translate(-15.0, 0.0, 0.0) * Matrix::rotate_z(FRAC_PI_2)
        );

    let right_wall = Shape::Plane.create()
        .with_material(
            Material::create()
                .with_reflective(0.0)
                .with_pattern(CheckerPattern::create(
                    SolidPattern::create(WHITE),
                    SolidPattern::create(Color::create(0.5, 0.5, 0.5))
                ))
        )
        .with_transform(
            Matrix::translate(0.0, 0.0, 15.0) * Matrix::rotate_x(FRAC_PI_2)
        );

    let light_source = PointLight::create(WHITE, Point::create(-5.0, 10.0, -10.0));
    let objects = vec![floor, left_wall, right_wall, group];
    World::create_world(light_source, objects)
}