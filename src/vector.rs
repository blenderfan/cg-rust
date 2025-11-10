
use std::fmt;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;

use core::simd::prelude::*;

pub trait Vector {

    type Element;

    fn dot(self, other: Self) -> Self::Element;

}

pub trait Vec2 : Vector {
    fn wedge(self, other:Self) -> Self::Element;
}

pub trait Vec3 : Vector {
    fn wedge(self, other:Self) -> Self;
}

#[derive(Clone, Copy)]
pub struct Vec2i {

    data : i32x2

}

impl Add for Vec2i {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}

impl AddAssign for Vec2i {

    fn add_assign(&mut self, other: Self) {
        self.data = self.data + other.data;
    }
}

impl Sub for Vec2i {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}

impl SubAssign for Vec2i {

    fn sub_assign(&mut self, other: Self) {
        self.data = self.data - other.data;
    }
}

impl Mul for Vec2i {

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { data: self.data * other.data }
    }
}

impl Mul<i32> for Vec2i {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self { data: self.data * i32x2::splat(other) }
    }
}

impl MulAssign for Vec2i {

    fn mul_assign(&mut self, other: Self) {
        self.data = self.data * other.data;
    }
}



impl fmt::Display for Vec2i {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(X: {} | Y: {})", self.data.as_array()[0], self.data.as_array()[0])
    }
}

impl Vec2i {

    pub fn new(x : i32, y : i32) -> Self {
        Self { data: Simd::from_array([x, y]) }
    }

    pub fn x(&self) -> i32 { return self.data.as_array()[0]; }
    pub fn y(&self) -> i32 { return self.data.as_array()[1]; }
}

impl Vector for Vec2i {
     type Element = i32;

     fn dot(self, other: Self) -> Self::Element {
         return self.x() * other.x() + self.y() * other.y(); 
     }


}
impl Vec2 for Vec2i {

     fn wedge(self, other:Self) -> Self::Element {
         return self.x() * other.y() - self.y() * other.x();
     }
}



#[derive(Clone, Copy)]
pub struct Vec2l {

    data: i64x2

}

impl Add for Vec2l {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}

impl AddAssign for Vec2l {

    fn add_assign(&mut self, other: Self) {
        self.data = self.data + other.data;
    }
}

impl Sub for Vec2l {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}

impl SubAssign for Vec2l {

    fn sub_assign(&mut self, other: Self) {
        self.data = self.data - other.data;
    }
}

impl Mul for Vec2l {

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { data: self.data * other.data }
    }
}

impl Mul<i64> for Vec2l {
    type Output = Self;

    fn mul(self, other: i64) -> Self {
        Self { data: self.data * i64x2::splat(other) }
    }
}

impl MulAssign for Vec2l {

    fn mul_assign(&mut self, other: Self) {
        self.data = self.data * other.data;
    }
}

impl fmt::Display for Vec2l {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(X: {} | Y: {})", self.data.as_array()[0], self.data.as_array()[0])
    }
}


impl Vec2l {

    pub fn new(x : i64, y : i64) -> Self {
        Self { data: Simd::from_array([x, y]) }
    }

    pub fn x(&self) -> i64 { return self.data.as_array()[0]; }
    pub fn y(&self) -> i64 { return self.data.as_array()[1]; }
}

impl Vector for Vec2l {
     type Element = i64;

    fn dot(self, other: Self) -> Self::Element {
         return self.x() * other.x() + self.y() * other.y(); 
     }


}
impl Vec2 for Vec2l {

     fn wedge(self, other:Self) -> Self::Element {
         return self.x() * other.y() - self.y() * other.x();
     }

}



#[derive(Clone, Copy)]
pub struct Vec2f {

    data: f32x2

}

impl Add for Vec2f {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}


impl AddAssign for Vec2f {

    fn add_assign(&mut self, other: Self) {
        self.data = self.data + other.data;
    }
}


impl Sub for Vec2f {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}


impl SubAssign for Vec2f {

    fn sub_assign(&mut self, other: Self) {
        self.data = self.data - other.data;
    }
}

impl Mul for Vec2f {

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { data: self.data * other.data }
    }
}

impl Mul<f32> for Vec2f {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self { data: self.data * f32x2::splat(other) }
    }
}

impl MulAssign for Vec2f {

    fn mul_assign(&mut self, other: Self) {
        self.data = self.data * other.data;
    }
}

impl fmt::Display for Vec2f {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(X: {} | Y: {})", self.data.as_array()[0], self.data.as_array()[0])
    }
}

impl Vec2f {
    pub fn new(x : f32, y : f32) -> Self {
        Self { data: Simd::from_array([x, y]) }
    }

    pub fn x(&self) -> f32 { return self.data.as_array()[0]; }
    pub fn y(&self) -> f32 { return self.data.as_array()[1]; }
}

impl Vector for Vec2f {
     type Element = f32;

    fn dot(self, other: Self) -> Self::Element {
         return self.x() * other.x() + self.y() * other.y(); 
     }


}
impl Vec2 for Vec2f {

     fn wedge(self, other:Self) -> Self::Element {
         return self.x() * other.y() - self.y() * other.x();
     }

}


#[derive(Clone, Copy)]
pub struct Vec2d {

    data: f64x2

}


impl Add for Vec2d {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}

impl AddAssign for Vec2d {

    fn add_assign(&mut self, other: Self) {
        self.data = self.data + other.data;
    }
}

impl Sub for Vec2d {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}

impl SubAssign for Vec2d {

    fn sub_assign(&mut self, other: Self) {
        self.data = self.data - other.data;
    }
}

impl Mul for Vec2d {

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { data: self.data * other.data }
    }
}


impl Mul<f64> for Vec2d {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self { data: self.data * f64x2::splat(other) }
    }
}

impl MulAssign for Vec2d {

    fn mul_assign(&mut self, other: Self) {
        self.data = self.data * other.data;
    }
}

impl fmt::Display for Vec2d {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(X: {} | Y: {})", self.data.as_array()[0], self.data.as_array()[1])
    }
}

impl Vec2d {
    pub fn new(x : f64, y : f64) -> Self {
        Self { data: Simd::from_array([x, y]) }
    }

    pub fn x(&self) -> f64 { return self.data.as_array()[0]; }
    pub fn y(&self) -> f64 { return self.data.as_array()[1]; }
}

impl Vector for Vec2d {
     type Element = f64;

    fn dot(self, other: Self) -> Self::Element {
         return self.x() * other.x() + self.y() * other.y(); 
     }


}
impl Vec2 for Vec2d {

     fn wedge(self, other:Self) -> Self::Element {
         return self.x() * other.y() - self.y() * other.x();
     }

}



#[derive(Clone, Copy)]
pub struct Vec3i {

    data: i32x4

}


impl Add for Vec3i {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}

impl AddAssign for Vec3i {

    fn add_assign(&mut self, other: Self) {
        self.data = self.data + other.data;
    }
}

impl Sub for Vec3i {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}

impl SubAssign for Vec3i {

    fn sub_assign(&mut self, other: Self) {
        self.data = self.data - other.data;
    }
}

impl Mul for Vec3i {

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { data: self.data * other.data }
    }
}

impl Mul<i32> for Vec3i {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self { data: self.data * i32x4::splat(other) }
    }
}

impl MulAssign for Vec3i {

    fn mul_assign(&mut self, other: Self) {
        self.data = self.data * other.data;
    }
}

impl fmt::Display for Vec3i {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.data.as_array();
        write!(f, "(X: {} | Y: {} | Z: {})", arr[0], arr[1], arr[2])
    }
}

impl Vec3i {
    pub fn new(x : i32, y : i32, z: i32) -> Self {
        Self { data: Simd::from_array([x, y, z, 0]) }
    }

    pub fn x(&self) -> i32 { return self.data.as_array()[0]; }
    pub fn y(&self) -> i32 { return self.data.as_array()[1]; }
    pub fn z(&self) -> i32 { return self.data.as_array()[2]; }
}

impl Vector for Vec3i {
     type Element = i32;

    fn dot(self, other: Self) -> Self::Element {
         return self.x() * other.x() 
            + self.y() * other.y()
            + self.z() * other.z(); 
     }


}
impl Vec3 for Vec3i {

     fn wedge(self, other:Self) -> Self {
        return Vec3i::new(self.y() * other.z() - self.z() * other.y(),
            self.x() * other.z() - self.z() * other.y(),
            self.x() * other.y() - self.y() * other.x());
     }

}



#[derive(Clone, Copy)]
pub struct Vec3l {

    data: i64x4

}

impl Add for Vec3l {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}

impl AddAssign for Vec3l {

    fn add_assign(&mut self, other: Self) {
        self.data = self.data + other.data;
    }
}

impl Sub for Vec3l {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}

impl SubAssign for Vec3l {

    fn sub_assign(&mut self, other: Self) {
        self.data = self.data - other.data;
    }
}


impl Mul for Vec3l {

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { data: self.data * other.data }
    }
}

impl Mul<i64> for Vec3l {
    type Output = Self;

    fn mul(self, other: i64) -> Self {
        Self { data: self.data * i64x4::splat(other) }
    }
}

impl MulAssign for Vec3l {

    fn mul_assign(&mut self, other: Self) {
        self.data = self.data * other.data;
    }
}


impl fmt::Display for Vec3l {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.data.as_array();
        write!(f, "(X: {} | Y: {} | Z: {})", arr[0], arr[1], arr[2])
    }
}

impl Vec3l {
    pub fn new(x : i64, y : i64, z: i64) -> Self {
        Self { data: Simd::from_array([x, y, z, 0]) }
    }

    pub fn x(&self) -> i64 { return self.data.as_array()[0]; }
    pub fn y(&self) -> i64 { return self.data.as_array()[1]; }
    pub fn z(&self) -> i64 { return self.data.as_array()[2]; }
}

impl Vector for Vec3l {
     type Element = i64;

    fn dot(self, other: Self) -> Self::Element {
         return self.x() * other.x() 
            + self.y() * other.y()
            + self.z() * other.z(); 
     }
}
impl Vec3 for Vec3l {

     fn wedge(self, other:Self) -> Self {
        return Vec3l::new(self.y() * other.z() - self.z() * other.y(),
            self.x() * other.z() - self.z() * other.y(),
            self.x() * other.y() - self.y() * other.x());
     }

}




#[derive(Clone, Copy)]
pub struct Vec3f {

    data: f32x4

}

impl Add for Vec3f {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}

impl AddAssign for Vec3f {

    fn add_assign(&mut self, other: Self) {
        self.data = self.data + other.data;
    }
}

impl Sub for Vec3f {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}

impl SubAssign for Vec3f {

    fn sub_assign(&mut self, other: Self) {
        self.data = self.data - other.data;
    }
}

impl Mul for Vec3f {

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { data: self.data * other.data }
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self { data: self.data * f32x4::splat(other) }
    }
}

impl MulAssign for Vec3f {

    fn mul_assign(&mut self, other: Self) {
        self.data = self.data * other.data;
    }
}


impl fmt::Display for Vec3f {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.data.as_array();
        write!(f, "(X: {} | Y: {} | Z: {})", arr[0], arr[1], arr[2])
    }
}

impl Vec3f {
    pub fn new(x : f32, y : f32, z: f32) -> Self {
        Self { data: Simd::from_array([x, y, z, 0.0_f32]) }
    }

    pub fn x(&self) -> f32 { return self.data.as_array()[0]; }
    pub fn y(&self) -> f32 { return self.data.as_array()[1]; }
    pub fn z(&self) -> f32 { return self.data.as_array()[2]; }
}

impl Vector for Vec3f {
     type Element = f32;

     
    fn dot(self, other: Self) -> Self::Element {
         return self.x() * other.x() 
            + self.y() * other.y()
            + self.z() * other.z(); 
     }
}
impl Vec3 for Vec3f {

     fn wedge(self, other:Self) -> Self {
        return Vec3f::new(self.y() * other.z() - self.z() * other.y(),
            self.x() * other.z() - self.z() * other.y(),
            self.x() * other.y() - self.y() * other.x());
     }


}



#[derive(Clone, Copy)]
pub struct Vec3d {

    data: f64x4

}

impl Add for Vec3d {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { data: self.data + other.data }
    }
}

impl AddAssign for Vec3d {

    fn add_assign(&mut self, other: Self) {
        self.data = self.data + other.data;
    }
}

impl Sub for Vec3d {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { data:self.data - other.data}
    }
}

impl SubAssign for Vec3d {

    fn sub_assign(&mut self, other: Self) {
        self.data = self.data - other.data;
    }
}


impl Mul for Vec3d {

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { data: self.data * other.data }
    }
}

impl Mul<f64> for Vec3d {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self { data: self.data * f64x4::splat(other) }
    }
}

impl MulAssign for Vec3d {

    fn mul_assign(&mut self, other: Self) {
        self.data = self.data * other.data;
    }
}

impl fmt::Display for Vec3d {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.data.as_array();
        write!(f, "(X: {} | Y: {} | Z: {})", arr[0], arr[1], arr[2])
    }
}

impl Vec3d {
    pub fn new(x : f64, y : f64, z: f64) -> Self {
        Self { data: Simd::from_array([x, y, z, 0.0_f64]) }
    }

    pub fn x(&self) -> f64 { return self.data.as_array()[0]; }
    pub fn y(&self) -> f64 { return self.data.as_array()[1]; }
    pub fn z(&self) -> f64 { return self.data.as_array()[2]; }
}


impl Vector for Vec3d {
     type Element = f64;

    fn dot(self, other: Self) -> Self::Element {
         return self.x() * other.x() 
            + self.y() * other.y()
            + self.z() * other.z(); 
     }
}
impl Vec3 for Vec3d {

     fn wedge(self, other:Self) -> Self {
        return Vec3d::new(self.y() * other.z() - self.z() * other.y(),
            self.x() * other.z() - self.z() * other.y(),
            self.x() * other.y() - self.y() * other.x());
     }

}


//TODO: Vector4
//TODO: Template