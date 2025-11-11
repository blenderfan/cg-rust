
use std::fmt;
use num_traits::Num;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;

use core::simd::prelude::*;


pub trait Vector<T> : Sized 
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + AddAssign
    + SubAssign 
    + Copy
    where T : Num + PartialOrd<T> {

    fn dot(a : Self, b: Self) -> T;

}

pub trait Vec2<T> : Vector<T>
    where T : Num + PartialOrd<T> {
    fn wedge(a : Self, b :Self) -> T;
}

pub trait Vec3<T> : Vector<T> 
    where T : Num + PartialOrd<T> {
    fn wedge(a : Self, b :Self) -> Self;
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

impl Vector<i32> for Vec2i {

     fn dot(a : Self, b :Self) -> i32 {
         return a.x() * b.x() + a.y() * b.y(); 
     }


}
impl Vec2<i32> for Vec2i {

     fn wedge(a : Self, b :Self) -> i32 {
         return a.x() * b.y() - a.y() * b.x();
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

impl Vector<i64> for Vec2l {

    fn dot(a : Self, b : Self) -> i64 {
         return a.x() * b.x() + a.y() * b.y(); 
     }


}
impl Vec2<i64> for Vec2l {

     fn wedge(a : Self, b : Self) -> i64 {
         return a.x() * b.y() - a.y() * b.x();
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

impl Vector<f32> for Vec2f {

    fn dot(a : Self, b : Self) -> f32 {
         return a.x() * b.x() + a.y() * b.y(); 
     }


}
impl Vec2<f32> for Vec2f {

     fn wedge(a : Self, b : Self) -> f32 {
         return a.x() * b.y() - a.y() * b.x();
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

impl Vector<f64> for Vec2d {

    fn dot(a : Self, b : Self) -> f64 {
         return a.x() * b.x() + a.y() * b.y(); 
     }


}
impl Vec2<f64> for Vec2d {

     fn wedge(a : Self, b : Self) -> f64 {
         return a.x() * b.y() - a.y() * b.x();
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

impl Vector<i32> for Vec3i {

    fn dot(a : Self, b : Self) -> i32 {
         return a.x() * b.x() + a.y() * b.y() + a.z() * b.z(); 
    }
}

impl Vec3<i32> for Vec3i {

     fn wedge(a : Self, b : Self) -> Self {
        return Vec3i::new(a.y() * b.z() - a.z() * b.y(),
            a.x() * b.z() - a.z() * b.y(),
            a.x() * b.y() - a.y() * b.x());
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

impl Vector<i64> for Vec3l {

    fn dot(a : Self, b : Self) -> i64 {
         return a.x() * b.x() + a.y() * b.y() + a.z() * b.z(); 
    }
}

impl Vec3<i64> for Vec3l {

     fn wedge(a : Self, b : Self) -> Self {
        return Vec3l::new(a.y() * b.z() - a.z() * b.y(),
            a.x() * b.z() - a.z() * b.y(),
            a.x() * b.y() - a.y() * b.x());
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

impl Vector<f32> for Vec3f {

    fn dot(a : Self, b : Self) -> f32 {
         return a.x() * b.x() + a.y() * b.y() + a.z() * b.z(); 
    }
}

impl Vec3<f32> for Vec3f {


     fn wedge(a : Self, b : Self) -> Self {
        return Vec3f::new(a.y() * b.z() - a.z() * b.y(),
            a.x() * b.z() - a.z() * b.y(),
            a.x() * b.y() - a.y() * b.x());
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


impl Vector<f64> for Vec3d {

    fn dot(a : Self, b : Self) -> f64 {
         return a.x() * b.x() + a.y() * b.y() + a.z() * b.z(); 
    }
}

impl Vec3<f64> for Vec3d {

     fn wedge(a : Self, b : Self) -> Self {
        return Vec3d::new(a.y() * b.z() - a.z() * b.y(),
            a.x() * b.z() - a.z() * b.y(),
            a.x() * b.y() - a.y() * b.x());
     }

}


//TODO: Vector4
//TODO: Template