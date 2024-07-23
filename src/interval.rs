use crate::common::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) ->Self {
        Self {min, max}
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        }
        
        return x;
    }
}

pub const EMPTY: Interval = Interval { min: INFINITY, max: -INFINITY };
pub const UNIVERSE: Interval = Interval { min: -INFINITY, max: INFINITY };
