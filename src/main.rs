use computer_graphics::{
    raytracer::{self, color, light::Light, shape::Sphere, Point, Raytracer, Vector, Viewport},
    razterization::{self, P2},
};

#[allow(dead_code)]
fn raytracer_spheres_example() {
    const CANVAS_WIDTH: usize = 1000;
    const CANVAS_HEIGHT: usize = 1000;

    let mut raytracer = Raytracer::new(
        Point(-0.0, 0.0, 0.0),
        Viewport::new(Point(0.0, 0.0, 1.0), 1.0, 1.0),
        raytracer::Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT),
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

fn razterization_example() {
    let mut canvas = razterization::Canvas::new(800, 800);

    canvas.draw_line(P2(-200, -100), P2(240, 120), razterization::YELLOW);
    canvas.draw_line(P2(-50, -200), P2(60, 240), razterization::RED);
    canvas.draw_line(P2(0, 0), P2(300, 300), razterization::GREEN);
    canvas
        .save_to_ppm_file("razterization.ppm")
        .expect("failed create ppm file");
}

fn main() {
    raytracer_spheres_example();
    razterization_example();
}
