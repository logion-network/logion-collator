#!/bin/bash

set -e

VERSION=v0.2.0

mkdir -p target/release/
cd target/release/
rm -f logion
wget https://github.com/logion-network/logion-collator/releases/download/${VERSION}/logion
chmod +x logion
