
pub mod mesh_normals {

    use std::hash::Hash;

    use num_traits::Num;
    use num_traits::PrimInt;
    use num_traits::Unsigned;
    use num_traits::Float;

    use crate::common_properties::NormalMap;
    use crate::mesh_components::MeshComponent;
    use crate::property_map::PropertyMap;
    use crate::vector::FloatVector;
    use crate::vector::Vec3;
    use crate::mesh::Mesh;

    pub fn calculate_face_normal<T: Vec3<U> + FloatVector<U>, U : Num + PartialOrd<U> + Float>(vertices: Vec<T>) -> Option<T> {

        let mut sum = T::zero();

        let size = vertices.len();
        if size < 3 { return None; }

        let mut current = vertices[0];

        for i in 0..size {

            let mut next = vertices[(i + 1) % size];

            let diff = current - next;
            let add = current + next;

            sum += T::new(diff.y() * add.z(), diff.z() * add.x(), diff.x() * add.y());

            current = next;
        }

        let n = sum.normalize();
        if n.is_ok() { return n.ok();}
        else { return None; }
    }

    /// Creates normals for vertices based on incident faces, such that larger angles, formed by the two adjacent edges to the vertex of the face,
    /// have a higher weight than smaller ones
    pub fn create_angle_weighted_pseudo_vertex_normals<MeshType : Mesh<T, U, IndexType>, T: Vec3<U> + FloatVector<U> + 'static, U : Num + PartialOrd<U> + Float + 'static, IndexType : PrimInt + Hash + Unsigned>(mesh: &MeshType) 
        -> NormalMap<T, U>
    {
        let incident_faces = mesh.make_incidence_map(MeshComponent::VERTEX, MeshComponent::FACE);
        let vertices = mesh.get_vertices();

        let mut normal_map = NormalMap::<T, U>::with_size(vertices.len(), T::zero());

        for entry in incident_faces {

            let vertex_idx = entry.0;
            let idx = vertex_idx.to_usize().unwrap();
            let vertex = vertices[idx];

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
                let mut face_vertices = Vec::<T>::with_capacity(face_size);

                for i in 0..face_size {

                    let face_vertex_idx = face[i];
                    if face_vertex_idx == vertex_idx {

                        let next_i = (i + 1).rem_euclid(face_size);
                        let prev_i = (i as i32 - 1).rem_euclid(face_size as i32) as usize;
                        next_vertex_idx = face[next_i];
                        prev_vertex_idx = face[prev_i];
                    }
                    face_vertices.push(vertices[face_vertex_idx.to_usize().unwrap()]);
                }

                let next_vertex = vertices[next_vertex_idx.to_usize().unwrap()];
                let prev_vertex = vertices[prev_vertex_idx.to_usize().unwrap()];
                
                let prev_edge = vertex - prev_vertex;
                let next_edge = next_vertex - vertex;

                let angle = T::angle(&prev_edge, &next_edge);

                angles.push(angle);
                let face_normal = calculate_face_normal(face_vertices);
                if face_normal.is_some() {
                    normals.push(face_normal.unwrap());
                } else {
                    normals.push(T::zero());
                }

                angle_sum = angle_sum + angle;
                counter += 1;
            }

            let mut vertex_normal = T::zero(); 
            for i in 0..counter {

                let weight = angles[i] / angle_sum;
                let weighted_normal = normals[i] * weight;
                vertex_normal += weighted_normal;
            }
            let vn = vertex_normal.normalize();

            if vn.is_ok() {
                normal_map.set(idx, vn.unwrap());
            } else {
                normal_map.set(idx, T::zero());
            }

        }

        return normal_map;
    }

    #[cfg(test)]
    mod unit_tests {

        use crate::mesh::Mesh;
        use crate::mesh_normal_calculation::mesh_normals::create_angle_weighted_pseudo_vertex_normals;
        use crate::property_map::VertexProperties;
        use crate::vector::Vec3f;
        use crate::vector::Vec3;
        use crate::triangle_vertex_mesh::TriangleVertexMesh;

        fn create_unit_square() -> TriangleVertexMesh<Vec3f, f32, u32> {

            //Unit Square
            let vertices = Vec::from(
                [Vec3f::new(0.0, 0.0, 0.0),
                Vec3f::new(1.0, 0.0, 0.0),
                Vec3f::new(1.0, 1.0, 0.0),
                Vec3f::new(0.0, 1.0, 0.0)]
            );

            let indices = Vec::from(
                [
                    0, 1, 2,
                    0, 2, 3
                ]
            );

            let square = TriangleVertexMesh::from(vertices, indices);
            return square.unwrap();
        }

        #[test]
        fn test_create_mesh_normals() {

            let mut usquare = create_unit_square();

            let normals = create_angle_weighted_pseudo_vertex_normals(&usquare);

            usquare.add_vertex_property(normals);
        }

    }

}