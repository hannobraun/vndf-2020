#!/usr/bin/env bash
set -e

SHADERS=vndf-client/src/frontend/bespoke/shaders

glslc \
    -fshader-stage=vertex \
    $SHADERS/shader.vert.glsl \
    -o $SHADERS/shader.vert.spv

glslc \
    -fshader-stage=fragment \
    $SHADERS/shader.frag.glsl \
    -o $SHADERS/shader.frag.spv
