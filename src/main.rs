use std::ops;

#[derive(Debug)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn intensify(&mut self, factor: f64) {
        let r = self.0 as f64 * factor as f64;
        let r: u8 = if r > 255.0 { 255 } else { r.floor() as u8 };
        let g = self.1 as f64 * factor as f64;
        let g: u8 = if g > 255.0 { 255 } else { g.floor() as u8 };
        let b = self.2 as f64 * factor as f64;
        let b: u8 = if b > 255.0 { 255 } else { b.floor() as u8 };

        self.0 = r;
        self.1 = g;
        self.2 = b;
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0.saturating_add(rhs.0),
            self.1.saturating_add(rhs.1),
            self.2.saturating_add(rhs.2),
        )
    }
}

/// Canvas: the abstract rectangular surface on which we can color pixels (draw on).
pub struct Canvas {
    w: usize,
    h: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            pixels: Vec::with_capacity(w * h),
        }
    }

    // Convert canvas coordinate system to computer screen coordinate
    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        let (sx, sy) = self.canvas_to_screen_coordinate(x, y);

        let i = sy as usize * self.w + sx as usize;
        self.pixels[i] = color;
    }

    fn canvas_to_screen_coordinate(&self, x: i32, y: i32) -> (i32, i32) {
        let sx = (self.w / 2) as i32 + x;
        let sy = (self.h / 2) as i32 - y;
        (sx, sy)
    }
}

/// The scene is 3D the set of objects rendered in the `Canvas`.
pub struct Scene;

fn main() {
    let mut c = Color(100, 200, 80);
    c.intensify(2.1);
    println!("{:?}", c);
}
