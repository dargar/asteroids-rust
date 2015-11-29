#version 430 core

layout(location = 0) in vec4 position;

layout(location = 1) uniform mat4 mvp;

void main() {
    gl_Position = mvp * position;
}
