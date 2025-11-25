
use std::any::Any;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum PropertyType {

    NORMAL = 0,
    COLOR = 1,
}

pub trait PropertyMap: Sized + Any {
    type Storage: PropertyBase<Self>;

}

pub trait PropertyBase<T>: Any {

    fn new() -> Self where Self: Sized;

    fn insert(&mut self, value: T);

    fn property_type(&mut self) -> PropertyType;
}

pub(in crate) struct PropertyStore {

    property_maps : HashMap<PropertyType, Box<dyn Any>>,
}

impl PropertyStore {

    pub fn new() -> Self {
        Self { property_maps: HashMap::new(), }
    }

    pub fn add_property_map<M: PropertyMap>(&mut self, property_type: PropertyType) -> bool {

        if self.property_maps.contains_key(&property_type) {
            return false;
        }

        let new_map = <M as PropertyMap>::Storage::new();
        let p = self.property_maps.insert(property_type, Box::new(new_map));
        return Option::is_some(&p);
    }

    pub fn get_property_map<M: PropertyMap>(&mut self, property_type: PropertyType) -> Option<&mut <M as PropertyMap>::Storage> {

        match self.property_maps.get_mut(&property_type) {
            Some(map) => {
                return map.downcast_mut::<<M as PropertyMap>::Storage>();
            }
            None => None,
        }
    }
}

pub trait VertexProperties {
    fn get_vertex_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> Option<&mut <M as PropertyMap>::Storage>;
    fn add_vertex_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> bool;
}

pub trait FaceProperties {
    fn get_face_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> Option<&mut <M as PropertyMap>::Storage>;
    fn add_face_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> bool;
}