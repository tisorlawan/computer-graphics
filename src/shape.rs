use crate::{color::Color, Point, Vector};

pub trait Shape {
    fn intersect_ray(&self, camera: Point, ray_direction: Vector) -> Option<(f64, f64)>;
    fn normal(&self, p: Point) -> Vector;
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    c: Point,
    r: f64,
    pub color: Color,
    pub specular: f64,
    pub reflective: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, color: Color, specular: f64, reflective: f64) -> Self {
        Self {
            c: center,
            r: radius,
            color,
            specular,
            reflective,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Shape for Sphere {
    fn intersect_ray(&self, camera: Point, ray_direction: Vector) -> Option<(f64, f64)> {
        let d = ray_direction;
        let o = camera;
        let co = o - self.c;

        let a = d.dot(d);
        let b = 2.0 * d.dot(co.into());
        let c = co.dot(co) - self.r.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        Some((t1, t2))
    }

    fn normal(&self, p: Point) -> Vector {
        let cp: Vector = (p - self.c).into();
        cp.unit()
    }
}
