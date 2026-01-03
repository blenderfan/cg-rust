
pub mod mesh_normals {

    use std::hash::Hash;

    use num_traits::Num;
    use num_traits::PrimInt;

    use crate::mesh_components::MeshComponent;
    use crate::vector::Vec3;
    use crate::mesh::Mesh;

    /* 
    pub fn create_angle_weighted_pseudo_vertex_normals<MeshType : Mesh<T, U, IndexType>, T: Vec3<U>, U : Num + PartialOrd<U>, IndexType : PrimInt + Hash>(mesh: MeshType) 
    {
        let incident_faces = mesh.make_incidence_map(MeshComponent::VERTEX, MeshComponent::FACE);

        for entry in incident_faces {

        }
    }
*/
}