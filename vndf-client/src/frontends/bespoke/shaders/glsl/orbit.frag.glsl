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
    float d   = 0.002;
    float min = 1.0 - d;
    float max = 1.0 + d;

    float r = length(pos);

    float alpha;
    if (min <= r && r <= max) {
        alpha = 0.5;
    }
    else if (r < min) {
        alpha = 0.1;
    }
    else {
        alpha = 0.0;
    }

    color_out = vec4(color_in.rgb, color_in.a * alpha);
}
