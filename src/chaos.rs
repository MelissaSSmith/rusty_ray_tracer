use rand::Rng;
use crate::features::canvas::Canvas;
use crate::features::color::Color;
use crate::features::color::consts::WHITE;

pub fn initialize_blank_canvas() -> Canvas {
    Canvas::create_with_default(1000, 1000, WHITE)
}

pub fn set_color(canvas: Canvas, color: Color) -> Canvas {
    Canvas::create_with_default(canvas.width(), canvas.height(), color)
}

pub fn draw_random_rectangles(mut canvas: &mut Canvas, number_of_rectangles: i32) {
    let mut n = 0;
    while n < number_of_rectangles {
        let mut rng = rand::thread_rng();
        let x: i32 = (rng.gen::<f64>() * (canvas.width() - 100) as f64) as i32;
        let y: i32 =  (rng.gen::<f64>() * (canvas.height() - 100) as f64) as i32;
        let w: i32 = 20 + (rng.gen::<f64>() * 100.0) as i32;
        let h: i32 = 20 + (rng.gen::<f64>() * 100.0) as i32;
        let r: f64 = rng.gen::<f64>() * 256.0;
        let g: f64 = rng.gen::<f64>() * 256.0;
        let b: f64 = rng.gen::<f64>() * 256.0;
        let color = Color::create(r.floor(), g.floor(), b.floor());
        println!("drawing rectangle with coordinates: {}, {}, {}, {} - Color: {}", x, y, w, h, color);
        canvas.write_rectangle(x, y, w, h, color);
        n += 1;
    }
}