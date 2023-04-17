#version 100

precision mediump float;

varying vec2 v_tex_coords;

uniform bool is_exr;
uniform float intensity;
uniform float compression_factor;
uniform sampler2D tex;
        
void main() {
    
    
    if (is_exr){
        float c = 0.0;
        vec4 ad = texture2D(tex, v_tex_coords*2.0);
        vec3 col = vec3(ad.x, ad.y, ad.z) * compression_factor - vec3(c,c,c);
        
        col = col* intensity;
        
        float avg = (col.x + col.y + col.z) / 3.0;
        
        
        vec3 col_desat = vec3(avg, avg, avg) + ( col - vec3(avg, avg, avg) ) * 0.7;
        float luminence_pre = (0.2126*col_desat.x + 0.7152*col_desat.y + 0.0722*col_desat.z);
        
        
        vec3 col_clamp = clamp(col, 0.0, 8.0);
        float luminence_post = (0.2126*col_clamp.x + 0.7152*col_clamp.y + 0.0722*col_clamp.z);
        
        float lum_diff = luminence_pre - luminence_post;
        
        vec3 col_final = col_clamp + vec3(lum_diff, lum_diff, lum_diff);
        
        
        float m =  max(col.x, max(col.y, col.z));
        
        float r = 1.0;
       
        r = ((m*(9.0/8.0))/(1.0 + m));
        
        vec3 col_div_m = col_final / m;
        
        vec3 col_post_map = col_div_m * r;
        
        //lum_post_map
        float lpm =  r;
        
        vec3 col_post_map_desat = vec3(lpm,lpm,lpm) + ( col - vec3(lpm,lpm,lpm) ) * (1.0 - clamp(r*(2.0*r - 1.0) + (1.0-r)*3.0*pow(r,1.85), 0.0, 1.0));        
        
        float p = 1.0/2.2;
        
        gl_FragColor = vec4(pow(col_post_map_desat, vec3(p, p, p)), ad[3]);
    }else{
        vec4 ad = texture2D(tex, v_tex_coords*2.0);
        vec3 col = vec3(ad.x, ad.y, ad.z) * intensity;
        gl_FragColor = vec4(col, ad[3]);
    }
    
}
 
