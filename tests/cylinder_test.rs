use core::f64::consts::{FRAC_PI_2, PI};
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::{BLUE, GREEN, RED, WHITE};
use rusty_ray_tracer::features::light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::{OneColorCreate, Pattern, TwoPatternCreate};
use rusty_ray_tracer::features::patterns::solid::SolidPattern;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::cylinder::Cylinder;
use rusty_ray_tracer::features::shapes::shape::Shape;
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn cylinder_test() {
    let camera = Camera::create(1080, 1080, PI/1.5)
        .with_transform(Matrix::view_transform(
            Point::create(5.0, 2.5, -10.5),
            Point::create(1.5, 3.0, 0.0),
            Vector::create(0.0, 1.0, 0.0)
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

    let cylinder_x = Shape::Cylinder(Cylinder::create().with_closed(true)).create()
        .with_material(
            Material::create()
                .with_color(RED)
                .with_diffuse(0.7)
                .with_specular(0.5)
                .with_reflective(0.1)
        )
        .with_transform(
            Matrix::translate(0.0, 0.0, 0.0) * Matrix::rotate_z(PI/2.0)
        );

    let cylinder_y = Shape::Cylinder(Cylinder::create().with_closed(true)).create()
        .with_material(
            Material::create()
                .with_color(BLUE)
                .with_diffuse(0.7)
                .with_specular(0.5)
                .with_reflective(0.1)
        )
        .with_transform(
            Matrix::translate(0.0, 0.0, 0.0)
       );

    let cylinder_z = Shape::Cylinder(Cylinder::create().with_closed(true)).create()
        .with_material(
            Material::create()
                .with_color(GREEN)
                .with_diffuse(0.7)
                .with_specular(0.5)
                .with_reflective(0.1)
        )
        .with_transform(
            Matrix::translate(0.0, 0.0, 0.0) * Matrix::rotate_x(PI/2.0)
        );

    let shallow_cylinder = Shape::Cylinder(
        Cylinder::create()
            .with_minimum_bound(-2.0)
            .with_maximum_bound(2.0)
    ).create()
        .with_material(
            Material::create()
                .with_pattern(
                    CheckerPattern::create(
                        SolidPattern::create(WHITE),
                        SolidPattern::create(RED)
                    )
                )
                .with_diffuse(0.3)
                .with_specular(0.2)
                .with_reflective(0.00)
        )
        .with_transform(
            Matrix::translate(-3.0, 3.0, -4.0) * Matrix::rotate_x(PI/2.0)
        );

    let cylinder = Shape::Cylinder(
        Cylinder::create()
            .with_closed(true)
            .with_minimum_bound(-2.0)
            .with_maximum_bound(2.0)
    ).create()
        .with_material(
            Material::create()
                .with_pattern(
                    CheckerPattern::create(
                        SolidPattern::create(WHITE),
                        SolidPattern::create(RED)
                    )
                )
                .with_diffuse(0.3)
                .with_specular(0.2)
                .with_reflective(0.00)
        )
        .with_transform(
            Matrix::translate(-3.0, 6.0, -4.0) * Matrix::rotate_z(PI/2.0)
        );

    let refractive_cylinder = Shape::Cylinder(
        Cylinder::create()
            .with_closed(true)
            .with_minimum_bound(-2.0)
            .with_maximum_bound(2.0)
    ).create()
        .with_material(
            Material::create()
                .with_color(Color::create(0.1, 0.1, 0.1))
                .with_diffuse(0.3)
                .with_specular(0.2)
                .with_reflective(0.00)
                .with_transparency(1.0)
                .with_refractive_index(1.5)
        )
        .with_transform(
            Matrix::translate(5.0, 2.0, -4.0) * Matrix::rotate_z(PI/2.0)
        );

    let light_source = PointLight::create(WHITE, Point::create(-5.0, 10.0, -10.0));
    let objects = vec![right_wall, left_wall, cylinder_x, cylinder_y, cylinder_z, shallow_cylinder, cylinder, refractive_cylinder];
    let world = World::create_world(light_source, objects);

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("cylinder_test.ppm"));
}