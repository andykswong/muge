[[group(1), binding(0)]]
var tex: texture_2d<f32>;
[[group(1), binding(1)]]
var tex_sampler: sampler;

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] uv: vec2<f32>;
    [[location(1)]] normal: vec3<f32>;
};

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(tex, tex_sampler, in.uv);
}
