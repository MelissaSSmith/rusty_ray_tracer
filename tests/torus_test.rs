use std::f64::consts::{FRAC_PI_2};
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::{BLUE, RED};
use rusty_ray_tracer::features::lights::Light;
use rusty_ray_tracer::features::lights::point_light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::{OneColorCreate, TwoPatternCreate};
use rusty_ray_tracer::features::patterns::solid::SolidPattern;
use rusty_ray_tracer::features::patterns::stripe::StripePattern;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::shape::Shape;
use rusty_ray_tracer::features::shapes::torus::Torus;
use rusty_ray_tracer::features::world::World;

#[test]
fn torus_test() {
    let camera = Camera::create(1080, 1080, 0.45)
        .with_transform(Matrix::view_transform(
            Point::create(0.0, 6.0, -5.0),
            Point::zero(),
            Vector::create(0.0, 2.0, 2.0)
        ));

    let wall = Shape::Plane.create()
        .with_transform(
            Matrix::translate(-3.0, -3.0, 10.0)
        )
        .with_material(
            Material::create()
                .with_pattern(CheckerPattern::create(
                    SolidPattern::create(Color::create(0.15, 0.15,  0.15)),
                    SolidPattern::create(Color::create(0.85, 0.85, 0.85))
                ))
                .with_ambient(0.8)
                .with_diffuse(0.2)
                .with_specular(0.0)
        );

    let torus = Shape::Torus(
        Torus::create()
            .with_radius(1.0)
            .with_tube_radius(0.5)
    ).create()
        .with_material(
            Material::create()
                .with_color(RED)
        )
        .with_transform(
            Matrix::translate(0.25, 0.5, 0.0) * Matrix::rotate_y(FRAC_PI_2)
        );

    let light_source = PointLight::create(Color::create(0.9, 0.9, 0.9), Point::create(2.0, 10.0, -5.0));
    let objects = vec![wall, torus];
    let world = World::create_world(Light::create_point_light(light_source), objects);

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("torus_test.ppm"));
}