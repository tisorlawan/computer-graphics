use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy)]
pub struct P2(pub i32, pub i32);

pub const BLACK: Color = Color(0, 0, 0);

pub const RED: Color = Color(225, 0, 0);
pub const GREEN: Color = Color(0, 255, 0);
pub const YELLOW: Color = Color(225, 225, 0);

pub const BG_COLOR: Color = BLACK;

pub struct Canvas {
    pub w: usize,
    pub h: usize,
    pixels: Vec<Color>,
}

fn swap(p0: &mut P2, p1: &mut P2) {
    let tmp = *p0;
    *p0 = *p1;
    *p1 = tmp;
}

fn interpolate(i0: i32, d0: f64, i1: i32, d1: f64) -> Vec<(i32, i32)> {
    if i0 == i1 {
        return vec![(i0 as i32, d0 as i32)];
    }

    let mut values = Vec::with_capacity((i1 - i0 + 1) as usize);
    let slope = (d1 - d0) / (i1 as f64 - i0 as f64);
    let mut d = d0;

    for i in i0..=i1 {
        values.push((i as i32, d as i32));
        d += slope;
    }
    return values;
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Canvas {
        Canvas {
            w,
            h,
            pixels: std::iter::repeat(BG_COLOR).take(w * h).collect(),
        }
    }

    pub fn draw_line(&mut self, mut p0: P2, mut p1: P2, color: Color) {
        let (x0, x1) = (p0.0 as f64, p1.0 as f64);
        let (y0, y1) = (p0.1 as f64, p1.1 as f64);

        let dy = y1 - y0;
        let dx = x1 - x0;

        if dx.abs() > dy.abs() {
            // line is horizontal-ish
            if x0 > x1 {
                swap(&mut p0, &mut p1);
            }

            self.put_pixels(&interpolate(p0.0, p0.1 as f64, p1.0, p1.1 as f64), color);
        } else {
            // line is vertical-ish

            if y0 > y1 {
                swap(&mut p0, &mut p1);
            }

            self.put_pixels(
                &interpolate(p0.1, p0.0 as f64, p1.1, p1.0 as f64)
                    .into_iter()
                    .map(|(y, x)| (x, y))
                    .collect::<Vec<_>>(),
                color,
            );
        }
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        let (x, y) = self.canvas_to_screen_coordinate(x, y);

        self.pixels[y as usize * self.w + x as usize] = color;
    }

    pub fn put_pixels(&mut self, pixels: &[(i32, i32)], color: Color) {
        for (x, y) in pixels {
            let (x, y) = self.canvas_to_screen_coordinate(*x, *y);

            self.pixels[y as usize * self.w + x as usize] = color;
        }
    }
    pub fn save_to_ppm_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut f = BufWriter::new(
            File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path.as_ref())?,
        );
        f.write_fmt(format_args!("P3\n{} {}\n255\n", self.w, self.h))?;
        for pixel in &self.pixels {
            f.write_fmt(format_args!("{} {} {}\n", pixel.0, pixel.1, pixel.2))?;
        }
        Ok(())
    }

    fn canvas_to_screen_coordinate(&self, x: i32, y: i32) -> (i32, i32) {
        let sx = (self.w / 2) as i32 + x;
        let sy = (self.h / 2) as i32 - y;
        (sx, sy)
    }
}
