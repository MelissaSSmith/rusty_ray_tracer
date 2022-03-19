use core::f64::consts::PI;
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::WHITE;
use rusty_ray_tracer::features::light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::shape::{Object, Shape};
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn sphere_scene_test() {
    let material = Material::create()
        .with_color(Color::create(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let floor = Shape::Sphere.create()
        .with_material(material.clone())
        .with_transform(Matrix::scale(10.0, 0.01, 10.0));

    let lw_transform = Matrix::translate(0.0, 0.0, 5.0)
        * Matrix::rotate_y(-PI/4.0)
        * Matrix::rotate_x(PI/2.0)
        * Matrix::scale(10.0, 0.01, 10.0);
    let left_wall = Shape::Sphere.create()
        .with_material(material.clone())
        .with_transform(lw_transform);

    let rw_transform = Matrix::translate(0.0, 0.0, 5.0)
        * Matrix::rotate_y(PI/4.0)
        * Matrix::rotate_x(PI/2.0)
        * Matrix::scale(10.0, 0.01, 10.0);
    let right_wall = Shape::Sphere.create()
        .with_material(material.clone())
        .with_transform(rw_transform);

    let mid_material = Material::create()
        .with_color(Color::create(0.1, 1.0, 0.5))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let middle = Shape::Sphere.create()
        .with_material(mid_material)
        .with_transform(Matrix::translate(-0.5, 1.0, 0.5));

    let r_transform = Matrix::translate(1.5, 0.5, -0.5) * Matrix::scale(0.5, 0.5, 0.5);
    let r_material = Material::create()
        .with_color(Color::create(0.5, 1.0, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let right = Shape::Sphere.create()
        .with_material(r_material)
        .with_transform(r_transform);

    let l_transform = Matrix::translate(-1.5, 0.33, -0.75) * Matrix::scale(0.33, 0.33, 0.33);
    let l_material = Material::create()
        .with_color(Color::create(1.0, 0.8, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let left = Shape::Sphere.create()
        .with_material(l_material)
        .with_transform(l_transform);

    let light_source = PointLight::create(WHITE, Point::create(-10.0, 10.0, -10.0));
    let objects = vec![floor, left_wall, right_wall, middle, right, left];
    let world = World::create_world(light_source, objects);

    let mut camera = Camera::create(300, 250, PI/3.0);
    camera.set_transform(Matrix::view_transform(Point::create(0.0, 1.5, -5.0), Point::create(0.0, 1.0, 0.0), Vector::create(0.0, 1.0, 0.0)));

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("sphere_scene_test.ppm"));
}