// ATTENTION:
// Changing this file will have no effect on the shader used in the game. You
// need to compile this file using `compile-shaders.sh` before anything you do
// shows up in the game.

#version 450

layout(set = 0, binding = 1) uniform Locals {
    vec4 color_in;
};

layout(location = 0) out vec4 color_out;

void main() {
    color_out = color_in;
}
