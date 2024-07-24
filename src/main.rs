use camera::Camera;
use vector::Point3;
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


fn main() {

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 10;

    let mut camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    camera.render(world);
}
