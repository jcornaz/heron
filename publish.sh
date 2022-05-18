#!/bin/sh

cd core && cargo publish --allow-dirty && cd ..
cd macros && cargo publish --allow-dirty && cd ..

sleep 30

cd rapier && cargo publish --allow-dirty && cd ..

sleep 30

cd rapier && cargo publish --allow-dirty && cd ..

sleep 30

cargo publish --allow-dirty
