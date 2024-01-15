use std::f64::consts::FRAC_PI_2;
use crate::features::color::consts::WHITE;
use crate::features::lights::Light;
use crate::features::lights::point_light::PointLight;
use crate::features::material::Material;
use crate::features::patterns::OneColorCreate;
use crate::features::patterns::solid::SolidPattern;
use crate::features::primitives::matrix::Matrix;
use crate::features::primitives::point::Point;
use crate::features::primitives::tuple_trait::Tuple;
use crate::features::shapes::shape::Object;
use crate::features::shapes::shape::Shape::Plane;
use crate::features::world::World;

pub fn create_default_world_with_group(group: Object) -> World {
    let light_source = PointLight::create(WHITE, Point::create(-5.0, 25.0, -15.0));
    let objects = vec![group];
    World::create_world(Light::create_point_light(light_source), objects)
}

pub fn create_empty_world() -> World {
    let light_source = PointLight::create(WHITE, Point::create(-5.0, 25.0, -15.0));
    World::create_world(Light::create_point_light(light_source), vec![])
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
        &_ => {
            plane
        }
    }
}