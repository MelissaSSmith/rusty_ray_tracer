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

const WIDTH: f64 = 1080.0;
const HEIGHT: f64 = 1080.0;

pub fn create_default_world_with_object(object: Object) -> World {
    let light_source = PointLight::create(WHITE, Point::create(-5.0, 25.0, -15.0));
    let objects = vec![object];
    World::create_world(Light::create_point_light(light_source), objects)
}

pub fn create_objects_from_input(input: &String) -> Object {
    let plane = Plane.create()
        .with_transform(
            Matrix::translate(0.0, 0.0, 0.0) * Matrix::rotate_x(FRAC_PI_2)
        ).with_material(
            Material::create()
                .with_pattern(SolidPattern::create(WHITE))
                .with_ambient(0.8)
                .with_diffuse(0.2)
                .with_specular(0.0)
        );
    match input.as_str() {
        "random_pixels" => {
            plane.with_material(
                Material::create()
                    .with_pattern(RandomPixelsPattern::create())
                    .with_ambient(0.8)
                    .with_diffuse(0.2)
                    .with_specular(0.0)
            )
        }
        "random_rectangles" => {
            let mut rng = rand::thread_rng();
            let w = rng.gen::<f64>() * (WIDTH - 100.0);
            let y = rng.gen::<f64>() * (HEIGHT - 100.0);
            let z = 20.0 + rng.gen::<f64>() * 100.0;
            let test_plane = Plane.create()
                .with_transform(
                    Matrix::translate(0.0, 0.0, z) * Matrix::rotate_x(FRAC_PI_2)
                ).with_material(
                Material::create()
                    .with_pattern(SolidPattern::create(GREEN))
            );
            Shape::Group(Group::create()).create()
                .with_children(vec![plane, test_plane])
        }
        &_ => {
            plane
        }
    }
}