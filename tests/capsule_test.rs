use std::f64::consts::{FRAC_PI_2, PI};
use canvas_draw::color::Color;
use canvas_draw::color::consts::{BLUE, GREEN, WHITE};
use canvas_draw::ppm_format::PPMFile;
use linear_algebra::matrix::Matrix;
use linear_algebra::point::Point;
use linear_algebra::tuple_trait::Tuple;
use linear_algebra::vector::Vector;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::lights::Light;
use rusty_ray_tracer::features::lights::point_light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::{OneColorCreate, TwoPatternCreate};
use rusty_ray_tracer::features::patterns::solid::SolidPattern;
use rusty_ray_tracer::features::shapes::capsule::Capsule;
use rusty_ray_tracer::features::shapes::shape::Shape;
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn capsule_test() {
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

    // Upright capsule: shows the cylindrical body capped by a hemisphere at each end.
    let capsule = Shape::Capsule(Capsule::create()).create()
        .with_material(
            Material::create()
                .with_color(GREEN)
        )
        .with_transform(
            Matrix::translate(1.5, 3.0, 0.0)
        );

    // Horizontal, elongated capsule: rounded end faces the camera.
    let horizontal_capsule = Shape::Capsule(
        Capsule::create()
            .with_minimum_bound(-2.0)
            .with_maximum_bound(2.0)
    ).create()
        .with_material(
            Material::create()
                .with_color(BLUE)
                .with_reflective(0.1)
        )
        .with_transform(
            Matrix::translate(-1.0, 0.75, 1.5) * Matrix::rotate_z(FRAC_PI_2)
        );

    let light_source = PointLight::create(WHITE, Point::create(-5.0, 10.0, -10.0));
    let objects = vec![left_wall, right_wall, floor, capsule, horizontal_capsule];
    let world = World::create_world(Light::create_point_light(light_source), objects);

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("capsule_test.ppm"));
}
