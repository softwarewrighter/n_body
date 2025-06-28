precision mediump float;

varying vec4 v_color;

void main() {
    vec2 coord = gl_PointCoord - vec2(0.5);
    if (length(coord) > 0.5) {
        discard;
    }
    
    float intensity = 1.0 - length(coord) * 2.0;
    gl_FragColor = v_color * intensity;
}