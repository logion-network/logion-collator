# Logion Collator Node

This project contains Logion's collator node.

Logion is not yet a parachain. It relies on a production-ready (i.e. mainnet) Substrate-based
solochain (see runtime [here](https://github.com/logion-network/logion-node)). The purpose of
this project is to follow the evolution of Logion's parachain runtime in the process of
migrating from the solochain to the parachain. It also enabled to build the runtime
and genesis state for the crowdloan.

[Logion collator node prototype](https://github.com/logion-network/logion-collator-prototype) is aligned,
in terms of features, with the solochain runtime and is a better representation of what this runtime
will look like once the parachain becomes production-ready.

## What to expect next?

Logion's [white paper](https://docs.logion.network/logion-white-paper/) describes all the aspects that
will be/are implemented by this runtime.

In particular, you will find there a descripiton of Logion's [tokenomics](https://docs.logion.network/logion-white-paper/tokenomics/introduction-to-logion-tokenomics)
and future [governance](https://docs.logion.network/logion-white-paper/governance/the-logion-governance-model-in-a-nutshell).

## Test locally

Below steps describe the "quick and dirty" way to run your collator node locally and, as a result, be able to test your developments
or play with the network. It does not describe the "production way" of registering a (logion) parachain.

### Prerequisites

Your environment must be ready for Substrate development, see
[here](https://docs.substrate.io/tutorials/v3/create-your-first-substrate-chain/#install-rust-and-the-rust-toolchain)
for a step-by-step guide.

### Setup

Below steps show how to instantiate a local logion parachain and its relay chain. If you already followed those steps
and did not clean-up the data, you can just start the nodes (steps 3, 4 and 10).

1. If not already done, download polkadot binaries with command `./scripts/download_polkadot.sh`

2. If relevant (e.g. after an upgrade), regenerate relay chainspec

```
./bin/polkadot build-spec --chain rococo-local --disable-default-bootnode > ./res/local-chainspec.json
./bin/polkadot build-spec --chain ./res/local-chainspec.json --raw --disable-default-bootnode > ./res/local-chainspec.raw.json
```

3. If not already done, build logion collator with command `cargo build --release`

4. Run validator alice with command `./scripts/run_validator.sh alice`

5. Run validator bob with command `./scripts/run_validator.sh bob`

6. Reserve para ID

- With [Polkadot.js](https://polkadot.js.org/apps), connect to the local relay chain (`ws://localhost:9944`)
- Go to Network > Parachains > Parathreads
- Click on "+ ParaID" and, with Charlie, register para ID 2000

7. (optional if you did not change the runtime) Generate plain chainspec:

```
./target/release/logion build-spec --chain local --disable-default-bootnode > ./res/local.json
```

8. (optional if you did not change the runtime) Generate raw chainspec

```
./target/release/logion build-spec --chain ./res/local.json --raw --disable-default-bootnode > ./res/local.raw.json
```

9. Generate WASM and genesis state

```
./target/release/logion export-genesis-wasm --chain ./res/local.raw.json > ./bin/local-wasm
./target/release/logion export-genesis-state --chain ./res/local.raw.json > ./bin/local-genesis
```

10. Register parachain

- With [Polkadot.js](https://polkadot.js.org/apps), connect to the local relay chain (`ws://localhost:9944`)
- Go to Developer > Sudo
- Select extrinsic `paraSudoWrapper.sudoScheduleParaInitialize` and set the following parameters:
    - id: 2000
    - genesisHead: set file `./bin/local-genesis` generated above
    - validationCode: set file `./bin/local-wasm` generated above
    - parachain: Yes
- Submit the extrinsic

11. Wait for the parathread to be onboard (Network > Parachains > Parathreads)

12. Run collator with command `./scripts/run_collator.sh`

13. Wait for the collator to start producing blocks (spy the parachain's best and finalized block in the logs
or via Polkadot.js's dashboard: Network > Parachains).

14. You may start interacting with the logion parachain using Polkadot.js and connecting to `ws://localhost:8844`.

### Clean-up

Once the two validators and the collator are stopped and you would like to wipe all previously created state,
you can run the following command:

```
rm -rf /tmp/relay /tmp/parachain/
```

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
