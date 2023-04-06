#version 330 core

attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
}