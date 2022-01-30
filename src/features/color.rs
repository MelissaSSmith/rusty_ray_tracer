use std::ops::{Add, Mul, Sub};
use crate::features::primitives::operations::Operations;

#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub(crate) red: f64,
    pub(crate) green: f64,
    pub(crate) blue: f64
}

impl Color {
    pub fn create(red: f64, green: f64, blue: f64) -> Color {
        Color{red, green, blue}
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
        self.red.equals_low_epsilon(other_color.red) &&
            self.green.equals_low_epsilon(other_color.green) &&
            self.blue.equals_low_epsilon(other_color.blue)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs
        }
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue
        }
    }
}

pub mod consts {
    use crate::features::color::Color;

    pub const BLACK: Color = Color {red: 0.0, green: 0.0, blue: 0.0};
    pub const WHITE: Color = Color {red: 1.0, green: 1.0, blue: 1.0};
    pub const RED: Color = Color {red: 1.0, green: 0.0, blue:0.0};
    pub const GREEN: Color = Color {red: 0.0, green: 1.0, blue:0.0};
    pub const BLUE: Color = Color {red: 0.0, green: 0.0, blue:1.0};
}

#[cfg(test)]
mod tests {
    use crate::features::color::Color;
    use crate::features::primitives::operations::Operations;

    #[test]
    fn test_create_color() {
        let color = Color::create(-0.5, 0.4, 1.7);

        assert_eq!(-0.5, color.red);
        assert_eq!(0.4, color.green);
        assert_eq!(1.7, color.blue);
    }

    #[test]
    fn test_add_colors() {
        let color_a = Color::create(0.9, 0.6, 0.75);
        let color_b = Color::create(0.7, 0.1, 0.25);

        let new_color = color_a + color_b;

        assert_eq!(1.6, new_color.red);
        assert_eq!(0.7, new_color.green);
        assert_eq!(1.0, new_color.blue);
    }

    #[test]
    fn test_subtract_colors() {
        let color_a = Color::create(0.9, 0.6, 0.75);
        let color_b = Color::create(0.7, 0.1, 0.25);

        let new_color = color_a - color_b;

        assert!(new_color.red.equals(0.2));
        assert_eq!(0.5, new_color.green);
        assert_eq!(0.5, new_color.blue);
    }

    #[test]
    fn test_multiply_color_by_scalar() {
        let color_a = Color::create(0.2, 0.3, 0.4);
        let scalar = 2.0;

        let new_color = color_a * scalar;

        assert!(new_color.red.equals(0.4));
        assert_eq!(0.6, new_color.green);
        assert_eq!(0.8, new_color.blue);
    }

    #[test]
    fn test_multiply_colors() {
        let color_a = Color::create(1.0, 0.2, 0.4);
        let color_b = Color::create(0.9, 1.0, 0.1);

        let new_color = color_a * color_b;

        assert_eq!(0.9, new_color.red);
        assert_eq!(0.2, new_color.green);
        assert!(new_color.blue.equals(0.04));
    }
}