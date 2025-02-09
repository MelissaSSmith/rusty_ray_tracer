use std::f64::consts::{FRAC_PI_2, PI};
use canvas_draw::color::Color;
use canvas_draw::color::consts::WHITE;
use canvas_draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::lights::Light;
use rusty_ray_tracer::features::lights::point_light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::solid::SolidPattern;
use rusty_ray_tracer::features::patterns::{OneColorCreate, TwoColorCreate};
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::cylinder::Cylinder;
use rusty_ray_tracer::features::shapes::group::Group;
use rusty_ray_tracer::features::shapes::shape::Shape;
use rusty_ray_tracer::features::world::World;

#[test]
fn capsule_test() {
    let camera = Camera::create(1080, 1080, 0.45)
        .with_transform(Matrix::view_transform(
            Point::create(0.0, 0.0, -10.0),
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
    let end_one = Shape::Sphere.create()
        .with_transform(
            Matrix::translate(0.0, 0.0, -1.0) *
                Matrix::rotate_z(PI/2.0) *
                Matrix::rotate_y(0.1111) *
                Matrix::scale(0.25, 0.25, 0.25));
    let end_two = Shape::Sphere.create()
        .with_transform(
            Matrix::translate(0.94, -0.01, -0.46) *
                Matrix::rotate_z(PI/4.0) *
                Matrix::rotate_y(-0.1111) *
                Matrix::scale(0.25, 0.25, 0.25));
    let center = Shape::Cylinder(Cylinder::create()
                                     .with_maximum_bound(1.0)
                                     .with_minimum_bound(0.0))
        .create()
        .with_transform(
            Matrix::translate(0.0, 0.0, -1.0) *
                Matrix::rotate_y(-PI/6.0) *
                Matrix::rotate_z(-PI/2.0) *
                Matrix::scale(0.25, 1.0, 0.25)

        );

    let capsule = Shape::Group(Group::create()).create()
        .with_transform(Matrix::translate(0.0, 0.5, 0.0) * Matrix::rotate_x(PI / 3.0))
        .with_children(vec![end_one, end_two, center])
        .with_material(
            Material::create().with_pattern(SolidPattern::create(WHITE))
        );

    let capsule_new = Shape::Cylinder(Cylinder::create()
        .with_maximum_bound(1.0)
        .with_minimum_bound(0.0)
        .with_capsule(true))
        .create()
        .with_transform(
            Matrix::translate(0.0, 0.0, -2.0) *
                Matrix::rotate_y(PI/4.0) *
                Matrix::rotate_z(PI/2.0) *
                Matrix::scale(0.25, 1.0, 0.25)

        ).with_material(Material::create().with_pattern(SolidPattern::create(WHITE)));

    let light_source = PointLight::create(Color::create(0.9, 0.9, 0.9), Point::create(2.0, 10.0, -5.0));
    let objects = vec![wall, capsule, capsule_new];
    let world = World::create_world(Light::create_point_light(light_source), objects);
    let canvas = camera.render(world);
    canvas.convert_to_ppm_and_save(String::from("capsule_test.ppm"));
}