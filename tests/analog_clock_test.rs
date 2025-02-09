use core::f64::consts::PI;
use canvas_draw::ppm_format::PPMFile;
use canvas_draw::canvas::Canvas;
use canvas_draw::color::Color;
use rusty_ray_tracer::features::primitives::matrix::Matrix;
use rusty_ray_tracer::features::primitives::point::Point;
use rusty_ray_tracer::features::primitives::tuple_trait::Tuple;

#[test]
#[ignore]
fn analog_clock_test() {
    let mut canvas = Canvas::create(300, 300);
    let color = Color::create(1.0,1.0,1.0);
    let clock_radius = canvas.width as f64 * 0.375;

    let center_x_coor = canvas.width / 2;
    let center_y_coor = canvas.height / 2;

    let twelve = Point::create(0.0, 0.0, 1.0);
    let num_of_permutations = 13;
    for n in 1..num_of_permutations {
        let rotation = Matrix::rotate_y(n as f64 * (PI/6.0));
        let clock_hand = rotation * twelve;

        let x_coor = (clock_hand.x() * clock_radius).round() as i32 + center_x_coor;
        let y_coor = (clock_hand.z() * clock_radius).round() as i32 + center_y_coor;

        canvas.write_pixel(x_coor, y_coor as i32, color);
    }

    canvas.convert_to_ppm_and_save(String::from("analog_clock_test.ppm"));
}
