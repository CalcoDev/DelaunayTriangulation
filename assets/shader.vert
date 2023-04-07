#version 450 core

in vec3 position;
in vec2 texcoord;
in vec4 color0;

uniform mat4 Model;
uniform mat4 Projection;

uniform vec3 gradient_a;
uniform vec3 gradient_b;

flat out vec4 vColor;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    
    float color = (gl_Position.y + 1.0) / 2.0;
    
    vec3 gradient = mix(gradient_a, gradient_b, color);
    vColor = vec4(gradient, 1.0);
}