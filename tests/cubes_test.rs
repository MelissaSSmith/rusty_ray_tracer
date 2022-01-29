use std::f64::consts::FRAC_PI_2;
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::{BLACK, WHITE};
use rusty_ray_tracer::features::light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::solid::SolidPattern;
use rusty_ray_tracer::features::patterns::{OneColorCreate, TwoPatternCreate};
use rusty_ray_tracer::features::patterns::gradient::GradientPattern;
use rusty_ray_tracer::features::patterns::stripe::StripePattern;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::shape::Shape;
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn cubes_test() {
    let camera = Camera::create(1080, 1080, 1.65347)
        .with_transform(Matrix::view_transform(
            Point::create(5.0, 2.5, -7.5),
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
                    SolidPattern::create(WHITE),
                    SolidPattern::create(Color::create(0.5, 0.5, 0.5))
                ))
        )
        .with_transform(
            Matrix::translate(0.0, 0.0, 15.0) * Matrix::rotate_x(FRAC_PI_2)
        );

    let blue_cube = Shape::Cube.create()
        .with_material(
            Material::create()
                .with_pattern(
                    GradientPattern::create(
                        SolidPattern::create(Color::create(0.0, 0.0, 1.0)),
                        SolidPattern::create(BLACK)
                    )
                )
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(0.05)
        )
        .with_transform(
            Matrix::translate(0.0, 2.0, 0.0) * Matrix::scale(2.0, 2.0, 2.0)
        );

    let red_cube = Shape::Cube.create()
        .with_material(
            Material::create()
                .with_pattern(
                    GradientPattern::create(
                        SolidPattern::create(Color::create(1.0, 0.0, 0.0)),
                        SolidPattern::create(BLACK)
                    )
                )
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(0.05)
        )
        .with_transform(
            Matrix::translate(0.0, 5.0, 0.0) * Matrix::scale(1.0, 1.0, 1.0)
        );

    let green_cube = Shape::Cube.create()
        .with_material(
            Material::create()
                .with_pattern(
                    GradientPattern::create(
                        SolidPattern::create(Color::create(0.0, 1.0, 0.0)),
                        SolidPattern::create(BLACK)
                    )
                )
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(0.05)
        )
        .with_transform(
            Matrix::translate(0.0, 6.5, 0.0) * Matrix::scale(0.5, 0.5, 0.5)
        );


    let light_source = PointLight::create(WHITE, Point::create(-5.0, 10.0, -10.0));
    let objects = vec![floor, left_wall, right_wall, blue_cube, red_cube, green_cube];
    let world = World::create_world(light_source, objects);

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("cubes_test.ppm"));
}