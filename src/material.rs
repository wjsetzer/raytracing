use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::{dot, random_unit_vector, reflect, unit_vector, Color};

// pub enum Materials {
//     Lambertian(Lambertian),
//     Metal(Metal),
// }

pub trait Material {
    fn scatter(&self, r_in: Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(c: Color) -> Self{
        Self {albedo: c}
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = record.normal + random_unit_vector();

        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        let scattered = Ray::new(record.p, scatter_direction);
        
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(c: Color, fuzz: f64) -> Self{
        Self {albedo: c, fuzz}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = reflect(r_in.direction(), record.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        let scattered = Ray::new(record.p, reflected);

        if dot(&scattered.direction(), &record.normal) > 0.0 {
            return Some((self.albedo, scattered));
        }

        return None;
    }
}