# Logion Collator Node

This project contains Logion's collator node.

Logion is not yet a parachain. It relies on a production-ready (i.e. mainnet) Substrate-based
solochain (see runtime [here](https://github.com/logion-network/logion-node)). The purpose of
this project is to follow the evolution of Logion's parachain runtime in the process of
migrating from the solochain to the parachain. It also enabled to build the runtime
and genesis state for the crowd loan.

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

1. If not already done, build your relay chain node with command `./scripts/build_relay_chain.sh`

2. If not already done, build logion collator with command `cargo build --release`

3. Run validator alice with command `./scripts/run_validator.sh alice`

4. Run validator bob with command `./scripts/run_validator.sh bob`

5. Reserve para ID

- With [Polkadot.js](https://polkadot.js.org/apps), connect to the local relay chain (`ws://localhost:9944`)
- Go to Network > Parachains > Parathreads
- Click on "+ ParaID" and, with Charlie, register para ID 2000

6. (optional if you did not change the runtime) Generate plain chainspec:

```
./target/release/logion build-spec --disable-default-bootnode > ./res/rococo-local-logion-plain.json
```

7. (optional if you did not change the runtime) Generate raw chainspec

```
./target/release/logion build-spec --chain ./res/rococo-local-logion-plain.json --raw --disable-default-bootnode > ./res/rococo-local-logion-raw.json
```

8. Generate WASM and genesis state

```
./target/release/logion export-genesis-wasm --chain ./res/rococo-local-logion-raw.json > ./bin/local-logion-wasm
```

```
./target/release/logion export-genesis-state --chain ./res/rococo-local-logion-raw.json > ./bin/local-logion-genesis
```

9. Register parachain

- With [Polkadot.js](https://polkadot.js.org/apps), connect to the local relay chain (`ws://localhost:9944`)
- Go to Developer > Sudo
- Select extrinsic `paraSudoWrapper.sudoScheduleParaInitialize` and set the following parameters:
    - id: 2000
    - genesisHead: set file `./bin/local-logion-genesis` generated above
    - validationCode: set file `./bin/local-logion-wasm` generated above
    - parachain: Yes
- Submit the extrinsic

10. Run collator with command `./scripts/run_collator.sh`

11. Wait for the collator to start producing blocks (spy the parachain's best and finalized block in the logs
or via Polkadot.js's dashboard: Network > Parachains), this may take some time (around 3 minutes). Also, block production
may not be stable at the beginning. Again, waiting for a couple of minutes should be enough.

12. You may start interacting with the logion parachain using Polkadot.js and connecting to `ws://localhost:8844`.

### Clean-up

Once the two validators and the collator are stopped and you would like to wipe all previously created state,
you can run the following command:

```
rm -rf /tmp/relay /tmp/parachain/
```

## Mainnet JSON chainspec generation

1. Generate plain chainspec:

```
./target/release/logion build-spec --chain main --disable-default-bootnode > ./res/main-plain.json
```

2. Generate raw chainspec

```
./target/release/logion build-spec --chain ./res/main-plain.json --raw --disable-default-bootnode > ./res/main-raw.json
```

3. Generate WASM and genesis state

```
./target/release/logion export-genesis-wasm --chain ./res/main-raw.json > ./bin/main-wasm
```

```
./target/release/logion export-genesis-state --chain ./res/main-raw.json > ./bin/main-genesis
```
