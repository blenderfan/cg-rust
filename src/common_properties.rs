
use std::marker::PhantomData;

use num_traits::Num;
use num_traits::Float;

use crate::property_map::PropertyType;
use crate::property_map::PropertyMap;
use crate::vector::{FloatVector, Vec3};

pub struct NormalMap<T : Vec3<U> + FloatVector<U>, U : Num + Float + PartialOrd<U>> {

    data : Vec<T>,
    vector_type : PhantomData<T>,
    numeric_type : PhantomData<U>
}

impl<T : Vec3<U> + FloatVector<U>, U : Num + Float + PartialOrd<U>> NormalMap<T, U> {
    
    pub fn with_size(size : usize, default_value : T) -> Self {

        let mut resized_arr = Vec::<T>::with_capacity(size);
        resized_arr.resize(size, default_value);
        Self {
            data : resized_arr,
            vector_type : PhantomData,
            numeric_type : PhantomData
        }

    }
}

impl<T : Vec3<U> + FloatVector<U> + 'static, U : Num + Float + PartialOrd<U> + 'static> PropertyMap<T> for NormalMap<T, U> {

    fn new() -> Self{
        Self {
            data : Vec::<T>::new(),
            vector_type : PhantomData,
            numeric_type : PhantomData
        }
    }
    
    fn with_capacity(capacity : usize) -> Self {
        Self {
            data : Vec::<T>::with_capacity(capacity),
            vector_type : PhantomData,
            numeric_type : PhantomData
        }
    }


    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn set(&mut self, idx : usize, value : T) {
        self.data[idx] = value;
    }

    fn property_type() -> PropertyType {
        return PropertyType::NORMAL;
    }

}