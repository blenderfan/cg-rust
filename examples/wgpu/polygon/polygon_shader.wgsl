
struct Data {
    proj: mat4x4<f32>,
    proj_inv: mat4x4<f32>,
    view: mat4x4<f32>,
    cam_pos: vec4<f32>,
};

@group(0)
@binding(0)
var<uniform> r_data: Data;

struct PolygonOutput {
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vs_main(@location(0) pos: vec3<f32>) -> PolygonOutput {
    var result : PolygonOutput;
    result.position = r_data.proj * r_data.view * vec4<f32>(pos, 1.0);
    return result;
}

@fragment
fn fs_main(vertex: PolygonOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.5, 0.0, 0.0, 1.0);
}
