#!/bin/bash

set -e

VERSION=polkadot-v1.5.0

cd bin
wget https://github.com/paritytech/polkadot-sdk/releases/download/${VERSION}/polkadot
wget https://github.com/paritytech/polkadot-sdk/releases/download/${VERSION}/polkadot-execute-worker
wget https://github.com/paritytech/polkadot-sdk/releases/download/${VERSION}/polkadot-prepare-worker
chmod +x polkadot*
