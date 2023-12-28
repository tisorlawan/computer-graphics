use computer_graphics::{shape::Sphere, Canvas, Point, Raytracer, Viewport, CYAN, MAGENTA, YELLOW};

fn raytracer_spheres_example() {
    let mut raytracer = Raytracer::new(Point::origin(), Viewport::default(), Canvas::new(600, 600));

    let shapes = vec![
        Sphere::new(Point(0.0, -1.0, 3.0), 1.0, YELLOW),
        Sphere::new(Point(2.0, 0.0, 4.0), 1.0, CYAN),
        Sphere::new(Point(-2.0, 0.0, 8.0), 1.0, MAGENTA),
    ];
    raytracer.fill_canvas(shapes);

    raytracer
        .save_canvas_to_ppm_file("raytracer_spheres_example.ppm")
        .expect("failed create ppm file");
}

fn main() {
    raytracer_spheres_example();
}
