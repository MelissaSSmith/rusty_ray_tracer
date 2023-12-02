use std::f64::consts::{FRAC_PI_2, PI};
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::{GREEN, WHITE};
use rusty_ray_tracer::features::lights::Light;
use rusty_ray_tracer::features::lights::point_light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::{OneColorCreate, TwoPatternCreate};
use rusty_ray_tracer::features::patterns::solid::SolidPattern;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::shape::Shape;
use rusty_ray_tracer::features::shapes::torus::Torus;
use rusty_ray_tracer::features::world::World;

#[test]
fn torus_test() {
    let camera = Camera::create(1080, 1080, PI/1.5)
        .with_transform(Matrix::view_transform(
            Point::create(5.0, 2.5, -5.5),
            Point::create(1.5, 3.0, 0.0),
            Vector::create(2.0, 4.0, 4.0)
        ));

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
                    SolidPattern::create(Color::create(0.5, 0.5, 0.5)),
                    SolidPattern::create(WHITE)

                ))
        )
        .with_transform(
            Matrix::translate(0.0, 0.0, 15.0) * Matrix::rotate_x(FRAC_PI_2)
        );

    let floor = Shape::Plane.create()
        .with_material(
            Material::create()
                .with_reflective(0.0)
                .with_pattern(CheckerPattern::create(
                    SolidPattern::create(WHITE),
                    SolidPattern::create(Color::create(0.5, 0.5, 0.5))
                ))
        )
        .with_transform(Matrix::translate(-10.0, -15.0, 5.0) * Matrix::rotate_y(FRAC_PI_2));

    let torus = Shape::Torus(
        Torus::create()
            .with_radius(1.0)
            .with_tube_radius(0.4)
    ).create()
        .with_material(
            Material::create()
                .with_color(GREEN)
        )
        .with_transform(
            Matrix::translate(0.0, 0.0, 0.0) * Matrix::rotate_y(FRAC_PI_2)
        );

    let light_source = PointLight::create(WHITE, Point::create(-5.0, 10.0, -10.0));
    let objects = vec![left_wall, right_wall, floor, torus];
    let world = World::create_world(Light::create_point_light(light_source), objects);

    let canvas = camera.render_scene(world);

    canvas.convert_to_ppm_and_save(String::from("torus_test.ppm"));
}