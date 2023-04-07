#version 450 core

precision lowp float;

flat in vec4 vColor;
out vec4 color;

void main() {
    color = vColor;
}