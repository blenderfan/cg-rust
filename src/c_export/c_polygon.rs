

use std::mem;
use libc::size_t;

use crate::c_export::c_arr::PArray;
use crate::c_export::c_vector::CVec2f;

use crate::vector::Vec2f;
use crate::polygon::Polygon;
use crate::polygon::PolygonFloat;
 
#[unsafe(no_mangle)]
unsafe extern "C" fn cg_rust_polygon_triangulate(points: &PArray<CVec2f>) -> * mut PArray<size_t> {

    let mut poly = Polygon::<f32, Vec2f>::new();
    let size = points.size;

    unsafe {

        for i in 0..size {

            let p = points.data.offset(i as isize);

            let vec : Vec2f = Vec2f::new((*p).x, (*p).y);
            poly.push(vec);
        }

        let triangulation = poly.triangulate::<size_t>();
        if Option::is_some(&triangulation) {
            let mut unwrapped_triangulation = triangulation.unwrap();
            unwrapped_triangulation.shrink_to_fit();
            let ptr = unwrapped_triangulation.as_mut_ptr();
            let len = unwrapped_triangulation.len();
            mem::forget(unwrapped_triangulation);

            let c_triangulation = PArray::<size_t>::new(ptr, len);
            let triangle_box = Box::new(c_triangulation);

            return Box::into_raw(triangle_box);
        }
        return std::ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn cg_rust_polygon_regular(center : CVec2f, radius : f32, corners: size_t) -> * mut PArray<CVec2f> {

    let mut poly = Polygon::<f32, Vec2f>::regular(Vec2f::new(center.x, center.y), radius, corners);

    let points: &mut Vec<Vec2f> = poly.get_mut_points();
    points.shrink_to_fit();
    let ptr = points.as_mut_ptr();
    let len = points.len();

    let c_points = PArray::<CVec2f>::new(ptr as *mut CVec2f, len);
    let points_box = Box::new(c_points);

    mem::forget(poly);

    return Box::into_raw(points_box);
}