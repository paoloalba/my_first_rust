#!/bin/bash

export PATH="$HOME/.cargo/bin:$PATH"

maturin build -b pyo3

pip install ./target/wheels/rust_app-0.1.0-cp310-cp310-manylinux_2_28_x86_64.whl
