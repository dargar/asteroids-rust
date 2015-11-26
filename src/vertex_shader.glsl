#version 430 core

layout(location = 0) in vec4 position;

layout(location = 1) uniform mat4 projection;
layout(location = 3) uniform mat4 model;

void main() {
    gl_Position = projection * model * position;
}
