#!/bin/bash

SERVER=localhost:7878
JSON="json/tmp.json"
mkdir -p json

curl $SERVER/consume > "$JSON"
target/release/a_fractal_a_day --json $JSON -t
