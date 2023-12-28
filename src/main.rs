use computer_graphics::{
    light::Light, shape::Sphere, Canvas, Point, Raytracer, Vector, Viewport, BLUE, GREEN, RED,
    YELLOW,
};

fn raytracer_spheres_example() {
    let mut raytracer = Raytracer::new(Point::origin(), Viewport::default(), Canvas::new(600, 600));

    let shapes = vec![
        Sphere::new(Point(0.0, -1.0, 3.0), 1.0, RED),
        Sphere::new(Point(2.0, 0.0, 4.0), 1.0, BLUE),
        Sphere::new(Point(-2.0, 0.0, 4.0), 1.0, GREEN),
        Sphere::new(Point(0.0, -5001.0, 0.0), 5000.0, YELLOW),
    ];

    let lights = vec![
        Light::Ambient(0.2),
        Light::Point(0.6, Point(2.0, 1.0, 0.0)),
        Light::Directional(0.2, Vector(1.0, 4.0, 4.0)),
    ];
    raytracer.fill_canvas(shapes, &lights);

    raytracer
        .save_canvas_to_ppm_file("raytracer_spheres_example.ppm")
        .expect("failed create ppm file");
}

fn main() {
    raytracer_spheres_example();
}
