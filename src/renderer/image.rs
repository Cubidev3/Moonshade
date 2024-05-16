use crate::math::color::Color;
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image { width, height, pixels: vec![Color::BLACK; width * height] }
    }

    pub fn paint(&mut self, x: usize, y: usize, color: Color) {
        if self.is_coordinate_valid(x, y) {
            self.pixels[x + y * self.width] = color;
        }
    }
    
    pub fn paint_all(&mut self, info: Vec<(usize, usize, Color)>) {
        for (x, y, color) in info.iter() {
            self.paint(*x, *y, *color);
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = Color::BLACK;
        }
    }

    pub fn color_at(&self, x: usize, y: usize) -> Option<Color> {
        if !self.is_coordinate_valid(x, y) {
            return None;
        }

        Some(self.pixels[x + y * self.width])
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn resolution(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    
    pub fn pixel_count(&self) -> usize {
        self.width * self.height
    }

    pub fn pixel_positions(&self) -> PixelPositionIterator {
        PixelPositionIterator { x: 0, y: 0, width: self.width, height: self.height }
    }

    pub fn is_coordinate_valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn pbm(&self) -> String {
        let colors: String = self.pixels.iter()
            .map(|color| {
                let red = (color.red * 255.0) as usize;
                let green = (color.green * 255.0) as usize;
                let blue = (color.blue * 255.0) as usize;

                format!("{} {} {}", red, green, blue)
            })
            .collect::<Vec<_>>()
            .join("\n");
        
        format!("P3\n{} {}\n255\n{}", self.width, self.height, colors)
    }
}

pub struct PixelPositionIterator {
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

impl Iterator for PixelPositionIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        if self.y >= self.height {
            return None;
        }

        self.x += 1;
        Some((self.x, self.y))
    }
}