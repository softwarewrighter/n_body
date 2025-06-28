attribute vec3 a_position;
attribute vec4 a_color;

uniform mat4 u_projection;
uniform mat4 u_view;

varying vec4 v_color;

void main() {
    gl_Position = u_projection * u_view * vec4(a_position, 1.0);
    gl_PointSize = 2.0;
    v_color = a_color;
}