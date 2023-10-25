# <h1 align="center"> Silius </h1>

![CI workflow](https://github.com/vid201/silius/actions/workflows/ci.yml/badge.svg)
![Docker workflow](https://github.com/vid201/silius/actions/workflows/publish_image.yml/badge.svg)
[![Telegram Group](https://img.shields.io/endpoint?color=neon&style=flat-square&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2F%2BsKeRcN4j3MM3NmNk)](https://t.me/+sKeRcN4j3MM3NmNk)
[![GitHub stars](https://img.shields.io/github/stars/vid201/silius.svg?style=social&label=Star&maxAge=2592000)](https://github.com/vid201/silius/stargazers/)
[![GitHub forks](https://img.shields.io/github/forks/vid201/silius.svg?style=social&label=Fork&maxAge=2592000)](https://github.com/vid201/silius/network/)

<p align="center">Silius - <a href="https://eips.ethereum.org/EIPS/eip-4337">ERC-4337 (Account Abstraction)</a> bundler implementation in Rust.</p>

<p align="center">
    <img src="./docs/images/banner.png" width="450">
</p>

For more information: https://hackmd.io/@Vid201/aa-bundler-rust

<i>This project is still under active development.</i>

## Getting started

### Native

<b>Prerequisites:</b>

Rust version: 1.71.1

1. `libclang-dev`, `pkg-config` and `libssl-dev` on Debian/Ubuntu.
2. Ethereum execution client JSON-RPC API with enabled [`debug_traceCall`](https://geth.ethereum.org/docs/interacting-with-geth/rpc/ns-debug#debug_tracecall). For production, you can use [Geth](https://github.com/ethereum/go-ethereum) or [Erigon](https://github.com/ledgerwatch/erigon). For testing, we are using Geth dev mode (tested with [v1.12.0](https://github.com/ethereum/go-ethereum/releases/tag/v1.12.0)); so you need to install [Geth](https://geth.ethereum.org/docs/getting-started/installing-geth) for running tests.
3. [`solc`](https://docs.soliditylang.org/en/v0.8.17/installing-solidity.html) >=0.8.12.
4. [`cargo-sort`](https://crates.io/crates/cargo-sort) and [`cargo-udeps`](https://crates.io/crates/cargo-udeps).

Set up third-party dependencies (ERC-4337 smart contracts and bundler tests):

```bash
make fetch-thirdparty
make setup-thirdparty
```

Create wallet for bundler:

```bash
cargo run --release -- create-wallet --output-path ${HOME}/.silius --chain-id 5
```

Run bundler (with user operation pool and JSON-RPC API):

```bash
cargo run --release -- bundler --eth-client-address http://127.0.0.1:8545 --mnemonic-file ${HOME}/.silius/0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --beneficiary 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --entry-points 0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789 --http --ws
```

Run only bundling component:

```bash
cargo run --release -- bundling --eth-client-address ws://127.0.0.1:8546 --mnemonic-file ${HOME}/.silius/0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --beneficiary 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --entry-points 0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789
```

Run only user operation pool:

```bash
cargo run --release -- uopool --eth-client-address ws://127.0.0.1:8546 --entry-points 0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789
```

Run only JSON-RPC API:

```bash
cargo run --release -- rpc --http --ws
```

### Docker

```bash
docker run --net=host -v ./bundler-spec-tests/keys/0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266:/data/silius/0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 -v ./db:/data/silius/db ghcr.io/vid201/silius:latest bundler --eth-client-address http://127.0.0.1:8545 --datadir data/silius --mnemonic-file data/silius/0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --beneficiary 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 --entry-points 0x5ff137d4b0fdcd49dca30c7cf57e578a026d2789 --http --http.addr 0.0.0.0 --http.port 3000 --http.api eth,debug,web3 --ws --ws.addr 0.0.0.0 --ws.port 3001 --ws.api eth,debug,web3 --eth-client-proxy-address http://127.0.0.1:8545
```

### Deploying with `systemd`

For users looking to deploy Silius on their servers using `systemd`, we provide a systemd service file to make the deployment process easier. This is tested for Ubuntu machines and runs the release build of silius.

1. Copy the `silius.service` to `/lib/systemd/system/`.
2. Make sure to adjust the file paths and other configuration details in the service file to match your setup. There are two variables:
   1. `WORKING_DIR`: Point this to the directory where you have the `silius` project.
   2. `HOME_DIR`: Point this to the `HOME` directory (used for locating mnemonic file).
3. Start the service using `sudo systemctl start silius`.
4. Enable it to start on boot using `sudo systemctl enable silius`.
5. To view the status of the service, you can run ```sudo systemctl status silius```
6. To view silius logs, you can run ```tail -f /var/log/silius/silius.log -f```

For more details on systemd and how it works, consult the [official systemd documentation](https://www.freedesktop.org/wiki/Software/systemd/).


## Supported networks

Bundler was tested on the following networks:

| Chain         | Mainnet   | Testnet                                       |
| :--------:    | :-------: | :-------:                                     |
| Ethereum      | :soon:    | :soon: (Goerli), :heavy_check_mark: (Sepolia) |
| Polygon PoS   | :soon:    | :heavy_check_mark: (Mumbai)                   |

## Supported entry point
The address of the entry point smart contract is the same on all EVM networks.
| Address    | Version   | Commit    | Audited   |
| :--------: | :-------: | :-------: | :-------: |
| [0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789](https://blockscan.com/address/0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789) | 0.6.0 | [9b5f2e4](https://github.com/eth-infinitism/account-abstraction/commit/9b5f2e4bb30a81aa30761749d9e2e43fee64c768) | [April 2023](https://blog.openzeppelin.com/eip-4337-ethereum-account-abstraction-incremental-audit)

## Examples

To get started, check the examples [here](./examples/). More examples will be added in the future.

## Contributing

Thank you for showing interest in contributing to the project!

There is [a contributing guide](./CONTRIBUTING.md) to help get you started.

There are some additional prerequisites for **testing**:

1. [`geth`](https://geth.ethereum.org/docs/getting-started/installing-geth)

Before making a PR, make sure to run the following commands:

```bash
make format
make lint
make test
```

Official [bundler spec tests](https://github.com/eth-infinitism/bundler-spec-tests) developed by the [eth-infinitism](https://github.com/eth-infinitism/) team are also included in the repo's CI pipeline (commit: [bbd61f21e95ed1290678fcbfd9551b1502c81fe9](https://github.com/eth-infinitism/bundler-spec-tests/tree/bbd61f21e95ed1290678fcbfd9551b1502c81fe9)). You can find more information on how to run tests [here](https://github.com/eth-infinitism/bundler-spec-tests). Make sure your contribution doesn't break the tests!

## Contact

The best place for the discussion is the dedicated [Telegram group](https://t.me/+sKeRcN4j3MM3NmNk).

## Authors

- Vid Kersic: [GitHub](https://github.com/Vid201), [Twitter](https://twitter.com/vidkersic)
- WillQ: [GitHub](https://github.com/zsluedem), [Twitter](https://twitter.com/zsluedem06)

## Projects using Silius

- [Luban the Paymaster](https://github.com/da-bao-jian/luban-the-paymaster): A Cross-chain Tx Sponsorship Protocol.
- [Ethers UserOp](https://github.com/qi-protocol/ethers-userop/): An ether-rs middleware to craft UserOperations.

## Licenses

This project is dual-licensed under Apache 2.0 and MIT terms:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Donations

Silius is an open-source project and a public good. If you want to help the project, you can send donations of any size via:

- Ethereum address: `0x7cB801446AC4f5EA8f7333EFc58ab787eB611558`

## Acknowledgements

- [Bundler - eth-infinitism](https://github.com/eth-infinitism/bundler)
- [Akula](https://github.com/akula-bft/akula)
- [ethers-rs](https://github.com/gakonst/ethers-rs)
- [Reth](https://github.com/paradigmxyz/reth)
- [Lighthouse](https://github.com/sigp/lighthouse)
- [Alloy](https://github.com/alloy-rs)
