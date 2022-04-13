struct Data {
    mvp: mat4x4<f32>;
    outline: vec4<f32>;
};
[[group(0), binding(0)]]
var<uniform> data: Data;

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] uv: vec2<f32>;
    [[location(1)]] normal: vec3<f32>;
};

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return data.outline;
}
