#version 300 es
precision mediump float;
uniform samplerCube tex;
in vec3 vNormal;
out vec4 outColor;
void main () {
  outColor = texture(tex, normalize(vNormal));
}
