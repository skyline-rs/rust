#!/bin/bash
python3 x.py build --stage 2 \
    --target aarch64-skyline-switch \
    --target x86_64-pc-windows-msvc \
    library/std src/tools/clippy src/tools/rustdoc
