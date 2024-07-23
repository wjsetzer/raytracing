use std::fs::File;
use std::io::Write;

use interval::Interval;
use ray::Ray;
use vector::{Color, Point3, unit_vector, Vec3};
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;
use common::INFINITY;

mod ray;
mod vector;
mod hittable;
mod hittable_list;
mod sphere;
mod common;
mod interval;
mod camera;

// fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
//     let oc = center - r.origin();
//     // let a = vector::dot(r.direction(), r.direction());
//     let a = r.direction().length_squared();
//     // let half_b = vector::dot(&oc, r.direction());
//     let h = vector::dot(r.direction(), &oc);
//     // let b = 2.0 * vector::dot(&oc, r.direction());
//     // let c = vector::dot(&oc, &oc) - radius.powi(2);
//     let c = oc.length_squared() - radius.powi(2);
// 
//     // let discriminant = b.powi(2) - 4.0 * a * c;
//     // let discriminant = half_b.powi(2) - a * c;
//     let discriminant = h.powi(2) - a*c;
// 
//     if discriminant < 0.0 {
//         return -1.0;
//     } else {
//         // return (-b - discriminant.sqrt()) / (2.0 * a);
//         // return (-half_b - discriminant.sqrt()) / a;
//         return (h - discriminant.sqrt()) / a;
//     }
// }

fn ray_color(r: Ray, world: &dyn Hittable) -> Color {

    /*
    let center = Point3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&center, 0.5, &r);

    if t > 0.0 {
        let big_n = unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));

        return 0.5 * Color::new(big_n.x() + 1.0, big_n.y() + 1.0, big_n.z() + 1.0);
    }
    */

    let maybe_record = world.hit(&r, Interval::new(0.0, INFINITY));
    if maybe_record.is_some() {
        let record = maybe_record.unwrap();

        return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0))
    }


    let unit_direction = vector::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vector::Point3::new(0.0, 0.0, 0.0);
    let horizontal = vector::Point3::new(viewport_width, 0.0, 0.0);
    let vertical = vector::Point3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = &origin - &horizontal / 2.0 - &vertical / 2.0 - vector::Vec3::new(0.0, 0.0, focal_length);

    // let mut stdout = std::io::stdout();
    let mut file = File::create("test.ppm").expect("Could not create file.");

    let header = std::format!("P6\n{} {}\n255\n", image_width, image_height);
    // let header_bytes = header_str.as_bytes();
    // let header_chars = header_str.as_bytes().chars().map(|c| c as char).collect();

    // stdout.write_all(header_bytes);
    file.write_all(header.as_bytes()).expect("Could not write to file.");

    // stdout.flush();
    for height in (0..image_height).rev() {

        print!("\rScanlines remaining: {:04}", height);

        for width in 0..image_width {
            // let r = width as f32 / (image_width - 1) as f32;
            // let g = height as f32 / (image_height - 1) as f32;
            // let b = 0.25;

            // let ir = (255.999 * r) as u8;
            // let ig = (255.999 * g) as u8;
            // let ib = (255.999 * b) as u8;

            // let bytes = [ir, ig, ib];

            // file.write_all(&bytes).expect("Could not write to file.");
            // let color = vector::Color::new(width as f64 / (image_width - 1) as f64, height as f64 / (image_height - 1) as f64, 0.25);
            let u = width as f64 / (image_width - 1) as f64;
            let v = height as f64 / (image_height - 1) as f64;

            let new_direction = &lower_left_corner + u * &horizontal + v * &vertical - &origin;
            let r = ray::Ray::new(&origin, &new_direction);

            let color = ray_color(r, &world);

            color.write(&mut file);
        }
    }

}
