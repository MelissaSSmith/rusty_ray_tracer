use core::f64::consts::{FRAC_PI_2, PI};
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::{BLUE, GREEN, RED, WHITE};
use rusty_ray_tracer::features::lights::point_light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::{OneColorCreate, TwoPatternCreate};
use rusty_ray_tracer::features::patterns::solid::SolidPattern;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::cone::Cone;
use rusty_ray_tracer::features::shapes::shape::Shape;
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn cone_test() {
    let camera = Camera::create(1080, 1080, PI/1.5)
        .with_transform(Matrix::view_transform(
            Point::create(3.0, 2.5, -2.5),
            Point::create(1.5, 3.0, 0.0),
            Vector::create(0.0, 1.0, 0.0)
        ));

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
                    SolidPattern::create(Color::create(0.5, 0.5, 0.5)),
                    SolidPattern::create(WHITE)

                ))
        )
        .with_transform(
            Matrix::translate(0.0, 0.0, 15.0) * Matrix::rotate_x(FRAC_PI_2)
        );

    let cone_x = Shape::Cone(
        Cone::create()
            .with_closed(true)
            .with_minimum_bound(-1.0)
            .with_maximum_bound(1.0)
    ).create()
        .with_material(
            Material::create()
                .with_color(RED)
                .with_diffuse(0.7)
                .with_specular(0.5)
                .with_reflective(0.1)
        )
        .with_transform(
            Matrix::translate(0.0, 2.0, 2.0) * Matrix::rotate_z(PI/2.0)
        );

    let cone_y = Shape::Cone(
        Cone::create()
            .with_closed(true)
            .with_minimum_bound(-1.0)
            .with_maximum_bound(1.0)
    ).create()
        .with_material(
            Material::create()
                .with_color(BLUE)
                .with_diffuse(0.7)
                .with_specular(0.5)
                .with_reflective(0.1)
        )
        .with_transform(
            Matrix::translate(-3.0, 2.0, 0.0) * Matrix::rotate_x(PI/2.0)
        );

    let cone_z = Shape::Cone(
        Cone::create()
            .with_minimum_bound(-1.0)
            .with_maximum_bound(1.0)
    ).create()
        .with_material(
            Material::create()
                .with_color(GREEN)
                .with_diffuse(0.7)
                .with_specular(0.5)
                .with_reflective(0.1)
        )
        .with_transform(
            Matrix::translate(3.0, 3.0, 2.0) * Matrix::rotate_x(PI/2.0)
        );

    let light_source = PointLight::create(WHITE, Point::create(-5.0, 10.0, -10.0));
    let objects = vec![floor, right_wall, left_wall, cone_x, cone_y, cone_z];
    let world = World::create_world(light_source, objects);

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("cone_test.ppm"));
}