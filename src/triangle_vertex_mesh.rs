
//! #Triangle-Vertex Meshes
//! 
//! Meshes can have different representations and data structures. The most common representation for meshes
//! for rendering (GPUs) is a list of three indices referenceing tree vertices, thus forming a triangle. 
//! This is a special-case of the Face-Vertex Mesh, but more restricted, since arbitrary polygons are not
//! allowed.

use std::marker::PhantomData;

use crate::property_map::PropertyType;
use crate::property_map::PropertyMap;
use crate::property_map::PropertyStore;
use crate::property_map::VertexProperties;
use crate::property_map::FaceProperties;
use crate::vector::Vec3;
use crate::mesh::Mesh;

use num_traits::PrimInt;
use num_traits::Num;

#[repr(C)]
pub struct TriangleVertexMesh<T: Vec3<U>, U : Num + PartialOrd<U>, IndexType : PrimInt> {

    vertices : Vec<T>,
    indices : Vec<IndexType>,
    vertex_properties : PropertyStore,
    face_properties : PropertyStore,
    number_type : PhantomData<U>
}

impl<T: Vec3<U>, U: Num + PartialOrd<U>, IndexType : PrimInt> TriangleVertexMesh<T, U, IndexType> {

    pub fn new() -> Self {
        Self {
            vertices: Vec::<T>::new(),
            indices: Vec::<IndexType>::new(),
            vertex_properties: PropertyStore::new(),
            face_properties: PropertyStore::new(),
            number_type: PhantomData
        }
    }

    /// Construct a Triangle-Vertex Mesh by providing a list of vertices and indices.
    /// Note that the number of indices must be a multiple of three.
    pub fn from(vertices : Vec<T>, indices : Vec<IndexType>) -> Option<Self> {

        if indices.len() % 3 != 0 {
            return None;
        }
        
        Some(Self {
            vertices: vertices,
            indices: indices,
            vertex_properties: PropertyStore::new(),
            face_properties: PropertyStore::new(),
            number_type: PhantomData
        })
    }

    pub fn get_vertices(&self) -> &Vec<T> {
        return &self.vertices;
    }

    pub fn get_indices(&self) -> &Vec<IndexType> {
        return &self.indices;
    }
}

impl<T: Vec3<U>, U : Num + PartialOrd<U>, IndexType : PrimInt> VertexProperties for TriangleVertexMesh<T, U, IndexType> {
    fn get_vertex_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> Option<&mut <M as PropertyMap>::Storage> {
        return self.vertex_properties.get_property_map::<M>(property_type);
    }

    fn add_vertex_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> bool {
        return self.vertex_properties.add_property_map::<M>(property_type);
    }
}

impl<T: Vec3<U>, U : Num + PartialOrd<U>, IndexType : PrimInt> FaceProperties for TriangleVertexMesh<T, U, IndexType> {
    fn get_face_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> Option<&mut <M as PropertyMap>::Storage> {
        return self.face_properties.get_property_map::<M>(property_type);
    }

    fn add_face_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> bool {
        return self.face_properties.add_property_map::<M>(property_type);
    }
}

impl<T: Vec3<U>, U : Num + PartialOrd<U>, IndexType : PrimInt> Mesh<T, U, IndexType> for TriangleVertexMesh<T, U, IndexType> {


} 


