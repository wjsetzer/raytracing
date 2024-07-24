use camera::Camera;
use common::{random_f64, random_range_f64};
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
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64 + 0.9 * random_f64());

            if choose_mat < 0.8 {
                // diffuse
                let albedo = Color::random() * Color::random();
                let material = Lambertian::new(albedo);
                world.add(Sphere::new(center, 0.2, material));
            } else if choose_mat < 0.95 {
                // metal
                let albedo = Color::random_range(0.5, 1.0);
                let fuzz = random_range_f64(0.0, 0.5);
                let material = Metal::new(albedo, fuzz);
                world.add(Sphere::new(center, 0.2, material));
            } else {
                // glass
                let material = Dielectric::new(1.5);
                world.add(Sphere::new(center, 0.2, material));
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let mut camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    camera.render(world);
}
