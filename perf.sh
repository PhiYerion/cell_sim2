#!/bin/bash

cargo build
./target/debug/game &
pid=$!
perf record -gbz -p "$pid" --call-graph fp &
sleep 60
kill "$pid"
sleep 1
~/.cargo/bin/flamegraph --perfdata perf.data --flamechart
google-chrome-stable flamegraph.svg
