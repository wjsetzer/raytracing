use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::{dot, random_unit_vector, reflect, refract, unit_vector, Color};

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

pub struct Dielectric {
    // refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index ofthe enclosing medium
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(r: f64) -> Self{
        Self { refraction_index: r }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if record.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = unit_vector(r_in.direction());
        let refracted = refract(unit_direction, record.normal, ri);

        let scattered = Ray::new(record.p, refracted);
        
        return Some((attenuation, scattered));
    }
}