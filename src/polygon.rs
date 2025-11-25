//! # Polygons
//! 
//! Polygons consist of a list of points in the plane, which are sequentially connected by
//! line segments (also called edges), forming a bounded area in this way. A choice can be made to list the vertices
//! clockwise (CW) or counter-clockwise (CCW). All polygons in this package are assumed to be
//! CCW, while holes in a polygon are assumed to be CW.
//! 
//! A polygon is convex if for each two consecutive edges in the polygon, the angle is not obtuse (strictly larger than 180°)

use core::f32;
use core::f64;
use std::marker::PhantomData;

use num_traits::Num;
use num_traits::Float;
use num_traits::NumCast;
use num_traits::PrimInt;

use crate::property_map::PropertyType;
use crate::property_map::PropertyMap;
use crate::property_map::PropertyStore;

use crate::property_map::VertexProperties;
use crate::vector::Vec2;
use crate::vector::Vec2d;
use crate::vector::Vec2f;

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
    vertex_properties : PropertyStore,
    element_type : PhantomData<T>,
}

impl< T : Num + PartialOrd<T>, U : Vec2<T>> Polygon<T, U> {

    pub fn new() -> Self {
        Self { points: Vec::<U>::new(), element_type: PhantomData,
             vertex_properties: PropertyStore::new() }
    }

    pub fn with_capacity(capacity : usize) -> Self {
        Self { points: Vec::<U>::with_capacity(capacity), element_type: PhantomData,
            vertex_properties: PropertyStore::new() }
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

    /// Returns all indices of concave / reflex vertices of the polygon.
    /// Reflexive vertices build an interior angle strictly greater than 180°.
    /// 
    /// If the polygon is self-intersecting, Option::None is returned.
    /// 
    /// # Examples
    /// ```
    /// let mut poly = Polygon::<f64, Vec2d>::new();
    /// poly.push_vector(Vec::from([Vec2d::new(0.0, 0.0),
    ///                             Vec2d::new(1.0, 0.0),
    ///                             Vec2d::new(1.0, 1.0),
    ///                             Vec2d::new(0.0, 1.0),
    ///                             Vec2d::new(0.5, 0.5)]));
    /// let concave_vertices = poly.get_concave_vertices(); //Returns a HashSet containing the value 4
    /// ```
    pub fn get_concave_vertices(&self) -> Option<Vec<usize>> {

        let mut concave_indices : Vec<usize> = Vec::<usize>::new();

        if self.points.len() < 3 {
            return Option::None;
        }
        //TODO polygon is simple check

        let size = self.points.len();
        let mut last_point = self.points[0];
        let mut edge = last_point - self.points[size - 1];

        for i in 0..size {

            let next_idx = (i + 1) % size;
            let next_p = self.points[next_idx];

            let next_edge = next_p - last_point;
            if U::wedge(edge, next_edge) < T::zero() {
                concave_indices.push(i);
            }
            edge = next_edge;
            last_point = next_p;

        }

        return Some(concave_indices);
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

        for i in 0..size {

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

    fn triangulate_convex<IndexType>(&self, indices : &mut Vec<IndexType>, start : usize) where IndexType : PrimInt {

        //Fan Triangulation
        let size = self.points.len();

        let mut next_idx = (start + 1) % size;
        for i in 0..size-2 {

            let next_next_idx = (start + i + 2) % size;

            indices.push(NumCast::from(start).unwrap());
            indices.push(NumCast::from(next_idx).unwrap());
            indices.push(NumCast::from(next_next_idx).unwrap());

            next_idx = next_next_idx;
        }
    }

    /// Splits the polygon into triangles and returns a list of integers, where each triple
    /// forms a triangle as part of the triangulation of the polygon.
    /// 
    /// The method will return null if the polygon is self-intersecting
    /// 
    /// # Examples
    /// ```
    /// let poly = Polygon::<f64, Vec2d>::regular(Vec2d::new(0.0, 0.0), 1.0, 8);
    /// let triangulation = poly.triangulate::<i32>();
    /// ```
    pub fn triangulate<IndexType>(&self) -> Option<Vec<IndexType>> where IndexType : PrimInt {

        let mut indices : Vec<IndexType> = Vec::<IndexType>::new();

        let concave_vertices = self.get_concave_vertices();
        if concave_vertices.is_none() {
            return None;
        }

        let unwrapped_indices = concave_vertices.unwrap();
        let nr_of_concave_vertices = unwrapped_indices.len();
        if nr_of_concave_vertices == 0 {
            self.triangulate_convex(&mut indices, 0);
        } else if nr_of_concave_vertices == 1 {
            let concave_vertex = unwrapped_indices[0];
            self.triangulate_convex(&mut indices, concave_vertex);
        }
        else {
            return None;
        }

        return Some(indices);
    }

}

impl<> PolygonFloat<f32, Vec2f> for Polygon<f32, Vec2f> {
        
    fn regular(center : Vec2f, radius : f32, corners : usize) -> Polygon<f32, Vec2f> {
        
        let angle_per_corner = f32::consts::TAU / (corners as f32);

        let mut poly = Polygon::<f32, Vec2f>::with_capacity(corners);

        for i in 0..corners {

            let current_angle : f32 = angle_per_corner * i as f32;

            let x_y = current_angle.sin_cos();
            let xy_vec = Vec2f::new(x_y.1, x_y.0) * 0.5_f32 * radius + center;

            poly.push(xy_vec);
        }

        return poly;
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


impl<T : Num + PartialOrd<T>, U : Vec2<T>> VertexProperties for Polygon<T, U> {

    fn get_vertex_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> Option<&mut <M as PropertyMap>::Storage> {
        return self.vertex_properties.get_property_map::<M>(property_type);
    }

    fn add_vertex_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> bool {
        return self.vertex_properties.add_property_map::<M>(property_type);
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

    #[test]
    fn test_regular_fan_triangulation() {

        let poly = Polygon::<f64, Vec2d>::regular(Vec2d::new(0.0, 0.0), 1.0, 8);
        let triangulation = poly.triangulate::<i32>();

        assert_eq!(triangulation.is_some(), true);

        let indices = triangulation.unwrap();

        assert_eq!(indices.len(), 6*3);
    }

    #[test]
    fn test_one_convex_vertex_fan_triangulation() {

        let mut poly = Polygon::<f64, Vec2d>::new();
        poly.push_vector(Vec::from([Vec2d::new(0.0, 0.0),
                                        Vec2d::new(1.0, 0.0),
                                        Vec2d::new(1.0, 1.0),
                                        Vec2d::new(0.0, 1.0),
                                        Vec2d::new(0.5, 0.5)]));

        let triangulation = poly.triangulate::<i32>();

        assert_eq!(triangulation.is_some(), true);

        let indices = triangulation.unwrap();

        assert_eq!(indices.len(), 3*3);
        assert_eq!(indices[0], 4);
    }

}