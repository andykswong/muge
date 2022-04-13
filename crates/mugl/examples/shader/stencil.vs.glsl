#version 300 es
precision mediump float;
layout(std140) uniform Data {
  mat4 mvp;
  vec4 outline;
};
layout (location=0) in vec3 position;
layout (location=1) in vec2 uv;
out vec2 vUv;
out vec3 vNormal;
void main(void) {
  vUv = uv;
  vNormal = normalize(position);
  gl_Position = mvp * vec4(position, 1.0);
}
