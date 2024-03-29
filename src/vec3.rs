// vec3.rs
//
// Copyright (c) 2019, Univerisity of Minnesota
//
// Author: Bridger Herman (herma582@umn.edu)

//! A 3 dimensional vector (mimicking GLM's vec3)

use std::f32;
use std::ops::{Add, Mul, Neg, Sub};

use num_traits::Zero;

pub const MAX_VECTOR3: Vec3 = Vec3 {
    x: f32::MAX,
    y: f32::MAX,
    z: f32::MAX,
};

/// 3 dimensional vector
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Zero for Vec3 {
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }
}

impl<'a> From<&'a [f32]> for Vec3 {
    fn from(slice: &'a [f32]) -> Self {
        assert_eq!(slice.len(), 3);
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Self {
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
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f32 {
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
    pub fn refract(self, normal: Self, eta: f32) -> Self {
        // Figure out if we're going into or coming out of the material
        let dot_ni = self.dot(&normal);
        let eta = if dot_ni > 0.0 { eta } else { 1.0 / eta };

        let k = 1.0 - eta * eta * (1.0 - dot_ni * dot_ni);
        if k < 0.0 {
            self * eta
        } else {
            self * eta - normal * (eta * dot_ni + k.sqrt())
        }
        .normalized()
    }

    pub fn angle(&self, other: &Self) -> f32 {
        self.normalized().dot(&other.normalized()).acos()
    }
}
