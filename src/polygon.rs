//! # Polygons
//! 
//! Polygons consist of a list of points in the plane, which are sequentially connected by
//! line segments (also called edges), forming a bounded area in this way. A choice can be made to list the vertices
//! clockwise (CW) or counter-clockwise (CCW). All polygons in this package are assumed to be
//! CCW, while holes in a polygon are assumed to be CW.
//! 
//! A polygon is convex if for each two consecutive edges in the polygon, the angle is not obtuse (strictly larger than 180°)

use core::f64;
use std::marker::PhantomData;

use num_traits::Num;
use num_traits::Float;

use crate::vector::Vec2;
use crate::vector::Vec2d;

pub trait PolygonFloat<U : Float, T : Vec2<U>> {
    
    /// Generates a regular polygon where each line segment has the same length and the
    /// angle between all edges are equal as well.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let poly = Polygon::<f64, Vec2d>::regular(Vec2d::new(0.5, 0.5), 0.5, 6);
    /// ```
    fn regular(center : T, radius : U, corners : usize) -> Polygon<U, T>;
}



pub struct Polygon<T : Num + PartialOrd<T>, U: Vec2<T>, > {

    points : Vec<U>,
    element_type : PhantomData<T>
}

impl< T : Num + PartialOrd<T>, U : Vec2<T>> Polygon<T, U> {

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
    /// let mut poly = Polygon::<f64, Vec2d>::new();
    /// poly.push(Vec2d::new(0.0, 0.0));
    /// poly.push(Vec2d::new(1.0, 0.0));
    /// poly.push(Vec2d::new(1.0, 1.0));
    /// poly.push(Vec2d::new(0.0, 1.0));
    /// ```
    pub fn push(&mut self, point : U) {
        self.points.push(point);
    }

    /// Adds a list of vertices to the end of the list of vertices of the polygon.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut poly = Polygon::<f64, Vec2d>::new();
    /// poly.push_vector(Vec::from([Vec2d::new(0.0, 0.0),
    ///                             Vec2d::new(1.0, 0.0),
    ///                             Vec2d::new(1.0, 1.0),
    ///                             Vec2d::new(0.0, 1.0)]));
    /// ```
    pub fn push_vector(&mut self, points : Vec<U>) {
        for point in points.iter() {
            self.push(*point);
        }
    }

    /// Checks if the polygon is convex. If it is, true is returned,
    /// if it is concave false is returned. If the polygon is not
    /// simple / self-intersecting, Option::None is returned.
    /// 
    /// # Examples
    /// ```
    /// let mut poly = Polygon::<f64, Vec2d>::new();
    /// poly.push_vector(Vec::from([Vec2d::new(0.0, 0.0),
    ///                             Vec2d::new(1.0, 0.0),
    ///                             Vec2d::new(1.0, 1.0),
    ///                             Vec2d::new(0.0, 1.0)]));
    /// let convex = poly.is_convex(); //Returns true, since polygon is a rectangle
    /// ```
    pub fn is_convex(&self) -> Option<bool> {

        if self.points.len() < 3 {
            return Option::None;
        }
        //TODO polygon is simple check

        let size = self.points.len();
        let mut last_point = self.points[0];
        let mut edge = last_point - self.points[size - 1];

        for i in 0..self.points.len() {

            let next_idx = (i + 1) % size;
            let next_p = self.points[next_idx];

            let next_edge = next_p - last_point;
            if U::wedge(edge, next_edge) < T::zero() {
                return Some(false);
            }

            edge = next_edge;
            last_point = next_p;
        }

        return Some(true);
    }

      

}

impl<> PolygonFloat<f64, Vec2d> for Polygon<f64, Vec2d> {


    fn regular(center : Vec2d, radius : f64, corners : usize) -> Polygon<f64, Vec2d> {
        
        let angle_per_corner = f64::consts::TAU / (corners as f64);

        let mut poly = Polygon::<f64, Vec2d>::with_capacity(corners);

        for i in 0..corners {

            let current_angle : f64 = angle_per_corner * i as f64;

            let x_y = current_angle.sin_cos();
            let xy_vec = Vec2d::new(x_y.1, x_y.0) * 0.5_f64 * radius + center;

            poly.push(xy_vec);
        }

        return poly;
    }
}


#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn test_is_triangle_convex() {

        let mut poly = Polygon::<f64, Vec2d>::new();
        poly.push_vector(Vec::from([Vec2d::new(0.0, 0.0),
                                Vec2d::new(1.0, 0.0),
                                 Vec2d::new(0.5, 0.66)]));

        let convex = poly.is_convex();
        assert_eq!(convex.unwrap(), true);
    }

    #[test]
    fn test_is_rect_convex() {

        let mut poly = Polygon::<f64, Vec2d>::new();
        poly.push_vector(Vec::from([Vec2d::new(0.0, 0.0),
                                Vec2d::new(1.0, 0.0),
                                 Vec2d::new(1.0, 1.0),
                                 Vec2d::new(0.0, 1.0)]));

        let convex = poly.is_convex();
        assert_eq!(convex.unwrap(), true);
    }

    #[test]
    fn test_is_concave() {

        let mut poly = Polygon::<f64, Vec2d>::new();
        poly.push_vector(Vec::from([Vec2d::new(0.0, 0.0),
                                Vec2d::new(1.0, 0.0),
                                 Vec2d::new(0.1, 0.1),
                                 Vec2d::new(0.0, 1.0)]));

        let convex = poly.is_convex();
        assert_eq!(convex.unwrap(), false);
    }

    
    #[test]
    fn test_is_collinear_triangle_convex() {

        let mut poly = Polygon::<f64, Vec2d>::new();
        poly.push_vector(Vec::from([Vec2d::new(0.0, 0.0),
                                Vec2d::new(1.0, 0.0),
                                 Vec2d::new(0.5, 0.5),
                                 Vec2d::new(0.0, 1.0)]));

        let convex = poly.is_convex();
        assert_eq!(convex.unwrap(), true);
    }

}