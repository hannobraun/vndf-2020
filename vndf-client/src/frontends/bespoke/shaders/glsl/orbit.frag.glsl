// ATTENTION:
// Changing this file will have no effect on the shader used in the game. You
// need to compile this file using `compile-shaders.sh` before anything you do
// shows up in the game.

#version 450

layout(set = 0, binding = 1) uniform Locals {
    vec4 color_in;
};

layout(location = 0) in  vec2 pos;
layout(location = 0) out vec4 color_out;

void main() {
    float x = 0.998;
    float r = length(pos);

    float alpha;
    if (x < r && r <= 1.0) {
        alpha = 0.5;
    }
    else if (r <= x) {
        alpha = 0.1;
    }
    else {
        alpha = 0.0;
    }

    color_out = vec4(color_in.rgb, color_in.a * alpha);
}
