default:
    just -l

lint:
    cargo fmt

build-contracts:
    cargo odra build

test:
    cargo odra test -b casper

cli *ARGS:
    cargo run --bin liquid-staking-cli -- {{ARGS}}
