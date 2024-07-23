use crate::hittable::{Hittable, HitRecord};
use crate::vector::{self, Point3};
use crate::ray::Ray;
use crate::interval::Interval;

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self{center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();

        let a = r.direction().length_squared();
        let h = vector::dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd)  / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;

            if !ray_t.surrounds(root){
                return None;
            }
        }

        let mut rec = HitRecord::new();
        rec.t = root;
        rec.p = r.at(rec.t);
        // rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        return Some(rec);
    }
}