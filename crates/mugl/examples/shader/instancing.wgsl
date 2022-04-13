// Vertex shader

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] offset: vec2<f32>;
    [[location(2)]] color: vec3<f32>;
    [[location(3)]] angle: f32;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(
        cos(model.angle) * model.position[0] + sin(model.angle) * model.position[1] + model.offset[0],
        -sin(model.angle) * model.position[0] + cos(model.angle) * model.position[1] + model.offset[1],
        0.0,
        1.0
    );
    return out;
}

// Fragment shader

struct Data {
    ambient: vec4<f32>;
};
[[group(0), binding(0)]]
var<uniform> data: Data;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(data.ambient.rgb + in.color, data.ambient[3]);
}
