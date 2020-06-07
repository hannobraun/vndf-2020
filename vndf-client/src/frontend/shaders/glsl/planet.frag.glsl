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
    const float LIMIT = 1.0;

    float alpha;
    if (length(pos) <= LIMIT) {
        alpha = 1.0;
    }
    else {
        alpha = 0.0;
    }

    color_out = vec4(color_in.rgb, color_in.a * alpha);
}
