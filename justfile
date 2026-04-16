BINARYEN_VERSION := "version_125"
BINARYEN_CHECKSUM := "7c3bc16599c8274a04d34a504fe4be2047884f900e0e2da2f6fb9cd667183be4"

prepare:
    wget https://github.com/WebAssembly/binaryen/releases/download/{{BINARYEN_VERSION}}/binaryen-{{BINARYEN_VERSION}}-x86_64-linux.tar.gz || { echo "Download failed"; exit 1; }
    sha256sum binaryen-{{BINARYEN_VERSION}}-x86_64-linux.tar.gz | grep {{BINARYEN_CHECKSUM}} || { echo "Checksum verification failed"; exit 1; }
    tar -xzf binaryen-{{BINARYEN_VERSION}}-x86_64-linux.tar.gz || { echo "Extraction failed"; exit 1; }
    sudo cp binaryen-{{BINARYEN_VERSION}}/bin/wasm-opt /usr/local/bin/wasm-opt

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
