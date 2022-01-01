use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Color {
    pub(crate) red: f64,
    pub(crate) green: f64,
    pub(crate) blue: f64
}

impl Color {
    pub fn create(color: (f64, f64, f64)) -> Color {
        Color{red:color.0, green:color.1, blue:color.2}
    }

    pub fn add(&self, _color: Color) -> Color {
        Color {
            red: self.red.add(_color.red),
            green: self.green.add(_color.green),
            blue: self.blue.add(_color.blue)
        }
    }

    pub fn subtract(&self, _color: Color) -> Color {
        Color {
            red: self.red.sub(_color.red),
            green: self.green.sub(_color.green),
            blue: self.blue.sub(_color.blue)
        }
    }

    pub fn multiply_colors(&self, _color: Color) -> Color {
        Color {
            red: self.red.mul(_color.red),
            green: self.green.mul(_color.green),
            blue: self.blue.mul(_color.blue)
        }
    }

    pub fn multiply(&self, _scalar: f64) -> Color {
        Color {
            red: self.red.mul(_scalar),
            green: self.green.mul(_scalar),
            blue: self.blue.mul(_scalar)
        }
    }

    pub fn scale_color(&self) -> Color {
        Color {
            red: Color::scale_number(self.red),
            green: Color::scale_number(self.green),
            blue: Color::scale_number(self.blue)
        }
    }

    fn scale_number(number: f64) -> f64 {
        let new_number = number * 255.0;
        if new_number > 255.0 {
            return 255.0;
        }
        if new_number < 0.0 {
            return 0.0;
        }
        new_number.round()
    }

    pub fn format_color_string(&self) -> String {
        format!("{} {} {}", self.red, self.green, self.blue)
    }

    pub fn equals(&self, other_color: Color) -> bool {
        self.red == other_color.red &&
            self.green == other_color.green &&
            self.blue == other_color.blue
    }
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;

    #[test]
    fn test_create_color() {
        let tuple = (-0.5, 0.4, 1.7);
        let color = Color::create(tuple);

        assert_eq!(tuple.0, color.red);
        assert_eq!(tuple.1, color.green);
        assert_eq!(tuple.2, color.blue);
    }

    #[test]
    fn test_add_colors() {
        let color_a = Color::create((0.9, 0.6, 0.75));
        let color_b = Color::create((0.7, 0.1, 0.25));

        let new_color = color_a.add(color_b);

        assert_eq!(1.6, new_color.red);
        assert_eq!(0.7, new_color.green);
        assert_eq!(1.0, new_color.blue);
    }

    #[test]
    fn test_subtract_colors() {
        let color_a = Color::create((0.9, 0.6, 0.75));
        let color_b = Color::create((0.7, 0.1, 0.25));

        let new_color = color_a.subtract(color_b);

        assert!(equals(0.2, new_color.red));
        assert_eq!(0.5, new_color.green);
        assert_eq!(0.5, new_color.blue);
    }

    #[test]
    fn test_multiply_color_by_scalar() {
        let color_a = Color::create((0.2, 0.3, 0.4));
        let scalar = 2.0;

        let new_color = color_a.multiply(scalar);

        assert!(equals(0.4, new_color.red));
        assert_eq!(0.6, new_color.green);
        assert_eq!(0.8, new_color.blue);
    }

    #[test]
    fn test_multiply_colors() {
        let color_a = Color::create((1.0, 0.2, 0.4));
        let color_b = Color::create((0.9, 1.0, 0.1));

        let new_color = color_a.multiply_colors(color_b);

        assert_eq!(0.9, new_color.red);
        assert_eq!(0.2, new_color.green);
        assert!(equals(0.04, new_color.blue));
    }

    fn equals(a: f64, b: f64) -> bool {
        let epsilon = 0.00001;
        let diff = a - b;
        diff.abs() < epsilon
    }
}