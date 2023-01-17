#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

# This script builds all subprojects and puts all created Wasm modules in one dir
cd config
cargo update --aggressive
marine build --release

cd ../game
cargo update --aggressive
marine build --release

cd ..
mkdir -p artifacts
rm -f artifacts/*.wasm

cp target/wasm32-wasi/release/config_file.wasm artifacts/
cp target/wasm32-wasi/release/game.wasm artifacts/
wget https://github.com/fluencelabs/sqlite/releases/download/v0.15.0_w/sqlite3.wasm
mv sqlite3.wasm artifacts/

marine aqua artifacts/game.wasm -s lefish_game -i lefish > aqua/lefish.aqua


RUST_LOG="info" mrepl --quiet Config.toml