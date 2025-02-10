use canvas_draw::canvas::Canvas;
use canvas_draw::color::Color;
use canvas_draw::ppm_format::PPMFile;
use linear_algebra::point::Point;
use linear_algebra::tuple_trait::Tuple;
use linear_algebra::vector::Vector;

mod world;

#[test]
#[ignore]
fn projectile_test() {
    let gravity = Vector::create(0.0, -0.1, 0.0);
    let wind= Vector::create(-0.01, 0.0, 0.0);
    let environment = world::Environment::create(gravity, wind);

    let mut canvas = Canvas::create(900, 880);
    let color = Color::create(1.0, 0.8, 0.6);

    let position = Point::create(0.0, 1.0, 0.0);
    let velocity = Vector::create(1.0, 1.8, 0.0).normalize() * 11.25;

    let mut projectile = world::Projectile::create(position, velocity);

    while projectile.position.y() > 0.0 {
        let x = projectile.position.x().round() as i32;
        let y = canvas.height() - projectile.position.y().round() as i32;
        canvas.write_pixel(x,y, color);

        projectile = world::tick(environment, projectile);
    }

    canvas.convert_to_ppm_and_save(String::from("projectile_test.ppm"));
}