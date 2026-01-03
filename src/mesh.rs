
use num_traits::PrimInt;
use num_traits::Num;

use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use crate::mesh_components::MeshComponent;
use crate::vector::Vec3;
use crate::property_map::VertexProperties;
use crate::property_map::FaceProperties;

pub trait Mesh<T: Vec3<U>, U : Num + PartialOrd<U>, IndexType : PrimInt + Hash> : VertexProperties + FaceProperties {
    
    ///Creates an incidence map between different basic components. E.g. by setting the origin type to vertex
    /// and the incident type to faces, a map is created where for each vertex, all incident faces are provided
    /// as a list. 
    fn make_incidence_map(self, origin_type : MeshComponent, incident_type : MeshComponent) -> HashMap<IndexType, HashSet<IndexType>>;

}
