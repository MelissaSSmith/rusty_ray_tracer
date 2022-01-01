use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::draw::canvas::Canvas;

trait PPMFormat {
    fn create_header(&self) -> String;
    fn create_pixel_data(&self) -> String;
    fn create_termination() -> String;
    fn convert_to_ppm(&self) -> String;
    fn write_to_file(file_name: String, file_data: String);
}

pub trait PPMFile {
    fn convert_to_ppm_and_save(&self, file_name: String);
}

impl PPMFormat for Canvas {
    fn create_header(&self) -> String {
        format!("P3\n{} {}\n255", self.width, self.height)
    }

    fn create_pixel_data(&self) -> String {
        let width = self.width;
        let height = self.height;
        let mut color_array = Vec::<String>::new();
        for h in 0..height {
            let mut line_array = Vec::<String>::new();
            for w in 0..width {
                let pixel = self.get_pixel(w, h);
                let scaled_color = pixel.color.scale_color();
                line_array.push(scaled_color.format_color_string());
            }

            color_array.push(Canvas::format_pixel_line(line_array));
        }

        String::from(&color_array.join("\n"))
    }

    fn create_termination() -> String {
        String::from("\n")
    }

    fn convert_to_ppm(&self) -> String {
        let header = self.create_header();
        let pixel_data = self.create_pixel_data();
        let termination = Canvas::create_termination();

        format!("{}\n{}{}", header, pixel_data, termination)
    }

    fn write_to_file(file_name: String, file_data: String) {
        let path = Path::new(&file_name);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(file_data.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => (),
        };
    }
}

impl PPMFile for Canvas {
    fn convert_to_ppm_and_save(&self, file_name: String) {
        let ppm = self.convert_to_ppm();
        Canvas::write_to_file(file_name, ppm);
    }
}

#[cfg(test)]
mod tests {
    use crate::draw::canvas::Canvas;
    use crate::features::color::Color;
    use crate::draw::ppm_format::PPMFormat;

    #[test]
    fn test_convert_canvas_to_ppm_header() {
        let width = 5;
        let height = 3;
        let canvas = Canvas::create(width, height);

        let result = canvas.create_header();

        let expected = String::from("P3\n5 3\n255");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_canvas_to_ppm_pixel_data() {
        let width = 5;
        let height = 3;
        let mut canvas = Canvas::create(width, height);

        let color_a = Color::create((1.5, 0.0, 0.0));
        let color_b = Color::create((0.0, 0.5, 0.0));
        let color_c = Color::create((-0.5, 0.0, 1.0));

        canvas.write_pixel(0, 0, color_a);
        canvas.write_pixel(2, 1, color_b);
        canvas.write_pixel(4, 2, color_c);

        let pixel_data = canvas.create_pixel_data();

        let expected = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255";
        assert_eq!(expected, pixel_data);
    }

    #[test]
    fn test_splitting_long_lines_when_convert_canvas_to_ppm_pixel_data() {
        let width = 10;
        let height = 2;
        let mut canvas = Canvas::create(width, height);

        for x in 0..width {
            for y in 0..height {
                let color = Color::create((1.0,0.8,0.6));
                canvas.write_pixel(x, y, color);
            }
        }

        let pixel_data = canvas.create_pixel_data();

        let expected = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 \
        255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \
        255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153";
        assert_eq!(expected, pixel_data);
    }

    #[test]
    fn test_convert_canvas_to_ppm_termination_line() {
        let result = Canvas::create_termination();

        let expected = String::from("\n");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_to_ppm() {
        let width = 10;
        let height = 2;
        let mut canvas = Canvas::create(width, height);

        for x in 0..width {
            for y in 0..height {
                let color = Color::create((1.0,0.8,0.6));
                canvas.write_pixel(x, y, color);
            }
        }

        let ppm = canvas.convert_to_ppm();

        let expected = "P3\n10 2\n255\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 \
        255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 \
        255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n";
        assert_eq!(expected, ppm);
    }

}