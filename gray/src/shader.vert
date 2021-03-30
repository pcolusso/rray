#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 window_size;
layout(location=2) in float seed;
layout(location=3) in vec3 camera_origin;
layout(location=4) in vec3 camera_lower_left;
layout(location=5) in vec3 camera_horizontal;
layout(location=6) in float camera_lens_radius;

layout(location=0) out vec3 out_position;
layout(location=1) out vec2 out_window_size;
layout(location=2) out float out_seed;
layout(location=3) out vec3 out_camera_origin;
layout(location=4) out vec3 out_camera_lower_left;
layout(location=5) out vec3 out_camera_horizontal;
layout(location=6) out float out_camera_lens_radius;


void main() {
    out_window_size = window_size;
    out_seed = seed;
    out_camera_origin = camera_origin;
    out_camera_lower_left = camera_lower_left;
    out_camera_horizontal = camera_horizontal;
    out_camera_lens_radius = camera_lens_radius;
    out_position = a_position;
    
    gl_Position = vec4(a_position, 1.0);
}
 