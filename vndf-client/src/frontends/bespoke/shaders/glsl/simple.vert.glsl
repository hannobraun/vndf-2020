// ATTENTION:
// Changing this file will have no effect on the shader used in the game. You
// need to compile this file using `compile-shaders.sh` before anything you do
// shows up in the game.

#version 450


layout(set = 0, binding = 0) uniform Locals {
    mat4 transform;
};

layout(location = 0) in  vec2 pos_in;
layout(location = 0) out vec2 pos_out;


void main() {
    pos_out = pos_in;
    gl_Position = transform * vec4(pos_in, 0.0, 1.0);
}
