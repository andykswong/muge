#version 300 es
layout (location=0) in vec2 position;
layout (location=1) in vec2 offset;
layout (location=2) in vec3 color;
layout (location=3) in float angle;
out vec3 vColor;
void main() {
  gl_Position = vec4(
    cos(angle) * position.x + sin(angle) * position.y + offset.x,
    -sin(angle) * position.x + cos(angle) * position.y + offset.y,
    0., 1.);
  vColor = color;
}
