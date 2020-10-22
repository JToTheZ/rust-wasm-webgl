attribute vec4 position;

void main() {
    gl_Position = position;
    gl_PointSize = 2.0;
}