// vec3.rs
//
// Copyright (c) 2019, Univerisity of Minnesota
//
// Author: Bridger Herman (herma582@umn.edu)

//! A 3 dimensional vector (mimicking GLM's vec3)

use std::f64;
use std::ops::{Add, Mul, Neg, Sub};

pub const MAX_VECTOR3: Vec3 = Vec3 {
    x: f64::MAX,
    y: f64::MAX,
    z: f64::MAX,
};

/// 3 dimensional vector
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl<'a> From<&'a [f64]> for Vec3 {
    fn from(slice: &'a [f64]) -> Self {
        assert_eq!(slice.len(), 3);
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn reflect(self, normal: &Self) -> Self {
        self - (*normal * self.dot(normal)) * 2.0
    }

    // Uses the same math from the GLM library - doesn't match GLM-rs
    pub fn refract(self, normal: Self, eta: f64) -> Self {
        // Figure out if we're going into or coming out of the material
        let dot_ni = self.dot(&normal);
        let eta = if dot_ni > 0.0 { eta } else { 1.0 / eta };

        let k = 1.0 - eta * eta * (1.0 - dot_ni * dot_ni);
        if k < 0.0 {
            self * eta
        } else {
            self * eta - normal * (eta * dot_ni + k.sqrt())
        }.normalized()
    }

    pub fn angle(&self, other: &Self) -> f64 {
        self.normalized().dot(&other.normalized()).acos()
    }
}

