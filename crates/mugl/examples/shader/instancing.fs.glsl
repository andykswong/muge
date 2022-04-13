#version 300 es
precision mediump float;
layout(std140) uniform Data {
  vec4 ambient;
};
in vec3 vColor;
out vec4 outColor;
void main () {
  outColor = vec4(ambient.rgb + vColor, ambient.a);
}
