#!/bin/sh

cargo build --release && target/release/toytracer > result/current.ppm && \
    convert result/current.ppm result/current.png &&
    rm result/current.ppm
