default:
    just -l

lint:
    cargo fmt

build-contracts:
    cargo odra build

test:
    ODRA_MODULE=StakedCSPR cargo test --test test_staking

cli *ARGS:
    cargo run --bin liquid-staking-cli -- {{ARGS}}
