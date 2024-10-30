default:
    just -l

lint:
    cargo fmt

build-contracts:
    cargo odra build

test:
    cargo odra test