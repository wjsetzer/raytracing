use std::fs::File;
use std::io::{self, Write};

use crate::common::{degrees_to_radians, random_f64, INFINITY};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::{self, cross, random_in_unit_disk, unit_vector, Color, Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,

    samples_per_pixel: i32,
    max_depth: i32,
    vfov: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    image_height: i32,
    pixel_samples_scale: f64, // color scale factor for a  sum of pixel samples
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,

            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            image_height: 0,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn render(&mut self, world: impl Hittable) {
        self.initialize();

        // let mut stdout = std::io::stdout();
        let mut file = File::create("test.ppm").expect("Could not create file.");

        let header = std::format!("P6\n{} {}\n255\n", self.image_width, self.image_height);

        // stdout.write_all(header_bytes);
        file.write_all(header.as_bytes())
            .expect("Could not write to file.");

        // stdout.flush();
        for height in 0..self.image_height {
            print!("\rScanlines remaining: {:04}", self.image_height - height);
            io::stdout().flush();

            for width in 0..self.image_width {
                // let color = Self::ray_color(r, world);
                // let pixel_center = self.pixel00_loc + (width as f64 * self.pixel_delta_u) + (height as f64 * self.pixel_delta_v);
                // let ray_direction = pixel_center - self.center;

                // let r = Ray::new(self.center, ray_direction);

                // let color = Self::ray_color(r, &world);

                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(width, height);
                    color = color + Self::ray_color(r, &world, self.max_depth);
                }

                color = self.pixel_samples_scale * color;
                color.write(&mut file);
            }
        }
    }

    fn initialize(&mut self) {
        let image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if image_height < 1 { 1 } else { image_height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        // self.center = Point3::new(0.0, 0.0, 0.0);
        self.center = self.lookfrom;

        // Determine viewport dimensions
        // let focal_length = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        // let viewport_height = 2.0 * h * focal_length;
        // let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // calculate the u,v, w, unit basis vectors for the camera coordinate frame
        let w = unit_vector(self.lookfrom - self.lookat);
        let u = unit_vector(cross(self.vup, w));
        let v = cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        // let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        // let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // calculate the location of the upper left pixel
        // let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        // let viewport_upper_left = self.center - (focal_length * w) - viewport_u/2.0 - viewport_v/2.0;
        let viewport_upper_left = self.center - (self.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // calculate the camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius; 
    }

    fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
        // if we've hit the max_depth, no more light is gathered
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let maybe_record = world.hit(&r, Interval::new(0.001, INFINITY));
        if maybe_record.is_some() {
            let record = maybe_record.unwrap();

            // return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0))
            // let direction = random_on_hemisphere(record.normal);
            // let direction = record.normal + random_unit_vector();
            // return 0.5 * Self::ray_color(Ray::new(record.p, direction), world, depth - 1);
            // return 0.5 * Self::ray_color(Ray::new(record.p, direction), world, depth-1);

            if record.mat.is_some() {
                let maybe_scattered = record.mat.unwrap().scatter(r, &record);
                if maybe_scattered.is_some() {
                    let (attenuation, scattered) = maybe_scattered.unwrap();

                    return attenuation * Self::ray_color(scattered, world, depth - 1);
                }

                return Color::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = vector::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // construct a camera ray originating from the defocus disk and directed at randomly sampled
        // points around the pixel location i, j

        let offset = Self::sample_square();
        let pixel_sample: Vec3 = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        // let ray_origin = self.center;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        // returns the vector to  a random point in the [-.5, -.5] - [.5, .5] unit square
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // returns a random point in the camera defeocus disk
        let p = random_in_unit_disk();
        return self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v);
    }
}
