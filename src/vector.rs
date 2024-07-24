use std::ops::{Add, Div, Mul, Neg, Sub};
use std::fs::File;
use std::io::Write;

use crate::common::{random_f64, random_range_f64};
use crate::interval::Interval;


#[derive(Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {e: [x, y, z]}
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn random() -> Self  {
        Vec3::new(random_f64(), random_f64(), random_f64())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(random_range_f64(min, max), random_range_f64(min, max), random_range_f64(min, max))
    }

    pub fn write(&self, f: &mut File) -> () {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        let intensity = Interval::new(0.0, 0.999);
        let ir = (256.0 * intensity.clamp(r)) as u8;
        let ig = (256.0 * intensity.clamp(g)) as u8;
        let ib = (256.0 * intensity.clamp(b)) as u8;

        // let bytes = [ir, ig, ib];

        f.write_all(&[ir, ig, ib]).expect("Could not write to file.");
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {e: [-self.e[0], -self.e[1], -self.e[2]]}
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {e: [-self.e[0], -self.e[1], -self.e[2]]}
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]}
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self {e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]}
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]}
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]]}
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self {
        Self {e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]]}
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        Vec3 {e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]]}
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]]}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self {e: [self.e[0] * t, self.e[1] * t, self.e[2] * t]}
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {e: [self.e[0] * t, self.e[1] * t, self.e[2] * t]}
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Vec3 {
        v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        (1.0 / t) * self
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        (1.0 / t) * self
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }

    // this will never return. just for the compiler
    return Vec3::new(0.0, 0.0, 0.0)
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, &normal)  > 0.0 {
        return on_unit_sphere;
    } 

    return -on_unit_sphere;
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    return 0.0;
}

pub type Point3 = Vec3;
pub type Color = Vec3;
