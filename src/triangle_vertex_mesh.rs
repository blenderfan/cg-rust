
//! #Triangle-Vertex Meshes
//! 
//! Meshes can have different representations and data structures. The most common representation for meshes
//! for rendering (GPUs) is a list of three indices referenceing tree vertices, thus forming a triangle. 
//! This is a special-case of the Face-Vertex Mesh, but more restricted, since arbitrary polygons are not
//! allowed.

use std::marker::PhantomData;

use crate::vector::Vec3;
use crate::mesh::Mesh;

use num_traits::PrimInt;
use num_traits::Num;

pub struct TriangleVertexMesh<T: Vec3<U>, U : Num + PartialOrd<U>, IndexType : PrimInt> {

    vertices : Vec<T>,
    indices : Vec<IndexType>,
    number_type : PhantomData<U>
}

impl<T: Vec3<U>, U: Num + PartialOrd<U>, IndexType : PrimInt> TriangleVertexMesh<T, U, IndexType> {

    pub fn new() -> Self {
        Self {
            vertices: Vec::<T>::new(),
            indices: Vec::<IndexType>::new(),
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

impl<T: Vec3<U>, U : Num + PartialOrd<U>, IndexType : PrimInt> Mesh<T, U, IndexType> for TriangleVertexMesh<T, U, IndexType> {

} 