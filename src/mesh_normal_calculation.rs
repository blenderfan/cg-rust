
pub mod mesh_normals {

    use std::hash::Hash;

    use num_traits::Num;
    use num_traits::PrimInt;
    use num_traits::Unsigned;
    use num_traits::Float;

    use crate::mesh_components::MeshComponent;
    use crate::vector::FloatVector;
    use crate::vector::Vec3;
    use crate::mesh::Mesh;

    pub fn calculate_face_normal<T: Vec3<U> + FloatVector<U>, U : Num + PartialOrd<U> + Float>(vertices: Vec<T>) -> Option<T> {
        return None;
    }

    /// Creates normals for vertices based on incident faces, such that larger angles, formed by the two adjacent edges to the vertex of the face,
    /// have a higher weight than smaller ones
    pub fn create_angle_weighted_pseudo_vertex_normals<MeshType : Mesh<T, U, IndexType>, T: Vec3<U> + FloatVector<U>, U : Num + PartialOrd<U> + Float, IndexType : PrimInt + Hash + Unsigned>(mesh: &MeshType) 
    {
        let incident_faces = mesh.make_incidence_map(MeshComponent::VERTEX, MeshComponent::FACE);
        let vertices = mesh.get_vertices();

        for entry in incident_faces {

            let vertex_idx = entry.0;
            let vertex = vertices[vertex_idx.to_usize().unwrap()];

            let face_indices = entry.1;

            let incident_faces_length = face_indices.len();

            let mut counter = 0;
            let mut angles = Vec::<U>::with_capacity(incident_faces_length);
            let mut normals = Vec::<T>::with_capacity(incident_faces_length);

            let mut angle_sum = U::zero();

            for face_idx in face_indices {

                let face = mesh.get_face(face_idx);
                let face_size = face.len();

                let mut next_vertex_idx = vertex_idx;
                let mut prev_vertex_idx = vertex_idx;

                for i in 0..face_size {

                    let face_vertex_idx = face[i];
                    if face_vertex_idx == vertex_idx {

                        let next_i = (i + 1).rem_euclid(face_size);
                        let prev_i = (i - 1).rem_euclid(face_size);
                        next_vertex_idx = face[next_i];
                        prev_vertex_idx = face[prev_i];
                    }
                }

                let next_vertex = vertices[next_vertex_idx.to_usize().unwrap()];
                let prev_vertex = vertices[prev_vertex_idx.to_usize().unwrap()];
                
                let prev_edge = vertex - prev_vertex;
                let next_edge = next_vertex - vertex;

                let angle = T::angle(&prev_edge, &next_edge);

                angles[counter] = angle;

                angle_sum = angle_sum + angle;
                counter += 1;
            }
        }
    }

}