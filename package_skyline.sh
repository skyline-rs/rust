#!/bin/bash
python3 x.py dist --stage 2 --target aarch64-skyline-switch --target x86_64-unknown-linux-gnu library/std src/tools/cargo src/tools/clippy src/tools/rustdoc
