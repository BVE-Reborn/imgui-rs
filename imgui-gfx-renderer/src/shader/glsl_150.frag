#version 150

uniform sampler2D tex;

in vec2 f_uv;
in vec4 f_color;

out vec4 Target0;

void main() {
  Target0 = f_color * texture(tex, f_uv.st);
}
