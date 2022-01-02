use rusty_ray_tracer::draw::canvas::Canvas;
use rusty_ray_tracer::features::color::Color;
use rusty_ray_tracer::features::point::Point;
use rusty_ray_tracer::features::vector::Vector;

mod world;

#[test]
fn projectile_test() {
    let gravity = Vector::create(0.0, -0.1, 0.0);
    let wind= Vector::create(-0.01, 0.0, 0.0);
    let environment = world::Environment::create(gravity, wind);

    let mut canvas = Canvas::create(900, 880);
    let color = Color::create(1.0, 0.8, 0.6);

    let position = Point::create(0.0, 1.0, 0.0);
    let velocity = Vector::create(1.0, 1.8, 0.0).normalize().multiply(11.25);

    let mut projectile = world::Projectile::create(position, velocity);

    while projectile.position.value().y > 0.0 {
        let x = projectile.position.value().x.round() as i32;
        let y = canvas.height - projectile.position.value().y.round() as i32;
        canvas.write_pixel(x,y, color);

        projectile = world::tick(environment, projectile);
    }

    //canvas.convert_to_ppm_and_save(String::from("projectile_test.ppm"));
}