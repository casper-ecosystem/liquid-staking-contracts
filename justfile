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

cli-repl:
    cargo run --bin liquid-staking-cli -- repl

run-nctl:
    docker run --rm -it --cpus=1 --name mynctl -d -p 11101:11101 -p 14101:14101 -p 18101:18101 -p 25101:25101 makesoftware/casper-nctl:v203

cli-repl-on-nctl:
    set shell := bash
    mkdir -p .node-keys
    # Extract the secret keys from the local Casper node
    docker exec mynctl /bin/bash -c "cat /home/casper/casper-nctl/assets/net-1/users/user-1/secret_key.pem" > .node-keys/secret_key.pem
    # Run the command
    ODRA_CASPER_LIVENET_SECRET_KEY_PATH=.node-keys/secret_key.pem ODRA_CASPER_LIVENET_NODE_ADDRESS=http://localhost:11101 ODRA_CASPER_LIVENET_EVENTS_URL=http://localhost:18101/events ODRA_CASPER_LIVENET_CHAIN_NAME=casper-net-1 cargo run --bin liquid-staking-cli -- repl
    rm resources/casper-net-1-contracts.toml
    rm -rf .node-keys