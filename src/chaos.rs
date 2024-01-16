use rand::prelude::ThreadRng;
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
    let mut rng = rand::thread_rng();
    while n < number_of_rectangles {
        let clear = n % 7 == 0;
        let x: i32 = (rng.gen::<f64>() * canvas.width() as f64) as i32;
        let y: i32 =  (rng.gen::<f64>() * canvas.height() as f64) as i32;
        let w: i32 = rng.gen_range(x, canvas.width()) + (rng.gen::<f64>() * 100.0) as i32;
        let h: i32 = rng.gen_range(y, canvas.width()) + (rng.gen::<f64>() * 100.0) as i32;
        let color = generate_color(rng, clear);
        canvas.write_rectangle(x, y, w, h, color);
        n += 1;
    }
}

fn generate_color(mut rng: ThreadRng, clear: bool) -> Color {
    if clear {
        return WHITE;
    }
    Color::create(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
}