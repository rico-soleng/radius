#version 100

precision mediump float;

attribute vec2 position;
attribute vec2 tex_coords;

uniform mat2 mat;
        
varying vec2 v_tex_coords;

void main() {
    
    v_tex_coords = tex_coords;
    
    vec2 pos = mat * position;

    gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
}