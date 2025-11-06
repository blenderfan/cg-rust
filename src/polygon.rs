

use core::f64;

use crate::vector::Vec2;
use crate::vector::Vec2d;

pub trait PolygonFloat<T : Vec2, ElementType> {
    fn regular(center : T, radius : ElementType, corners : i32) -> Polygon<T>;
}



pub struct Polygon<T: Vec2> {

    points : Vec<T>

}

impl<T: Vec2> Polygon<T> {

    pub fn new() -> Self {
        Self { points: Vec::<T>::new() }
    }

    pub fn get_points(&self) -> &Vec<T> {
        return &self.points;
    }

    pub fn push(&mut self, point : T) {
        self.points.push(point);
    }

}

impl<> PolygonFloat<Vec2d, f64> for Polygon<Vec2d> {

    fn regular(center : Vec2d, radius : f64, corners : i32) -> Polygon<Vec2d> {
        
        let angle_per_corner = f64::consts::TAU / (corners as f64);

        let mut poly = Polygon::<Vec2d>::new();

        let mut current_angle = 0_f64;
        for _ in 0..corners {

            let x_y = current_angle.sin_cos();
            let x = x_y.1 * 0.5_f64 * radius;
            let y = x_y.0 * 0.5_f64 * radius;

            let mut p = Vec2d::new(x, y);
            p += center;

            poly.push(p);

            current_angle += angle_per_corner;
        }

        return poly;
    }
}
