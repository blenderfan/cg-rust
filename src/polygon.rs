
use crate::vector::Vec2;

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
