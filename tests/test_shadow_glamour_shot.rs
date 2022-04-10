use core::f64::consts::PI;
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::{RED, WHITE};
use rusty_ray_tracer::features::lights::area_light::AreaLight;
use rusty_ray_tracer::features::lights::Light;
use rusty_ray_tracer::features::lights::spot_light::SpotLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::sequence::Sequence;
use rusty_ray_tracer::features::shapes::shape::Shape;
use rusty_ray_tracer::features::world::World;

#[test]
//#[ignore]
fn test_glamour_shot() {
    let cube = Shape::Cube.create()
        .with_material(
            Material::create()
                .with_color(Color::create(1.5, 1.5, 1.5))
                .with_ambient(1.0)
                .with_diffuse(0.0)
                .with_specular(0.0)
        )
        .with_transform(
            Matrix::translate(0.0, 3.0, 4.0) * Matrix::scale(1.0, 1.0, 0.01)
        )
        .with_has_shadow(false);

    let plane = Shape::Plane.create()
        .with_material(
            Material::create()
                .with_color(WHITE)
                .with_ambient(0.025)
                .with_diffuse(0.67)
                .with_specular(0.0)
        );

    let sphere_1 = Shape::Sphere.create()
        .with_material(
            Material::create()
                .with_color(RED)
                .with_ambient(0.1)
                .with_specular(0.0)
                .with_diffuse(0.6)
                .with_reflective(0.3)
        )
        .with_transform(
            Matrix::translate(0.5, 0.5, 0.0) * Matrix::scale(0.5, 0.5, 0.5)
        );

    let sphere_2 = Shape::Sphere.create()
        .with_material(
            Material::create()
                .with_color(Color::create(0.5, 0.5, 1.0))
                .with_ambient(0.1)
                .with_specular(0.0)
                .with_diffuse(0.6)
                .with_reflective(0.3)
        )
        .with_transform(Matrix::translate(-0.25, 0.33, 0.0) * Matrix::scale(0.33, 0.33, 0.33));

    let light_source = Light::SpotLight(
        SpotLight::create(Point::create(-1.0, 1.0, 2.0),
                          Color::create(1.5, 1.5, 1.5),
                          -10.0,
                          20.0)
    );
    let objects = vec![cube, plane, sphere_1, sphere_2];
    let world = World::create_world(light_source, objects);

    let mut camera = Camera::create(1000, 400, 0.7854);
    camera.set_transform(Matrix::view_transform(Point::create(-3.0, 1.0, 2.5), Point::create(0.0, 0.5, 0.0), Vector::create(0.0, 1.0, 0.0)));

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("shadow_glamour_shot.ppm"));
}