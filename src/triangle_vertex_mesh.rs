
//! #Triangle-Vertex Meshes
//! 
//! Meshes can have different representations and data structures. The most common representation for meshes
//! for rendering (GPUs) is a list of three indices referenceing tree vertices, thus forming a triangle. 
//! This is a special-case of the Face-Vertex Mesh, but more restricted, since arbitrary polygons are not
//! allowed.

use std::marker::PhantomData;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use crate::property_map::PropertyType;
use crate::property_map::PropertyMap;
use crate::property_map::PropertyStore;
use crate::property_map::VertexProperties;
use crate::property_map::FaceProperties;
use crate::vector::FloatVector;
use crate::vector::Vec3;
use crate::mesh::Mesh;
use crate::mesh_components::MeshComponent;

use num_traits::PrimInt;
use num_traits::Num;
use num_traits::NumCast;
use num_traits::Unsigned;
use num_traits::Float;

#[repr(C)]
pub struct TriangleVertexMesh<T: Vec3<U> + FloatVector<U>, U : Num + PartialOrd<U> + Float, IndexType : PrimInt + Hash + Unsigned> {

    vertices : Vec<T>,
    indices : Vec<IndexType>,
    vertex_properties : PropertyStore,
    face_properties : PropertyStore,
    number_type : PhantomData<U>
}

impl<T: Vec3<U> + FloatVector<U>, U: Num + PartialOrd<U> + Float, IndexType : PrimInt + Hash + Unsigned> TriangleVertexMesh<T, U, IndexType> {

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



    pub fn get_indices(&self) -> &Vec<IndexType> {
        return &self.indices;
    }

    // Private

    fn make_vertex_vertex_incidence_map(&self, map : &mut HashMap<IndexType, HashSet<IndexType>>) {

        let size = self.indices.len();
        let triangle_count = size / 3;

        for i in 0..triangle_count {

            let idx_a = i * 3 + 0;
            let idx_b = i * 3 + 1;
            let idx_c = i * 3 + 2;

            let v_idx_a = self.indices[idx_a];
            let v_idx_b = self.indices[idx_b];
            let v_idx_c = self.indices[idx_c];

            unsafe {
                if !map.contains_key(&v_idx_a) {
                    map.insert(v_idx_a, HashSet::<IndexType>::new());
                }

                if !map.contains_key(&v_idx_b) {
                    map.insert(v_idx_b, HashSet::<IndexType>::new());
                }

                if !map.contains_key(&v_idx_c) {
                    map.insert(v_idx_c, HashSet::<IndexType>::new());
                }

                let incident_a = map.get_mut(&v_idx_a).unwrap_unchecked();
                incident_a.insert(v_idx_b);
                incident_a.insert(v_idx_c);

                let incident_b = map.get_mut(&v_idx_b).unwrap_unchecked();
                incident_b.insert(v_idx_a);
                incident_b.insert(v_idx_c);

                let incident_c = map.get_mut(&v_idx_c).unwrap_unchecked();
                incident_c.insert(v_idx_a);
                incident_c.insert(v_idx_b);
            }

        } 
    }

    fn make_vertex_face_incidence_map(&self, map: &mut HashMap<IndexType, HashSet<IndexType>>) {

        let size = self.indices.len();
        let triangle_count = size / 3;

        for i in 0..triangle_count {

            let idx_a = i * 3 + 0;
            let idx_b = i * 3 + 1;
            let idx_c = i * 3 + 2;

            let v_idx_a = self.indices[idx_a];
            let v_idx_b = self.indices[idx_b];
            let v_idx_c = self.indices[idx_c];

            unsafe {

                if !map.contains_key(&v_idx_a) {
                    map.insert(v_idx_a, HashSet::<IndexType>::new());
                }

                if !map.contains_key(&v_idx_b) {
                    map.insert(v_idx_b, HashSet::<IndexType>::new());
                }

                if !map.contains_key(&v_idx_c) {
                    map.insert(v_idx_c, HashSet::<IndexType>::new());
                }

                let idx = NumCast::from(i).unwrap();

                let incident_a = map.get_mut(&v_idx_a).unwrap_unchecked();
                incident_a.insert(idx);

                let incident_b = map.get_mut(&v_idx_b).unwrap_unchecked();
                incident_b.insert(idx);

                let incident_c = map.get_mut(&v_idx_c).unwrap_unchecked();
                incident_c.insert(idx);
            }
        }
    }

}

impl<T: Vec3<U> + FloatVector<U>, U : Num + PartialOrd<U> + Float, IndexType : PrimInt + Hash + Unsigned> VertexProperties for TriangleVertexMesh<T, U, IndexType> {
    fn get_vertex_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> Option<&mut <M as PropertyMap>::Storage> {
        return self.vertex_properties.get_property_map::<M>(property_type);
    }

    fn add_vertex_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> bool {
        return self.vertex_properties.add_property_map::<M>(property_type);
    }
}

impl<T: Vec3<U> + FloatVector<U>, U : Num + PartialOrd<U> + Float, IndexType : PrimInt + Hash + Unsigned> FaceProperties for TriangleVertexMesh<T, U, IndexType> {
    fn get_face_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> Option<&mut <M as PropertyMap>::Storage> {
        return self.face_properties.get_property_map::<M>(property_type);
    }

    fn add_face_property<M: PropertyMap>(&mut self, property_type: PropertyType) -> bool {
        return self.face_properties.add_property_map::<M>(property_type);
    }
}

impl<T: Vec3<U> + FloatVector<U>, U : Num + PartialOrd<U> + Float, IndexType : PrimInt + Hash + Unsigned> Mesh<T, U, IndexType> for TriangleVertexMesh<T, U, IndexType> {


    fn make_incidence_map(&self, origin_type : MeshComponent, incident_type : MeshComponent) -> HashMap<IndexType, HashSet<IndexType>> {

        let mut map = HashMap::<IndexType, HashSet<IndexType>>::new();

        match origin_type {

            MeshComponent::VERTEX => 
                match incident_type {
                    MeshComponent::VERTEX => self.make_vertex_vertex_incidence_map(&mut map),
                    MeshComponent::EDGE => todo!(),
                    MeshComponent::FACE => self.make_vertex_face_incidence_map(&mut map)
                }
            MeshComponent::EDGE => 
                match incident_type {
                    MeshComponent::VERTEX => todo!(),
                    MeshComponent::EDGE => todo!(),
                    MeshComponent::FACE => todo!()
                }
            MeshComponent::FACE => 
                match incident_type {
                    MeshComponent::VERTEX => todo!(),
                    MeshComponent::EDGE => todo!(),
                    MeshComponent::FACE => todo!()
                }
        }

        return map;
    }

    fn get_face(&self, idx: IndexType) -> Vec<IndexType> {

        let mut triangle_indices = Vec::<IndexType>::with_capacity(3);

        let idx = idx.to_usize().unwrap();
        let a = idx * 3 + 0;
        let b = idx * 3 + 1;
        let c = idx * 3 + 2;

        let idx_a = self.indices[a];
        let idx_b = self.indices[b];
        let idx_c = self.indices[c];

        triangle_indices.push(idx_a);
        triangle_indices.push(idx_b);
        triangle_indices.push(idx_c);

        return triangle_indices;
    }

    fn get_vertices(&self) -> &Vec<T> {
        return &self.vertices;
    }

} 


#[cfg(test)]
mod unit_tests {

    use crate::{mesh::Mesh, triangle_vertex_mesh::TriangleVertexMesh, vector::Vec3f};
    use crate::mesh_components::MeshComponent;

    fn create_unit_cube() -> TriangleVertexMesh<Vec3f, f32, u32> {

        //Unit Cube
        let vertices = Vec::from(
            [Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(1.0, 0.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
            Vec3f::new(1.0, 1.0, 0.0),
            Vec3f::new(0.0, 0.0, 1.0),
            Vec3f::new(1.0, 0.0, 1.0),
            Vec3f::new(0.0, 1.0, 1.0),
            Vec3f::new(1.0, 1.0, 1.0),
            ]);

        let indices = Vec::from(
            [
                //Front
                0, 1, 3,
                0, 3, 2,

                //Back
                5, 4, 6,
                5, 6, 7,

                //Left
                4, 0, 3,
                4, 3, 6,

                //Right
                1, 5, 7,
                1, 7, 3,

                //Top
                2, 3, 7,
                2, 7, 6,

                //Bottom
                4, 5, 1,
                4, 1, 0
            ]);

        let cube = TriangleVertexMesh::from(vertices, indices);
        return cube.unwrap();

    }

    #[test]
    fn test_vertex_vertex_map() {

        let ucube = create_unit_cube();

        let vv_map = ucube.make_incidence_map(MeshComponent::VERTEX, MeshComponent::VERTEX);
        assert_eq!(vv_map.len(), 8);

        let v0_incidence = vv_map.get(&0);
        assert!(v0_incidence.is_some());

        let uv0_incidence = v0_incidence.unwrap();
        assert_eq!(uv0_incidence.len(), 4);

    }

    #[test]
    fn test_vertex_face_map() {

        let ucube = create_unit_cube();

        let vertex_triangle_map = ucube.make_incidence_map(MeshComponent::VERTEX, MeshComponent::FACE);
        assert_eq!(vertex_triangle_map.len(), 8);

        let v0_incidence = vertex_triangle_map.get(&0);
        assert!(v0_incidence.is_some());

        let uv0_incidence = v0_incidence.unwrap();
        assert_eq!(uv0_incidence.len(), 4);
    }
}