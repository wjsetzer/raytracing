use crate::vector::{Point3, Vec3};

pub struct Ray<'a> {
    origin: &'a Point3,
    direction: &'a Vec3
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point3, direction: &'a Vec3) -> Self {
        Self {origin, direction}
    }

    pub fn direction(&self) -> &'a Vec3 {
        self.direction
    }

    pub fn origin(&self) -> &'a Point3 {
        self.origin
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}