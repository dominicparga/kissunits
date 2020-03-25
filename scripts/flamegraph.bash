#!/usr/bin/env bash

set -e

# build
CUR_DIR="$(dirname "$(pwd)${0:1}")"
"${CUR_DIR}/build.sh"

# cargo install flamegraph, see https://github.com/killercup/cargo-flamegraph
cargo flamegraph --bin my_binary --
