//! # Polygons
//! 
//! Polygons consist of a list of points in the plane, which are sequentially connected by
//! line segments, forming a bounded area in this way. A choice can be made to list the vertices
//! clockwise (CW) or counter-clockwise (CCW). All polygons in this package are assumed to be
//! CCW, while holes in a polygon are assumed to be CW.

use core::f64;
use std::marker::PhantomData;

use num_traits::Num;
use num_traits::Float;

use crate::vector::Vec2;
use crate::vector::Vec2d;

pub trait PolygonFloat<T : Vec2<U>, U : Float> {
    
    /// Generates a regular polygon where each line segment has the same length and the
    /// angle between all edges are equal as well.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let poly = Polygon::Vec2d::regular(Vec2d::new(0.5, 0.5), 0.5, 6);
    /// ```
    fn regular(center : T, radius : U, corners : usize) -> Polygon<T, U>;
}



pub struct Polygon<U: Vec2<T>, T : Num + PartialOrd<T>> {

    points : Vec<U>,
    element_type : PhantomData<T>
}

impl<U : Vec2<T>, T : Num + PartialOrd<T>> Polygon<U, T> {

    pub fn new() -> Self {
        Self { points: Vec::<U>::new(), element_type: PhantomData }
    }

    pub fn with_capacity(capacity : usize) -> Self {
        Self { points: Vec::<U>::with_capacity(capacity), element_type: PhantomData }
    }

    pub fn get_points(&self) -> &Vec<U> {
        return &self.points;
    }

    /// Adds one vertex to the end of the list of vertices of the polygon.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut poly = Polygon::Vec2f::new();
    /// poly.push(Vec2f::new(0.0, 0.0));
    /// poly.push(Vec2f::new(1.0, 0.0));
    /// poly.push(Vec2f::new(1.0, 1.0));
    /// poly.push(Vec2f::new(0.0, 1.0));
    /// ```
    pub fn push(&mut self, point : U) {
        self.points.push(point);
    }

    pub fn is_convex(&self) -> bool {

        if self.points.len() < 3 { 
            return false; 
        }

        let size = self.points.len();
        let mut last_point = self.points[0];
        let mut edge = last_point - self.points[size - 1];

        for i in 0..self.points.len() {

            let next_idx = (i + 1) % size;
            let next_p = self.points[next_idx];

            if U::wedge(edge, next_p) < T::zero() {
                return false;
            }

            edge = next_p - last_point;
            last_point = next_p;
        }

        return true;
    }    

}

impl<> PolygonFloat<Vec2d, f64> for Polygon<Vec2d, f64> {


    fn regular(center : Vec2d, radius : f64, corners : usize) -> Polygon<Vec2d, f64> {
        
        let angle_per_corner = f64::consts::TAU / (corners as f64);

        let mut poly = Polygon::<Vec2d, f64>::with_capacity(corners);

        for i in 0..corners {

            let current_angle : f64 = angle_per_corner * i as f64;

            let x_y = current_angle.sin_cos();
            let xy_vec = Vec2d::new(x_y.1, x_y.0) * 0.5_f64 * radius + center;

            poly.push(xy_vec);
        }

        return poly;
    }
}
