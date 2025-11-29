
#![feature(portable_simd)]

pub mod traits;
pub mod property_map;

pub mod cardinal_direction;
pub mod vector;
pub mod util;

pub mod polygon;

pub mod mesh;
pub mod triangle_vertex_mesh;

#[cfg(feature = "c_export")]
pub mod c_export;