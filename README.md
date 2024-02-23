### Development Testing

To test while developing, without a full build (thus reduce time to results):

```sh
cargo t -p pallet-dex
cargo t -p pallet-dpos
cargo t -p pallet-voting
cargo t -p pallet-free-tx
cargo t -p pallet-multisig
cargo t -p pallet-treasury
cargo t -p <other crates>
```

### Build

Build the node without launching it, with `release` optimizations:

```sh
cargo b -r
```

### Run

Build and launch the node, with `release` optimizations:

```sh
cargo r -r -- --dev
```

### CLI Docs

Once the project has been built, the following command can be used to explore all CLI arguments and subcommands:

```sh
./target/release/node-template -h
```
