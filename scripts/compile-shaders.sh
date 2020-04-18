#!/usr/bin/env bash
set -e

SHADERS=vndf-client/src/frontends/bespoke/shaders

glslc \
    -fshader-stage=vertex \
    $SHADERS/glsl/shader.vert.glsl \
    -o $SHADERS/spv/shader.vert.spv

glslc \
    -fshader-stage=fragment \
    $SHADERS/glsl/shader.frag.glsl \
    -o $SHADERS/spv/shader.frag.spv
