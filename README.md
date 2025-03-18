# Liquid Staking for Casper

It contains two projects:
- `liquid-staking-contracts` - this is where
  [StakedCSPR](liquid-staking-contracts/src/token.rs) contract is implemented.
- `liquid-staking-cli` - this is where the CLI tool is implemented.

## Testing
To run the tests suite, you can use the following command:

```bash
$ just test
```

## Liquid Staking CLI

```bash
$ just cli

Liquid Staking for CSPR. The CLI.

Usage: liquid-staking-cli <COMMAND>

Commands:
  deploy    Runs the deploy script
  contract  Commands for interacting with contracts
  scenario  Commands for running user-defined scenarios
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Stake

You can stake CSPR using CLI.

```bash
$ just cli contract StakedCSPR stake --__attached_value 1000000000

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
[deployed_contracts.toml](resources/deployed_contracts.toml) file.
