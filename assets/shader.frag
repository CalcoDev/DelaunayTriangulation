#version 330 core

precision lowp float;
uniform vec2 canvasSize;
uniform vec4 _Time;
uniform sampler2D Texture;

void main() {
    float time = _Time.x;
    vec2 coord = gl_FragCoord.xy / canvasSize.xy; // [0,1]
    coord = 2. * (coord - 1.); // [-1,1]
    
    float scale = sin(time) + 1.;
    coord = coord / scale;
    
    // if (abs(coord.x) < 1. && abs(coord.y) < 1.) {
        coord = (coord + 1.) / 2.;
        gl_FragColor = vec4(coord.x, coord.y, 1. - coord.x, 1);
    // } else {
        // gl_FragColor = vec4(1,1,1,1); // white
    // }
}