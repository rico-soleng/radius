#version 100

precision mediump float;

attribute vec2 position;

uniform vec2 scale;
uniform float rot;
uniform vec2 loc;
uniform mat2 mat;

uniform vec2 dimensions;
        
attribute vec2 tex_coords;
varying vec2 v_tex_coords;
varying float t_2;

void main() {
    
    
    vec2 pos0 = mat * position * scale;
    
    vec2 pos_f;
    
    pos_f.x = pos0.x * cos(rot) - pos0.y * sin(rot);
    pos_f.y = pos0.y * cos(rot) + pos0.x * sin(rot);
    
    pos_f = pos_f + loc;
    
    v_tex_coords = tex_coords;
    vec2 pos = pos_f;
    vec2 pos2 = pos_f;

    gl_Position = vec4(pos2.x, pos2.y, 0.0, 1.0);
}