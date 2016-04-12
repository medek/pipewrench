#version 100

uniform lowp mat4 matrix;

attribute lowp vec2 position;
attribute lowp vec4 color;

varying lowp vec4 v_color;

void main() {
	gl_Position = matrix * vec4(position, 0.0, 1.0);
	v_color = color;
}

