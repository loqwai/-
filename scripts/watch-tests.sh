#!/bin/bash
cargo test
fswatch -x -l 2 -i="*.rs" -o src/ | xargs -n1 -I{} cargo test
