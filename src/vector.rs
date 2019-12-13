use std::convert::From;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
    pub fn reflect(&self, normal: Self) -> Self {
        *self - normal * 2. * self.dot(normal)
    }
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<[f64; 2]> for Vector {
    fn from(item: [f64; 2]) -> Self {
        Self {
            x: item[0],
            y: item[1],
        }
    }
}
