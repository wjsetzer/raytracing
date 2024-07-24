use camera::Camera;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vector::{Color, Point3, Vec3};

mod camera;
mod common;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vector;

fn main() {
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    // let material_center = Dielectric::new(1.5);
    // let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.1);
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.1);

    // World
    let mut world = HittableList::new();
    // world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    // world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));
    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 10;
    let max_depth = 10;

    let mut camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        40.0,
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        10.0,
        3.4,
    );

    camera.render(world);
}
