use std::f64::consts::PI;
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::consts::WHITE;
use rusty_ray_tracer::features::light::PointLight;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::cylinder::Cylinder;
use rusty_ray_tracer::features::shapes::group::Group;
use rusty_ray_tracer::features::shapes::shape::{Object, Shape};
use rusty_ray_tracer::features::world::World;

fn hexagon_corner() -> Object {
    Shape::Sphere.create()
        .with_transform(Matrix::translate(0.0, 0.0, -1.0) * Matrix::scale(0.25, 0.25, 0.25))
}

fn hexagon_edge() -> Object {
    let cylinder = Cylinder::create()
        .with_maximum_bound(1.0)
        .with_minimum_bound(0.0);
    Shape::Cylinder(cylinder)
        .create()
        .with_transform(
            Matrix::translate(0.0, 0.0, -1.0) *
                Matrix::rotate_y(-PI/6.0) *
                Matrix::rotate_z(-PI/2.0) *
                Matrix::scale(0.25, 1.0, 0.25)

        )
}

fn hexagon_side() -> Object {
    let mut side = Shape::Group(Group::create()).create();
    side.add_child(hexagon_corner());
    side.add_child(hexagon_edge());

    side
}

fn hexagon() -> Object {
    let mut hex = Shape::Group(Group::create()).create()
        .with_transform(Matrix::translate(0.0, 0.75, 0.0) * Matrix::rotate_x(PI / 3.0));

    for n in 0..=5 {
        let side = hexagon_side()
            .with_transform(Matrix::rotate_y(n as f64 * PI / 3.0));
        hex.add_child(side);
    }

    hex
}

#[test]
fn test_hexagon() {
    let light_source = PointLight::create(WHITE, Point::create(-10.0, 10.0, -10.0));
    let objects = vec![hexagon()];
    let world = World::create_world(light_source, objects);

    let mut camera = Camera::create(1080, 1080, PI/3.0);
    camera.set_transform(Matrix::view_transform(Point::create(0.0, 1.5, -5.0), Point::create(0.0, 1.0, 0.0), Vector::create(0.0, 1.0, 0.0)));

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("hexagon_test.ppm"));
}