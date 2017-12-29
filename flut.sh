#!/bin/bash

git fetch origin
git reset --hard origin
#git checkout -- .
#git pull

source params.sh

rm ./temp
cargo run --bin pixelflood_gen --release "$image" "$xoffset" "$yoffset" "$iterations"

echo 'starting the flood'
cargo run --bin main --release "$threads" "$ip" "$port"

