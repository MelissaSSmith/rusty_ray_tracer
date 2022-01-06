use std::f64::consts::PI;
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::WHITE;
use rusty_ray_tracer::features::light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::matrix::Matrix;
use rusty_ray_tracer::features::point::Point;
use rusty_ray_tracer::features::shapes::plane::Plane;
use rusty_ray_tracer::features::shapes::Shape;
use rusty_ray_tracer::features::shapes::sphere::Sphere;
use rusty_ray_tracer::features::vector::Vector;
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn sphere_scene_test() {
    let mut material = Material::create();
    material.set_color(Color::create(1.0, 0.9, 0.9));
    material.set_specular(0.0);

    let mut middle = Sphere::create();
    middle.set_transform(Matrix::translate(-0.5, 1.0, 0.5));
    let mut mid_material = Material::create();
    mid_material.set_color(Color::create(0.1, 1.0, 0.5));
    mid_material.set_diffuse(0.7);
    mid_material.set_specular(0.3);
    middle.set_material(mid_material);

    let mut right = Sphere::create();
    let r_transform = Matrix::translate(1.5, 0.5, -0.5)
        .multiply(&Matrix::scale(0.5, 0.5, 0.5));
    right.set_transform(r_transform);
    let mut r_material = Material::create();
    r_material.set_color(Color::create(0.5, 1.0, 0.1));
    r_material.set_diffuse(0.7);
    r_material.set_specular(0.3);
    right.set_material(r_material);

    let mut left = Sphere::create();
    let l_transform = Matrix::translate(-1.5, 0.33, -0.75)
        .multiply(&Matrix::scale(0.33, 0.33, 0.33));
    left.set_transform(l_transform);
    let mut l_material = Material::create();
    l_material.set_color(Color::create(1.0, 0.8, 0.1));
    l_material.set_diffuse(0.7);
    l_material.set_specular(0.3);
    left.set_material(l_material);

    let mut backdrop = Plane::create();
    let backdrop_transform = Matrix::translate(0.0, 0.0, 2.0)
        .multiply(&Matrix::rotate_x(PI/2.0));
    backdrop.set_transform(backdrop_transform);

    let mut floor = Plane::create();
    floor.set_material(material);

    let light_source = PointLight::create(WHITE, Point::create(-10.0, 10.0, -10.0));
    let objects: Vec<Box<dyn Shape>> = vec![Box::new(floor), Box::new(backdrop),
                                            Box::new(middle), Box::new(right), Box::new(left)];
    let world = World::create_world(light_source, objects);

    let mut camera = Camera::create(100, 50, PI/3.0);
    camera.set_transform(Matrix::view_transform(Point::create(0.0, 1.5, -5.0), Point::create(0.0, 1.0, 0.0), Vector::create(0.0, 1.0, 0.0)));

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("sphere_and_planes_test.ppm"));
}