// Vertex shader

struct Data {
    mvp: mat4x4<f32>;
    outline: vec4<f32>;
};
[[group(0), binding(0)]]
var<uniform> data: Data;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] uv: vec2<f32>;
    [[location(1)]] normal: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = data.mvp * vec4<f32>(model.position, 1.0);
    out.uv = model.uv;
    out.normal = normalize(model.position);
    return out;
}
