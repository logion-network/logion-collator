# Logion Collator Node

This project contains Logion's collator node and runtime.

## Test locally

### Prerequisites

Install [Zombienet](https://github.com/paritytech/zombienet).

### Setup

1. If not already done, download polkadot binaries with command `./scripts/download_polkadot.sh`

2. If not already done, build logion collator with command `cargo build --release`
   or download a pre-compiled binary with `./scripts/download_logion.sh`

3. Run `$ZOMBIENET spawn local-zombienet.toml` where `$ZOMBIENET` is the path to the Zombienet binary

## JSON chainspec generation

Below, `$CHAIN` is one of `logion`, `logion-dev`, `logion-test` or `local`. It is recommended to define the variable before running the commands (`export CHAIN=...`).

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
  - `cp rust-toolchain.toml runtime/ ; srtool build --root --package logion-runtime --runtime-dir runtime ; rm runtime/rust-toolchain.toml`
- `parachainSystem.authorizeUpgrade(codeHash, checkVersion)`
  - `codeHash`: `BLAKE2_256` field of compressed runtime in `srtool` build output
  - `checkVersion`: Yes
- `parachainSystem.enactAuthorizedUpgrade(code)`
  - `code`: the compressed runtime produced by `srtool`

## Try Runtime

`try-runtime` tool enables the testing of a new runtime against real data.

### Test a runtime upgrade

Generally, what's tested here is one or several storage migrations activated by the new runtime or any Polkadot upgrade.

If not yet done, the [Substrate Try Runtime CLI](https://github.com/paritytech/try-runtime-cli) must be installed:

```sh
cargo install --git https://github.com/paritytech/try-runtime-cli --locked
```

If not yet done, the runtime has to be built with the `try-runtime` feature:

```sh
cargo build --release --features=try-runtime
```

It can then be tested by executing the following command:

```sh
try-runtime --runtime target/release/wbuild/logion-runtime/logion_runtime.compact.compressed.wasm on-runtime-upgrade live --uri wss://para-rpc01.logion.network:443
```

This will:
- connect to RPC node
- download current state
- execute the upgrade
- run pallets' `post_upgrade` hook
