use std::vec::Vec;

use crate::{hittable::{HitRecord, Hittable}, ray::Ray};
use crate::interval::Interval;
use crate::material::Lambertian;
use crate::vector::Color;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new()}
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
        // self.objects.insert(0, Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // let mut temp_rec: HitRecord = HitRecord::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0);
        let default_material = Lambertian::new(Color::new(0.0, 0.0, 0.0));
        let mut record: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;



        for object in self.objects.iter() {
            let maybe_record = object.hit(r, Interval::new(ray_t.min, closest_so_far));
            // match temp_record {
            //     Some(p) => {
            //         hit_anything = true;
            //         closest_so_far = temp_record.t;
            //         record = temp_record;
            //     }
            // }
            if maybe_record.is_some() {
                let temp_record = maybe_record.unwrap();
                hit_anything = true;
                // closest_so_far = temp_rec.t;
                // *rec = temp_rec;
                closest_so_far = temp_record.t;
                record = temp_record;
            }
        }

        if hit_anything {
            return Some(record);
        }

        return None;
    }
}