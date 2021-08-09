#!/bin/bash

# waiting for https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#configurable-env
EMSCRIPTEN="/emsdk/upstream/emscripten" EMMAKEN_CFLAGS="-s FETCH=1" cargo build

simple-http-server .