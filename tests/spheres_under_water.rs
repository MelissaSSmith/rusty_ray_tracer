use std::f64::consts::FRAC_PI_2;
use rusty_ray_tracer::draw::ppm_format::PPMFile;
use rusty_ray_tracer::features::camera::Camera;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::color::consts::WHITE;
use rusty_ray_tracer::features::light::PointLight;
use rusty_ray_tracer::features::material::Material;
use rusty_ray_tracer::features::patterns::{OneColorCreate, OnePatternWithScale, TwoColorCreate};
use rusty_ray_tracer::features::patterns::perturb::PerturbedPattern;
use rusty_ray_tracer::features::patterns::radial_gradient::RadialGradientPattern;
use rusty_ray_tracer::features::patterns::solid::SolidPattern;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;
use rusty_ray_tracer::features::primitives::vector::Vector;
use rusty_ray_tracer::features::shapes::shape::{Object, Shape};
use rusty_ray_tracer::features::world::World;

#[test]
#[ignore]
fn spheres_under_water_test() {
    let camera = Camera::create(300, 300, 0.45)
        .with_transform(Matrix::view_transform(
            Point::create(0.0, 0.0, -5.0),
            Point::zero(),
            Vector::create(0.0, 1.0, 0.0)
        ));

    let ground = Shape::Plane.create()
        .with_transform(
            Matrix::translate(0.0, 0.0, 10.0) * Matrix::rotate_x(FRAC_PI_2)
        )
        .with_material(
            Material::create()
                .with_pattern(SolidPattern::create(
                    Color::create(0.15, 0.15, 0.15)
                ))
                .with_ambient(0.8)
                .with_diffuse(0.2)
                .with_specular(0.0)
        );

    let water = Shape::Plane.water()
        .with_transform(Matrix::translate(0.0, 0.0, -2.0) * Matrix::rotate_x(FRAC_PI_2));

    let sphere = Shape::Sphere.create()
        .with_transform(Matrix::scale(0.5, 0.5, 0.5))
        .with_material(Material::create()
            .with_ambient(0.1)
            .with_diffuse(0.7)
            .with_specular(0.3)
            .with_pattern(PerturbedPattern::create(
                RadialGradientPattern::create(Color::create(1.0, 0.0, 1.0), Color::create(0.5, 1.0, 0.1)),
                Some(4.0)))
        );

    let light_source = PointLight::create(WHITE, Point::create(2.0, 5.0, -5.0));
    let objects: Vec<Object> = vec![ground, water, sphere];
    let world = World::create_world(light_source, objects);

    let canvas = camera.render(world);

    canvas.convert_to_ppm_and_save(String::from("spheres_under_water.ppm"));
}