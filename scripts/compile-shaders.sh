#!/usr/bin/env bash
set -e

SHADERS=vndf-client/src/frontends/bespoke/shaders

mkdir -p $SHADERS/spv

for SHADER_SOURCE in $SHADERS/glsl/*.vert.glsl; do
    FILE=$(basename $SHADER_SOURCE)
    NAME=${FILE%.*.*}

    glslc \
        -fshader-stage=vertex \
        $SHADER_SOURCE \
        -o $SHADERS/spv/$NAME.vert.spv
done

for SHADER_SOURCE in $SHADERS/glsl/*.frag.glsl; do
    FILE=$(basename $SHADER_SOURCE)
    NAME=${FILE%.*.*}

    glslc \
        -fshader-stage=fragment \
        $SHADER_SOURCE \
        -o $SHADERS/spv/$NAME.frag.spv
done
