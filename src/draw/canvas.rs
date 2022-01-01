use std::collections::HashMap;
use std::ops::Div;
use crate::features::color::Color;
use crate::draw::pixel::Pixel;

pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pub(crate) pixels: HashMap<String, Pixel>
}

impl Canvas {
    pub fn create(_width: i32, _height: i32) -> Canvas {
        let mut default_pixels = HashMap::<String, Pixel>::new();
        for x in 0.._width {
            for y in 0.._height {
                let default_color = Color::create((0.0,0.0,0.0));
                let key = Canvas::create_key(x, y);
                default_pixels.insert(key, Pixel::create(x, y, default_color));
            }
        }
        Canvas {
            width: _width,
            height: _height,
            pixels: default_pixels
        }
    }

    fn create_key(x: i32, y: i32) -> String {
        format!("{}.{}", x, y)
    }

    pub fn write_pixel(&mut self, _x: i32, _y: i32, _color: Color) {
        let key = Canvas::create_key(_x, _y);
        let pixel = self.pixels.get_mut(&key);
        match pixel {
            None => (),
            Some(p) => {p.color = _color}
        }
    }

    pub fn get_pixel(&self, _x: i32, _y: i32) -> &Pixel {
        let key = Canvas::create_key(_x, _y);
        let pixel = &self.pixels.get(&key);
        pixel.unwrap()
    }

    pub fn format_pixel_line(line_array: Vec<String>) -> String {
        let mut line_string: String = line_array.join(" ");
        if line_string.len() > 70 {
            let num_of_line_splits = line_string.len().div(70);
            for n in 0..num_of_line_splits {
                let mut position = 70 * (n+1);
                while position > 0 {
                    let char = line_string.chars().nth(position).unwrap();
                    if char != ' ' {
                        position -= 1;
                    } else {
                        line_string.replace_range(position..position+1, "\n");
                        position = 0;
                    }
                }
            }
        }
        line_string
    }
}

#[cfg(test)]
mod tests {
    use crate::draw::canvas::Canvas;
    use crate::features::color::Color;

    #[test]
    fn test_create_canvas() {
        let width = 10;
        let height = 20;

        let canvas = Canvas::create(width, height);

        assert_eq!(width, canvas.width);
        assert_eq!(height, canvas.height);
        assert_eq!(200, canvas.pixels.len());

        for key_value in canvas.pixels {
            let pixel = key_value.1;
            assert_eq!(0.0, pixel.color.red);
            assert_eq!(0.0, pixel.color.green);
            assert_eq!(0.0, pixel.color.blue);
        }
    }

    #[test]
    fn test_write_pixel() {
        let width = 10;
        let height = 20;

        let mut canvas = Canvas::create(width, height);

        let x = 2;
        let y = 3;
        let color = Color::create((1.0, 0.0, 0.0));

        canvas.write_pixel(x, y, color);

        let new_pixel = canvas.get_pixel(x, y);

        assert_eq!(1.0, new_pixel.color.red);
        assert_eq!(0.0, new_pixel.color.blue);
        assert_eq!(0.0, new_pixel.color.green);
    }
}