
use num_traits::PrimInt;
use num_traits::Num;

use crate::vector::Vec3;
use crate::property_map::VertexProperties;
use crate::property_map::FaceProperties;

pub trait Mesh<T: Vec3<U>, U : Num + PartialOrd<U>, IndexType : PrimInt> : VertexProperties + FaceProperties {
    
}
