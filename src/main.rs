use computer_graphics::raytracer::{
    color, light::Light, shape::Sphere, Canvas, Point, Raytracer, Vector, Viewport,
};

fn raytracer_spheres_example() {
    const CANVAS_WIDTH: usize = 600;
    const CANVAS_HEIGHT: usize = 600;

    let mut raytracer = Raytracer::new(
        Point(-0.0, 0.0, 0.0),
        Viewport::new(Point(0.0, 0.0, 1.0), 1.0, 1.0),
        Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT),
    );

    let shapes = vec![
        Sphere::new(Point(0.0, -1.0, 3.0), 1.0, color::RED, 500.0, 0.2),
        Sphere::new(Point(-2.0, 0.0, 4.0), 1.0, color::GREEN, 500.0, 0.3),
        Sphere::new(Point(2.0, 0.0, 4.0), 1.0, color::BLUE, 10.0, 0.4),
        Sphere::new(Point(0.0, -5001.0, 0.0), 5000.0, color::YELLOW, 1000.0, 0.5),
    ];

    let lights = vec![
        Light::Ambient(0.1),
        Light::Point(0.6, Point(3.0, 10.0, -2.0)),
        Light::Directional(0.3, Vector(3.0, 0.0, -1.0)),
    ];
    raytracer.fill_canvas(shapes, &lights);

    raytracer
        .save_canvas_to_ppm_file("raytracer_spheres_example.ppm")
        .expect("failed create ppm file");
}

fn main() {
    raytracer_spheres_example();
}
