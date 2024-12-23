# StakedCSPR JavaScript client

NOTES:
1. This is a work in progress client.
2. `stake` and `unstake` entry points in the contract are 'dummy'. No real delegation/undelegation hapens under the hood.
3. Casper JS SDK v5.0.1-beta2 is required.
4. Casper 2.0 network is required for testing.
5. WASM links for testing:
 - https://drive.google.com/file/d/1hZk7tjB4uK-35CSiXxCJXNDeJexVp0BI/view?usp=sharing
 - https://drive.google.com/file/d/1prsJjEUmyNDHozeBvMAsjLmf-LMnVitM/view?usp=sharing
 
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
  --owner_keys_path ./nctl-docker/users/user-1/secret_key.pem \
  --proxy_caller ../wasm/proxy_caller_dummy.wasm \
  --contract_hash <StakedCSPR-contract-hash> \
  --amount 1000000000
```

### Unstake CSPR

```
npm run scripts:unstake -- \
  --owner_keys_path ./nctl-docker/users/user-1/secret_key.pem \
  --contract_hash <StakedCSPR-contract-hash> \
  --amount 1000000000
```