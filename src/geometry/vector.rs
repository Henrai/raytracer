use std::ops::{Add, Sub, Mul, Neg}; 
use std::marker::Copy;

#[derive(Debug,Copy,Clone)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn fill(a: f64) -> Vec3D {
        Vec3D { x:a, y:a, z:a }
    }

    pub fn zero() -> Vec3D {
        Vec3D::fill(0.0)
    }

    pub fn norm(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z)
    }

    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn normalize(&self) -> Vec3D {
        let t = self.length().recip();
        Vec3D {
            x : self.x*t,
            y : self.y*t,
            z : self.z*t
        }
    }
}

impl Add for Vec3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3D {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self{
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Neg for Vec3D {
    type Output = Self;
    fn neg(self) -> Self {
        Self{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
