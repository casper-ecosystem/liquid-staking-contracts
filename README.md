# Liquid Staking for Casper

It contains two projects:
- `liquid-staking-contracts` - this is where
  [StakedCSPR](liquid-staking-contracts/src/token.rs) contract is implemented.
- `liquid-staking-cli` - this is where the CLI tool is implemented.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-odra](https://github.com/odra-dev/cargo-odra) - with all its prerequisites
- [just](https://github.com/casey/just)

## Liquid staking contracts

This folder contains the implementation of the liquid staking contract. Detailed description of the contract can be found in the [README.md](liquid-staking-contracts/README.md) file. Tests are implemented in Gherkin format and can be found in the [features](liquid-staking-contracts/tests/features) folder.

### Testing
To run the tests suite, you can use the following command:

```bash
$ just test
```


## Liquid Staking CLI

```bash
$ just cli -h

Liquid Staking for CSPR. The CLI.

Usage: liquid-staking-cli [OPTIONS] <COMMAND>

Commands:
  deploy        Runs the deploy script
  contract      Commands for interacting with contracts
  scenario      Commands for interacting with scenarios
  print-events  Prints the most recent events emitted by a contract
  whoami        Prints the address of the current caller.
  help          Print this message or the help of the given subcommand(s)

Options:
  -c, --contracts-toml <PathBuf>  The path to the file with the deployed contracts. Relative to the project root.
  -h, --help                      Print help
```

This tool is used to interact with the contract on the Casper network. To deploy the contract, make sure it is built:

```bash
$ just build-contracts
```

Configure your keys in .env file. You can reuse defaults from .env.sample.


Then you can deploy the contract:

```bash
$ just cli deploy
```

The repository holds a [contracts.toml](resources/contracts.toml) file that contains the information about the latest contract deployed on the testnet, so you can skip the deployment step and use it directly.

To see all the entrypoints of the contract, you can use the following command:

```bash
$ just cli contract StakedCSPR help
```
Below are some examples of how to use the contract.

### Stake

You can stake CSPR using CLI.

```bash
$ just cli contract StakedCSPR stake --attached_value 1000000000

💁  INFO : Calling "hash-462aee162553159ae9b380cd5b7f915a2a0e11bdc0127792c3e6b8077b4e3a67" with entrypoint "stake" through proxy.
💁  INFO : Command executed successfully
```

### Balance Of

You can check balance of an account.

```bash
$ just cli contract StakedCSPR balance_of \
  --account account-hash-55c081be92939b87d1cac05916e9273056edaa67cf3cefb561157160082be739

💁  INFO : 1000000000
💁  INFO : Command executed successfully
```
Remember about `.env` file.


## Testnet Instance

Contract is deployed to the testnet. Contract's package hash is defined in
[contracts.toml](resources/contracts.toml) file.
