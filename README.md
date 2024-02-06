# Logion Collator Node

This project contains Logion's collator node.

Logion is now a parachain. However, it still relies on a production-ready (i.e. mainnet) Substrate-based
solochain (see runtime [here](https://github.com/logion-network/logion-node)). The purpose of
this project is to follow the evolution of Logion's parachain runtime in the process of
migrating from the solochain to the parachain.

## What to expect next?

Logion's [white paper](https://docs.logion.network/logion-white-paper/) describes all the aspects that
will be/are implemented by this runtime.

In particular, you will find there a descripiton of Logion's [tokenomics](https://docs.logion.network/logion-white-paper/tokenomics/introduction-to-logion-tokenomics)
and future [governance](https://docs.logion.network/logion-white-paper/governance/the-logion-governance-model-in-a-nutshell).

## Test locally

### Prerequisites

Install [Zombienet](https://github.com/paritytech/zombienet).

### Setup

1. If not already done, download polkadot binaries with command `./scripts/download_polkadot.sh`

2. If not already done, build logion collator with command `cargo build --release`

3. Run `$ZOMBIENET spawn local-zombienet.toml` where `$ZOMBIENET` is the path to Zombienet binary

## JSON chainspec generation

Below, `$CHAIN` is one of `logion`, `logion-dev`, `logion-test` or `local`. It is recommanded to define the variable before running the commands (`export CHAIN=...`).

1. Generate plain chainspec:

```
./target/release/logion build-spec --chain $CHAIN --disable-default-bootnode > ./res/$CHAIN.json
```

2. Generate raw chainspec

```
./target/release/logion build-spec --chain ./res/$CHAIN.json --raw --disable-default-bootnode > ./res/$CHAIN.raw.json
```

3. Generate WASM and genesis state

```
./target/release/logion export-genesis-wasm --chain ./res/$CHAIN.raw.json > ./bin/$CHAIN-wasm
```

```
./target/release/logion export-genesis-state --chain ./res/$CHAIN.raw.json > ./bin/$CHAIN-genesis
```

## Deploy an upgrade

- Build using [`srtool`](https://docs.substrate.io/reference/command-line-tools/srtool/)
  - `srtool build --root --package logion-runtime --runtime-dir runtime`
- `parachainSystem.authorizeUpgrade(codeHash, checkVersion)`
  - `codeHash`: `BLAKE2_256` field of compressed runtime in `srtool` build output
  - `checkVersion`: Yes
- `parachainSystem.enactAuthorizedUpgrade(code)`
  - `code`: the compressed runtime produced by `srtool`
