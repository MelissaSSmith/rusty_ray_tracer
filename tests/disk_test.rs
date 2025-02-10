use std::f64::consts::{FRAC_PI_2, PI};
use canvas_draw::color::Color;
use canvas_draw::color::consts::BLUE;
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
use rusty_ray_tracer::features::shapes::disk::Disk;
use rusty_ray_tracer::features::shapes::shape::Shape;
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn disk_test() {
    let camera = Camera::create(1080, 1080, 0.45)
        .with_transform(Matrix::view_transform(
            Point::create(0.0, 10.0, 0.0),
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

    let disk = Shape::Disk(
        Disk::create()
            .with_radius(2.0)
            .with_inner_radius(0.5)
            .with_height(1.0)
    ).create()
        .with_material(
            Material::create()
                .with_color(BLUE)
        )
        .with_transform(
            Matrix::translate(0.0, 0.0, 0.0) * Matrix::rotate_x(FRAC_PI_2)
        );

    let disk_2 = Shape::Disk(
        Disk::create()
            .with_radius(2.0)
            .with_inner_radius(1.0)
            .with_height(1.0)
    ).create()
        .with_material(
            Material::create()
                .with_color(Color::create(0.3, 0.3, 0.3))
                .with_diffuse(0.5)
                .with_specular(0.2)
                .with_reflective(0.25)
                .with_transparency(1.0)
                .with_refractive_index(3.2)
        )
        .with_transform(
            Matrix::translate(0.0, 2.0, 0.0) * Matrix::rotate_x(FRAC_PI_2)
        );

    let disk_3 = Shape::Disk(
        Disk::create()
            .with_radius(0.8)
            .with_inner_radius(0.6)
            .with_height(1.0)
            .with_phi_max(PI)
    ).create()
        .with_material(
            Material::create()
                .with_color(Color::create(0.3, 0.3, 0.3))
                .with_diffuse(0.5)
                .with_specular(0.2)
                .with_reflective(0.25)
                .with_transparency(1.0)
                .with_refractive_index(3.2)
        )
        .with_transform(
            Matrix::translate(0.0, 2.0, 0.0) * Matrix::rotate_x(FRAC_PI_2) * Matrix::rotate_y(PI)
        );

    let light_source = PointLight::create(Color::create(0.9, 0.9, 0.9), Point::create(0.0, 10.0, 0.0));
    let objects = vec![wall, disk, disk_2, disk_3];
    let world = World::create_world(Light::create_point_light(light_source), objects);

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("disk_test.ppm"));
}