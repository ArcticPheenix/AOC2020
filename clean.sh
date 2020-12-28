#!/bin/sh
for D in `find . -maxdepth 1 -type d -name "day*"`
do
    cd $D; cargo clean; cd ..
done