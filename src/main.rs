use camera::Camera;
use material::{Lambertian, Metal};
use vector::{Color, Point3};
use hittable_list::HittableList;
use sphere::Sphere;

mod ray;
mod vector;
mod hittable;
mod hittable_list;
mod sphere;
mod common;
mod interval;
mod camera;
mod material;


fn main() {

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    // World
    let mut world = HittableList::new();
    // world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    // world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right));

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let samples_per_pixel = 100;
    let max_depth = 10;

    let mut camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    camera.render(world);
}
