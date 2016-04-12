#version 110

uniform mat4 matrix;

attribute vec2 position;
attribute vec4 color;

varying vec4 v_color;

void main() {
	gl_Position = matrix * vec4(position, 0.0, 1.0);
	v_color = color;
}