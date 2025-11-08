
//! #Triangle-Vertex Meshes
//! 
//! Meshes can have different representations and data structures. The most common representation for meshes
//! for rendering (GPUs) is a list of three indices referenceing tree vertices, thus forming a triangle. 
//! This is a special-case of the Face-Vertex Mesh, but more restricted, since arbitrary polygons are not
//! allowed.

use crate::vector::Vec3;
use crate::mesh::Mesh;

use num_traits::PrimInt;

pub struct TriangleVertexMesh<T: Vec3, IndexType : PrimInt> {

    vertices : Vec<T>,
    indices : Vec<IndexType>
}

impl<T: Vec3, IndexType : PrimInt> TriangleVertexMesh<T, IndexType> {

    pub fn new() -> Self {
        Self {
            vertices: Vec::<T>::new(),
            indices: Vec::<IndexType>::new()
        }
    }

    pub fn get_vertices(&self) -> &Vec<T> {
        return &self.vertices;
    }

    pub fn get_indices(&self) -> &Vec<IndexType> {
        return &self.indices;
    }
}

impl<T: Vec3, IndexType : PrimInt> Mesh<T, IndexType> for TriangleVertexMesh<T, IndexType> {

} 