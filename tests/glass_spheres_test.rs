use core::f64::consts::{FRAC_PI_2};
use canvas_draw::color::Color;
use canvas_draw::color::consts::WHITE;
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
use rusty_ray_tracer::features::patterns::{TwoColorCreate};
use rusty_ray_tracer::features::shapes::shape::Shape;
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn glass_sphere_test() {
    let camera = Camera::create(1080, 1080, 0.45)
        .with_transform(Matrix::view_transform(
            Point::create(0.0, 0.0, -5.0),
            Point::zero(),
            Vector::create(0.0, 1.0, 0.0)
        ));

    let wall = Shape::Plane.create()
        .with_transform(
            Matrix::translate(0.0, 0.0, 10.0) * Matrix::rotate_x(FRAC_PI_2)
        )
        .with_material(
            Material::create()
                .with_pattern(CheckerPattern::create(
                    Color::create(0.15, 0.15,  0.15),
                    Color::create(0.85, 0.85, 0.85)
                ))
                .with_ambient(0.8)
                .with_diffuse(0.2)
                .with_specular(0.0)
        );

    let glass_ball = Shape::Sphere.create()
        .with_material(
            Material::create()
                .with_color(WHITE)
                .with_ambient(0.0)
                .with_diffuse(0.0)
                .with_specular(0.9)
                .with_shininess(300.0)
                .with_reflective(0.9)
                .with_transparency(0.9)
                .with_refractive_index(1.5)
        );

    let hollow_center = Shape::Sphere.create()
        .with_transform(
            Matrix::scale(0.5, 0.5, 0.5)
        )
        .with_material(
            Material::create()
                .with_color(WHITE)
                .with_color(WHITE)
                .with_ambient(0.0)
                .with_diffuse(0.0)
                .with_specular(0.9)
                .with_shininess(300.0)
                .with_reflective(0.9)
                .with_transparency(0.9)
                .with_refractive_index(1.0000034)
        );

    let light_source = PointLight::create(Color::create(0.9, 0.9, 0.9), Point::create(2.0, 10.0, -5.0));
    let objects = vec![wall, glass_ball, hollow_center];
    let world = World::create_world(Light::create_point_light(light_source), objects);

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("glass_spheres_test.ppm"));
}