use crate::raytracer::{Direction, Point};

type Intensity = f64;

#[derive(Debug, Clone, Copy)]
pub enum Light {
    Ambient(Intensity),
    Point(Intensity, Point),
    Directional(Intensity, Direction),
}
