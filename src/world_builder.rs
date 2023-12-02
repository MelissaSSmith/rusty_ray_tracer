use std::f64::consts::FRAC_PI_2;
use rand::Rng;
use crate::features::color::consts::{GREEN, WHITE};
use crate::features::lights::Light;
use crate::features::lights::point_light::PointLight;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::shapes::shape::Object;
use crate::features::world::World;
use crate::features::patterns::{EmptyCreate, OneColorCreate};
use crate::{Material, Matrix, Plane, RandomPixelsPattern, RandomRectanglesPattern, Shape, SolidPattern};
use crate::features::color::Color;
use crate::features::shapes::group::Group;
use crate::features::shapes::plane::Plane as OtherPlane;
use crate::Shape::Rectangle;

const WIDTH: f64 = 1080.0;
const HEIGHT: f64 = 1080.0;
const MAX_RECTANGLES: i32 = 2;

pub fn create_default_world_with_object(object: Object) -> World {
    let light_source = PointLight::create(WHITE, Point::create(-5.0, 25.0, -15.0));
    let objects = vec![object];
    World::create_world(Light::create_point_light(light_source), objects)
}

pub fn create_default_world_with_objects(objects: Vec<Object>) -> World {
    let light_source = PointLight::create(WHITE, Point::create(-5.0, 25.0, -15.0));
    World::create_world(Light::create_point_light(light_source), objects)
}

pub fn create_objects_from_input(input: &String) -> Vec<Object> {
    let plane = Plane.create()
        .with_transform(
            Matrix::translate(0.0, 0.0, 0.0) * Matrix::rotate_x(FRAC_PI_2)
        ).with_material(
            Material::create()
                .with_pattern(SolidPattern::create(WHITE))
                .with_ambient(1.0)
                .with_diffuse(0.2)
                .with_specular(0.0)
        );
    match input.as_str() {
        "random_pixels" => {
            vec![
                plane.with_material(
                    Material::create()
                        .with_pattern(RandomPixelsPattern::create())
                        .with_ambient(0.8)
                        .with_diffuse(0.2)
                        .with_specular(0.0)
                )
            ]
        }
        "random_rectangles" => {
            let mut rng = rand::thread_rng();
            let w = rng.gen::<f64>() * 0.1;
            let y = rng.gen::<f64>() * 0.1;
            let z = 20.0 + rng.gen::<f64>() * 100.0;
            let rectangle = OtherPlane::create()
                .with_width(0.25)
                .with_height(0.1);
            let test_plane = Rectangle(rectangle).create()
                .with_transform(
                    Matrix::translate(0.0, 0.0, z) * Matrix::rotate_x(FRAC_PI_2)
                ).with_material(
                Material::create()
                    .with_pattern(SolidPattern::create(GREEN))
            );
            vec![test_plane]
        }
        &_ => {
            vec![plane]
        }
    }
}