use std::io::{BufWriter, Write};
use std::{fs::File, io, ops, path::Path};

use shape::{Shape, Sphere};

pub mod shape;

const BG_COLOR: Color = Color(30, 30, 30);

pub const RED: Color = Color(210, 50, 50);
pub const GREEN: Color = Color(50, 210, 50);
pub const BLUE: Color = Color(50, 50, 210);
pub const MAGENTA: Color = Color(210, 50, 210);
pub const CYAN: Color = Color(50, 210, 210);
pub const YELLOW: Color = Color(210, 210, 50);

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64, pub f64);

impl Point {
    pub fn origin() -> Point {
        Point(0.0, 0.0, 0.0)
    }

    pub fn scale(&self, s: f64) -> Point {
        Point(self.0 * s, self.1 * s, self.2 * s)
    }

    pub fn dot(&self, other: Point) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }
}

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

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
        let canvas = Self {
            w,
            h,
            pixels: std::iter::repeat(BG_COLOR).take(w * h).collect(),
        };
        canvas
    }

    // Convert canvas coordinate system to computer screen coordinate
    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        let (sx, sy) = self.canvas_to_screen_coordinate(x, y);

        let i = sy as usize * self.w + sx as usize;
        if i < self.pixels.len() {
            self.pixels[i] = color;
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = BG_COLOR;
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

/// The scene is 3D the set of objects rendered in the `Canvas`.
pub struct Scene;

pub struct Viewport {
    center: Point,
    h: f64,
    w: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            center: Point(0.0, 0.0, 1.0),
            w: 1.0,
            h: 1.0,
        }
    }
}

pub struct Raytracer {
    camera: Point,
    vw: Viewport,
    canvas: Canvas,
}

impl Raytracer {
    pub fn new(camera: Point, viewport: Viewport, canvas: Canvas) -> Raytracer {
        Raytracer {
            camera,
            vw: viewport,
            canvas,
        }
    }

    pub fn fill_canvas(&mut self, scenes: Vec<Sphere>) {
        for y in -(self.canvas.h as i32) / 2..(self.canvas.h as i32 / 2) {
            for x in -(self.canvas.w as i32) / 2..(self.canvas.w as i32 / 2) {
                let vp_point = self.viewport_point_from_canvas_point(x as f64, y as f64);
                let color = self.trace_ray(vp_point, &scenes, 1.0, 100.0);
                self.canvas.put_pixel(x, y, color);
            }
        }
    }

    pub fn trace_ray(
        &self,
        viewport_point: Point,
        scenes: &[Sphere],
        t_min: f64,
        t_max: f64,
    ) -> Color {
        let mut closest_t = f64::MAX - 1.0;
        let mut closest_sphere: Option<&Sphere> = None;

        let d = viewport_point - self.camera;
        for shape in scenes {
            if let Some((t1, t2)) = shape.intersect_ray(self.camera, d) {
                if t_min <= t1 && t1 <= t_max && t1 < closest_t {
                    closest_t = t1;
                    closest_sphere = Some(shape);
                }

                if t_min <= t2 && t2 <= t_max && t2 < closest_t {
                    closest_t = t2;
                    closest_sphere = Some(shape);
                }
            }
        }

        match closest_sphere {
            None => BG_COLOR,
            Some(sphere) => sphere.color(),
        }
    }

    pub fn viewport_point_from_canvas_point(&self, x: f64, y: f64) -> Point {
        let vx = (x / self.canvas.w as f64) * self.vw.w;
        let vy = (y / self.canvas.h as f64) * self.vw.h;
        let vz = self.vw.center.z();

        Point(vx, vy, vz)
    }

    pub fn ray_at(&self, viewport_point: Point, t: f64) -> Point {
        let direction = viewport_point - self.camera;
        direction.scale(t) + self.camera
    }

    pub fn save_canvas_to_ppm_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        self.canvas.save_to_ppm_file(path.as_ref())
    }
}
