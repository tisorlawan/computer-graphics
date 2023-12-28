use crate::{Point, Vector};

type Intensity = f64;
type Direction = Vector;

pub enum Light {
    Ambient(Intensity),
    Point(Intensity, Point),
    Directional(Intensity, Direction),
}
