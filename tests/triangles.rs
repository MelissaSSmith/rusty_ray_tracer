use core::f64::consts::PI;
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::{BLUE, GREEN, RED, WHITE};
use rusty_ray_tracer::features::lights::Light;
use rusty_ray_tracer::features::lights::point_light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::patterns::checkers::CheckerPattern;
use rusty_ray_tracer::features::patterns::{Pattern, TwoColorCreate};
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::group::Group;
use rusty_ray_tracer::features::shapes::shape::{Object, Shape};
use rusty_ray_tracer::features::shapes::triangle::Triangle;
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn test_triangles() {
    let floor = Shape::Plane.create()
        .with_material(
            Material::create()
                .with_pattern(CheckerPattern::create(WHITE, Color::create(0.5, 0.5, 0.5)))
                .with_reflective(0.0)
        );

    let t1 = Shape::Triangle(
        Triangle::create(
            Point::create(0.0, 0.0, 0.0),
            Point::create(1.0, 0.0, 0.0),
            Point::create(0.5, 1.0, 0.5)
        )
    ).create().with_material(
        Material::create()
            .with_color(RED)
    );

    let t2 = Shape::Triangle(
        Triangle::create(
            Point::create(1.0, 0.0, 0.0),
            Point::create(1.0, 0.0, 1.0),
            Point::create(0.5, 1.0, 0.5)
        )
    ).create().with_material(
        Material::create()
            .with_color(RED)
    );

    let t3 = Shape::Triangle(
        Triangle::create(
            Point::create(0.0, 0.0, 0.0),
            Point::create(0.0, 0.0, 1.0),
            Point::create(0.5, 1.0, 0.5)
        )
    ).create().with_material(
        Material::create()
            .with_color(GREEN)
    );

    let t4 = Shape::Triangle(
        Triangle::create(
            Point::create(0.0, 0.0, 1.0),
            Point::create(1.0, 0.0, 1.0),
            Point::create(0.5, 1.0, 0.5)
        )
    ).create().with_material(
        Material::create()
            .with_color(BLUE)
    );

    let children = vec![t1, t2, t3, t4];
    let pyramid = Shape::Group(Group::create()).create()
        .with_transform(
            Matrix::translate(-1.5, 0.0, 0.0) *
                Matrix::rotate_y(PI / 5.0) *
                Matrix::scale(2.0, 2.0, 2.0)
        )
        .with_children(children);

    let cube = Shape::Cube.create()
        .with_material(
            Material::create()
                .with_reflective(1.0)
                .with_ambient(0.0)
                .with_diffuse(0.3)
                .with_specular(0.1)
                .with_shininess(100.0)
        )
        .with_transform(Matrix::translate(0.0, 1.0, 4.0) * Matrix::scale(100.0, 100.0, 0.00001));

    let light_source = PointLight::create(WHITE, Point::create(-20.0, 6.0, -7.0));
    let objects = vec![floor, pyramid, cube];
    let world = World::create_world(Light::create_point_light(light_source), objects);

    let mut camera = Camera::create(1080, 1080, PI/4.5);
    camera.set_transform(Matrix::view_transform(Point::create(3.0, 3.0, -6.0), Point::create(0.0, 1.0, 0.0), Vector::create(0.0, 1.0, 0.0)));

    let canvas = camera.render_scene(world);

    canvas.convert_to_ppm_and_save(String::from("triangle_test.ppm"));
}