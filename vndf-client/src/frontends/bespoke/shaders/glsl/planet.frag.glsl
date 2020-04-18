// ATTENTION:
// Changing this file will have no effect on the shader used in the game. You
// need to compile this file using `compile-shaders.sh` before anything you do
// shows up in the game.

#version 450

layout(location = 0) in  vec2 pos;
layout(location = 0) out vec4 color;

void main() {
    float alpha;
    if (length(pos) <= 1.0) {
        alpha = 1.0;
    }
    else {
        alpha = 0.0;
    }

    color = vec4(1.0, 1.0, 1.0, alpha);
}
