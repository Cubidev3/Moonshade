use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64
}

impl Color {
    pub const WHITE: Color = Color { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 };
    pub const BLACK: Color = Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 };
    pub const RED: Color = Color { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 };
    pub const GREEN: Color = Color { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 };
    pub const BLUE: Color = Color { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 };
    pub const ZERO: Color = Color { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 };

    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Color {
        Color { red: red.abs(), green: green.abs(), blue: blue.abs(), alpha: alpha.abs() }
    }

    pub fn solid(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue, alpha: 1.0 }
    }

    pub fn with_alpha(color: Color, alpha: f64) -> Color {
        Color::new(color.red, color.green, color.blue, alpha)
    }

    pub fn grayscale(color: Color) -> Color {
        let gray: f64 = (color.red + color.green + color.blue) / 3.0;
        Color::new(gray, gray, gray, color.alpha)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.red + rhs.red, self.green + rhs.green, self.blue + rhs.blue, self.alpha + rhs.alpha)
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(self.red - rhs.red, self.green - rhs.green, self.blue - rhs.blue, self.alpha - rhs.alpha)
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.red * rhs.red, self.green * rhs.green, self.blue * rhs.blue, self.alpha * rhs.alpha)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs, self.alpha * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(rhs.red * self, rhs.green * self, rhs.blue * self, rhs.alpha * self)
    }
}

impl Div<Color> for Color {
    type Output = Color;

    fn div(self, rhs: Color) -> Self::Output {
        Color::new(self.red / rhs.red, self.green / rhs.green, self.blue / rhs.blue, self.alpha / rhs.alpha)
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        let factor: f64 = 1.0 / rhs;
        self * factor
    }
}

impl Div<Color> for f64 {
    type Output = Color;

    fn div(self, rhs: Color) -> Self::Output {
        Color::new(self / rhs.red, self / rhs.green, self / rhs.blue, self / rhs.alpha)
    }
}