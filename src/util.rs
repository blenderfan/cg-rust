

use crate::cardinal_direction::plane_indices;
use crate::vector::*;
use crate::cardinal_direction::CardinalDirection;
use crate::cardinal_direction::as_index;

pub fn embed_to_3f(axis: CardinalDirection, default_value: f32, point: Vec2f) -> Vec3f {

    let idx = as_index(&axis);
    let plane_indices = plane_indices(axis);

    let mut new_v = Vec3f::default();

    new_v[idx] = default_value;
    new_v[plane_indices.0] = point[0];
    new_v[plane_indices.1] = point[1];

    return new_v;
}

pub fn embed_to_3d(axis: CardinalDirection, default_value: f64, point: Vec2d) -> Vec3d {

    let idx = as_index(&axis);
    let plane_indices = plane_indices(axis);

    let mut new_v = Vec3d::default();

    new_v[idx] = default_value;
    new_v[plane_indices.0] = point[0];
    new_v[plane_indices.1] = point[1];

    return new_v;
}


pub fn embed_vertices_to_3f(axis: CardinalDirection, default_value: f32, points: Vec<Vec2f>) -> Vec<Vec3f> {

    let idx = as_index(&axis);
    let plane_indices = plane_indices(axis);
    let l = points.len();

    let mut new_vertices = Vec::<Vec3f>::with_capacity(l);

    for v in points {

        let mut new_v = Vec3f::default();

        new_v[idx] = default_value;
        new_v[plane_indices.0] = v[0];
        new_v[plane_indices.1] = v[1];

        new_vertices.push(new_v);
    }

    return new_vertices;
}

pub fn embed_vertices_to_3d(axis: CardinalDirection, default_value: f64, points: Vec<Vec2d>) -> Vec<Vec3d> {

    let idx = as_index(&axis);
    let plane_indices = plane_indices(axis);
    let l = points.len();

    let mut new_vertices = Vec::<Vec3d>::with_capacity(l);

    for v in points {

        let mut new_v = Vec3d::default();

        new_v[idx] = default_value;
        new_v[plane_indices.0] = v[0];
        new_v[plane_indices.1] = v[1];

        new_vertices.push(new_v);
    }

    return new_vertices;
}