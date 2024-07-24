use crate::material::Material;
use crate::vector::{self, Color, Point3, Vec3};
use crate::ray::Ray;
use crate::interval::Interval;

// #[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<&'a dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    /*
    pub fn new(p: Point3, normal: Point3, t: f64) -> Self {
        Self { p, normal, t, front_face: false }
    }
    */
    pub fn new() -> Self {
        
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat: None, 
            t: 0.0,
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length

        self.front_face = vector::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -outward_normal }
    }

    // pub fn update_mat(&mut self, mat: &'a dyn Material) {
    //     self.mat = Box::new(mat);
    // }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}