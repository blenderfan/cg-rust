

use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use core::simd::prelude::*;


pub struct Vec2i {

    data : i32x2

}

impl Add for Vec2i {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}

impl Sub for Vec2i {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}

impl fmt::Display for Vec2i {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.data.extract::<0, 1>().as_array()[0], self.data.extract::<1, 1>().as_array()[0])
    }
}

impl Vec2i {

    pub fn new(x : i32, y : i32) -> Self {
        Self { data: Simd::from_array([x, y]) }
    }

}





pub struct Vec2l {

    data: i64x2

}

impl Add for Vec2l {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}

impl Sub for Vec2l {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}

impl fmt::Display for Vec2l {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.data.extract::<0, 1>().as_array()[0], self.data.extract::<1, 1>().as_array()[0])
    }
}


impl Vec2l {

    pub fn new(x : i64, y : i64) -> Self {
        Self { data: Simd::from_array([x, y]) }
    }
}





pub struct Vec2f {

    data: f32x2

}

impl Add for Vec2f {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}

impl Sub for Vec2f {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}

impl fmt::Display for Vec2f {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.data.extract::<0, 1>().as_array()[0], self.data.extract::<1, 1>().as_array()[0])
    }
}

impl Vec2f {
    pub fn new(x : f32, y : f32) -> Self {
        Self { data: Simd::from_array([x, y]) }
    }
}





pub struct Vec2d {

    data: f64x2

}


impl Add for Vec2d {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}

impl Sub for Vec2d {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}

impl fmt::Display for Vec2d {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.data.extract::<0, 1>().as_array()[0], self.data.extract::<1, 1>().as_array()[0])
    }
}

impl Vec2d {
    pub fn new(x : f64, y : f64) -> Self {
        Self { data: Simd::from_array([x, y]) }
    }
}




pub struct Vec3i {

    data: i32x4

}

impl Vec3i {

}

pub struct Vec3l {

    data: i64x4

}

impl Vec3l {

}


pub struct Vec3f {

    data: f32x4

}

impl Vec3f {

}


pub struct Vec3d {

    data: f64x4

}

impl Vec3d {



}

//TODO: Vector4
//TODO: Template