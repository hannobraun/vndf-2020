// ATTENTION:
// Changing this file will have no effect on the shader used in the game. You
// need to compile this file using `compile-shaders.sh` before anything you do
// shows up in the game.

#version 450

layout(set = 0, binding = 1) uniform Locals {
    vec4  color_in;
    vec2  u_per_pixel;
    float orbiter_angle;
    float orbiter_dir;
};

layout(location = 0) in  vec2 pos;
layout(location = 0) out vec4 color_out;

void main() {
    const float LIMIT = 0.5;

    // This isn't really right, as it only takes into account the units per
    // pixel for the y axis. Orbits are "round enough" that I don't notice it on
    // screen though.
    float x = LIMIT - (2.0 * u_per_pixel[0]);
    float r = length(pos);

    float alpha;
    if (x < r && r <= LIMIT) {
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
