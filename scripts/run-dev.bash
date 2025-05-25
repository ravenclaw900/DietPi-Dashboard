#!/bin/bash -ex

./scripts/build-assets.bash

cargo build --bins

./target/debug/server &
sleep 0.1
./target/debug/backend &

trap 'kill $(jobs -pr)' EXIT
wait
