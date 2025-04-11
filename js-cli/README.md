# StakedCSPR JavaScript client

### Install the contract

```
npm run scripts:install_staking -- \
  --wasm ../wasm/StakedCSPR.wasm \
  --owner_keys_path ./nctl-docker/users/user-1/secret_key.pem \
  --validator <validator-public-key>;
```

# Stake CSPR

```
npm run scripts:stake -- \
  --node_url https://node.testnet.casper.network/rpc \
  --network_name casper-test \
  --owner_keys_path ./account3_cw.pem \
  --keys_algo secp256k1 \
  --proxy_caller ../wasm/proxy_caller.wasm \
  --contract_package_hash 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3 \
  --amount 500000000000
```

### Unstake CSPR

```
npm run scripts:unstake -- \
  --node_url https://node.testnet.casper.network/rpc \
  --network_name casper-test \
  --owner_keys_path ./account3_cw.pem \
  --keys_algo secp256k1 \
  --contract_package_hash 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3 \
  --amount 3000000000
```

### Claim

```
npm run scripts:claim -- \
--node_url https://node.testnet.casper.network/rpc \
--network_name casper-test \
--owner_keys_path ./account3_cw.pem \
--keys_algo secp256k1 \
--contract_package_hash 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3
```

### Get Transaction Events

```
npm run scripts:get_transaction_events -- \
--node_url https://node.testnet.casper.network/rpc \
--contract_hash 3e945c35a602d049a3ec5960dd729ed7fb338c9c3b0d2e375313825b749d0042 \
--transaction_hash 0c75981065996aa697236b7fc018bfed41a3e20213f6b7848511082718ae56bc
```