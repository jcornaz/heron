#!/bin/sh

cd core && cargo publish --allow-dirty --no-verify && cd ..
cd macros && cargo publish --allow-dirty --no-verify && cd ..

sleep 30

cd rapier && cargo publish --allow-dirty --no-verify && cd ..

sleep 30

cd debug && cargo publish --allow-dirty --no-verify  && cd ..

sleep 30

cargo publish --allow-dirty --no-verify
