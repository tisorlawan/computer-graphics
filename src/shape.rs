use crate::{Color, Point};

pub trait Shape {
    fn intersect_ray(&self, camera: Point, ray_direction: Point) -> Option<(f64, f64)>;
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    c: Point,
    r: f64,
    pub color: Color,
}

impl Sphere {
    pub fn new(c: Point, r: f64, color: Color) -> Self {
        Self { c, r, color }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Shape for Sphere {
    fn intersect_ray(&self, camera: Point, ray_direction: Point) -> Option<(f64, f64)> {
        let d = ray_direction;
        let o = camera;
        let co = o - self.c;

        let a = d.dot(d);
        let b = 2.0 * d.dot(co);
        let c = co.dot(co) - self.r.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        Some((t1, t2))
    }
}