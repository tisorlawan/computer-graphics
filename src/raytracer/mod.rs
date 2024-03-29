use std::io::{BufWriter, Write};
use std::{fs::File, io, ops, path::Path};

use light::Light;
use shape::{Shape, Sphere};

pub mod light;
pub mod shape;

const RECURSION_DEPTH: usize = 3;
const EPSILON: f64 = 0.001;

type Direction = Vector;

pub mod color {
    use std::ops;

    pub const BLACK: Color = Color(0, 0, 0);
    pub const WHITE: Color = Color(255, 255, 255);
    pub const RED: Color = Color(255, 0, 0);
    pub const GREEN: Color = Color(0, 255, 0);
    pub const BLUE: Color = Color(0, 0, 255);
    pub const MAGENTA: Color = Color(255, 0, 255);
    pub const CYAN: Color = Color(0, 255, 255);
    pub const YELLOW: Color = Color(255, 255, 0);

    pub const BG_COLOR: Color = BLACK;

    #[derive(Debug, Clone, Copy)]
    pub struct Color(pub u8, pub u8, pub u8);

    impl Color {
        pub fn scale(&self, factor: f64) -> Color {
            let mut c = self.clone();
            let r = c.0 as f64 * factor as f64;
            let r: u8 = if r > 255.0 { 255 } else { r.floor() as u8 };
            let g = c.1 as f64 * factor as f64;
            let g: u8 = if g > 255.0 { 255 } else { g.floor() as u8 };
            let b = c.2 as f64 * factor as f64;
            let b: u8 = if b > 255.0 { 255 } else { b.floor() as u8 };

            c.0 = r;
            c.1 = g;
            c.2 = b;
            c
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
}

#[derive(Debug, Clone, Copy)]
pub struct Vector(pub f64, pub f64, pub f64);

impl Vector {
    pub fn dot(&self, other: Vector) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cos(&self, other: Vector) -> f64 {
        self.dot(other) / (self.length() * other.length())
    }

    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn unit(&self) -> Vector {
        let length = self.length();
        Vector(self.0 / length, self.1 / length, self.2 / length)
    }

    pub fn scale(&self, factor: f64) -> Vector {
        Vector(self.0 * factor, self.1 * factor, self.2 * factor)
    }

    pub fn mul(&self, o: Vector) -> Vector {
        Vector(self.0 * o.0, self.1 * 0.1, self.2 * 0.2)
    }
}

impl From<Point> for Vector {
    fn from(value: Point) -> Self {
        Vector(value.0, value.1, value.2)
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

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

impl From<Vector> for Point {
    fn from(value: Vector) -> Self {
        Point(value.0, value.1, value.2)
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

/// Canvas: the abstract rectangular surface on which we can color pixels (draw on).
pub struct Canvas {
    w: usize,
    h: usize,
    pixels: Vec<color::Color>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        let canvas = Self {
            w,
            h,
            pixels: std::iter::repeat(color::BG_COLOR).take(w * h).collect(),
        };
        canvas
    }

    // Convert canvas coordinate system to computer screen coordinate
    pub fn put_pixel(&mut self, x: i32, y: i32, color: color::Color) {
        let (sx, sy) = self.canvas_to_screen_coordinate(x, y);

        let i = sy as usize * self.w + sx as usize;
        if i < self.pixels.len() {
            self.pixels[i] = color;
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = color::BG_COLOR;
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

impl Viewport {
    pub fn new(center: Point, h: f64, w: f64) -> Self {
        Self { center, h, w }
    }
}

// n: normal surface, unit vector, perpendicular to surface at P
// s: specular exponent, -1.0 for non specular
fn compute_lighting(
    p: Point,
    n: Vector,
    v: Direction,
    lights: &[Light],
    s: f64,
    scenes: &[Sphere],
) -> f64 {
    let mut i = 0.0;

    for light in lights {
        match light {
            Light::Ambient(intensity) => i += intensity,
            _ => {
                let mut t_max = 1.0;
                let (l, intensity) = if let Light::Point(intensity, poisition) = *light {
                    ((poisition - p).into(), intensity)
                } else if let Light::Directional(intensity, direction) = *light {
                    t_max = f64::MAX;
                    (direction, intensity)
                } else {
                    unreachable!()
                };

                if closest_intersection(p, l, scenes, EPSILON, t_max).is_some() {
                    continue;
                }

                // diffuse
                let n_dot_l = n.dot(l);
                if n_dot_l > 0.0 {
                    i += intensity * n_dot_l / (n.length() * l.length());
                }

                // specular
                if s != -1.0 {
                    let r = reflect_ray(n, l);
                    let r_cos_v = r.cos(v);
                    if r_cos_v > 0.0 {
                        i += intensity * r_cos_v.powf(s);
                    }
                }
            }
        }
    }
    i
}

fn reflect_ray(n: Vector, ray: Vector) -> Vector {
    n.scale(2.0).scale(n.dot(ray)) - ray
}

fn closest_intersection<'a>(
    origin: Point,
    direction_vector: Direction,
    scenes: &'a [Sphere],
    t_min: f64,
    t_max: f64,
) -> Option<(f64, &'a Sphere)> {
    let mut closest_t = f64::MAX - 1.0;
    let mut closest_sphere: Option<&Sphere> = None;

    for shape in scenes {
        if let Some((t1, t2)) = shape.intersect_ray(origin, direction_vector) {
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

    closest_sphere.map(|e| (closest_t, e))
}

fn trace_ray(
    start: Point,                // usually camera
    direction_vector: Direction, // usually w.r.t viewport_point
    scenes: &[Sphere],
    lights: &[Light],
    t_min: f64,
    t_max: f64,
    recursion_depth: usize,
) -> color::Color {
    let intersection = closest_intersection(start, direction_vector.into(), scenes, t_min, t_max);

    match intersection {
        None => color::BG_COLOR,
        Some((closest_t, sphere)) => {
            let p = ray_at(start, direction_vector, closest_t);
            let local_color = sphere.color().scale(compute_lighting(
                p,
                sphere.normal(p),
                direction_vector.scale(-1.0).into(),
                lights,
                sphere.specular,
                scenes,
            ));
            if recursion_depth == 0 || sphere.reflective <= 0.0 {
                return local_color;
            }

            let reflected_ray = reflect_ray(sphere.normal(p), direction_vector.scale(-1.0).into());
            let reflected_color = trace_ray(
                p,
                reflected_ray,
                scenes,
                lights,
                EPSILON,
                f64::MAX,
                recursion_depth - 1,
            );
            local_color.scale(1.0 - sphere.reflective) + reflected_color.scale(sphere.reflective)
        }
    }
}

pub fn ray_at(origin: Point, direction_vector: Direction, t: f64) -> Point {
    origin + Point::from(direction_vector.scale(t))
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

    pub fn fill_canvas(&mut self, scenes: Vec<Sphere>, lights: &[Light]) {
        for y in -(self.canvas.h as i32) / 2..(self.canvas.h as i32 / 2) {
            for x in -(self.canvas.w as i32) / 2..(self.canvas.w as i32 / 2) {
                let vp_point = self.viewport_point_from_canvas_point(x as f64, y as f64);

                let color = trace_ray(
                    self.camera,
                    (vp_point - self.camera).into(),
                    &scenes,
                    lights,
                    1.0,
                    5000.0,
                    RECURSION_DEPTH,
                );
                self.canvas.put_pixel(x, y, color);
            }
        }
    }

    pub fn viewport_point_from_canvas_point(&self, x: f64, y: f64) -> Point {
        let vx = (x / self.canvas.w as f64) * self.vw.w;
        let vy = (y / self.canvas.h as f64) * self.vw.h;
        let vz = self.vw.center.z();

        Point(vx, vy, vz)
    }

    pub fn save_canvas_to_ppm_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        self.canvas.save_to_ppm_file(path.as_ref())
    }
}
