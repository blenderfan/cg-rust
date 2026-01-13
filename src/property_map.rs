
use std::any::Any;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum PropertyType {

    NORMAL = 0,
    COLOR = 1,
}


pub trait PropertyMap<T>  {

    fn new() -> Self;
    fn with_capacity(capacity : usize) -> Self;

    fn push(&mut self, value : T);
    fn set(&mut self, idx : usize, value : T);

    fn get(&self, idx : usize) -> T;

    fn len(&self) -> usize;

    fn property_type() -> PropertyType;
}


pub(in crate) struct PropertyStore {

    property_maps : HashMap<PropertyType, Box<dyn Any>>,
}

impl PropertyStore {

    pub fn new() -> Self {
        Self { property_maps: HashMap::new(), }
    }

    pub fn add_property_map<M: PropertyMap<T> + 'static, T>(&mut self, map : M) -> bool {

        if self.property_maps.contains_key(&M::property_type()) {
            return false;
        }

        let p = self.property_maps.insert(M::property_type(), Box::new(map));
        return Option::is_some(&p);
    }

    pub fn get_property_map<M: PropertyMap<T> + 'static, T>(&mut self, property_type : PropertyType) -> Option<&mut M> {

        match self.property_maps.get_mut(&property_type) {
            Some(map) => {
                return map.downcast_mut::<M>();
            }
            None => None,
        }
    }
}

pub trait VertexProperties {
    fn get_vertex_property<M: PropertyMap<T> + 'static, T>(&mut self, property_type : PropertyType) -> Option<&mut M>;
    fn add_vertex_property<M: PropertyMap<T> + 'static, T>(&mut self, map : M) -> bool;
}

pub trait FaceProperties {
    fn get_face_property<M: PropertyMap<T> + 'static, T>(&mut self, property_type: PropertyType) -> Option<&mut M>;
    fn add_face_property<M: PropertyMap<T> + 'static, T>(&mut self, map : M) -> bool;
}