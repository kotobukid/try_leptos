#!/bin/sh

(cd front && trunk build --dist ../dist --release --minify --config front/Trunk.toml)
