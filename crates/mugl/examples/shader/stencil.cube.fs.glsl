#version 300 es
precision mediump float;
in vec2 vUv;
out vec4 outColor;
uniform sampler2D tex;
const float GAMMA = 2.2;
vec3 linearTosRGB(vec3 color) {
  return pow(color, vec3(1./GAMMA));
}
void main () {
  vec4 color = texture(tex, vUv);
  outColor = vec4(linearTosRGB(color.rgb), color.a);
}
