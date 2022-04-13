#version 300 es
precision mediump float;
layout(std140) uniform Data {
  mat4 mvp;
  vec4 outline;
};
out vec4 outColor;
void main () {
  outColor = outline;
}
