# StakedCSPR JavaScript client

This document outlines how a client application can interact with the **StakedCSPR** smart contract from a user perspective, enabling staking and unstaking of the native **CSPR** token.

## Overview

The StakedCSPR contract provides three primary methods that a client application can invoke. The `stake` method allows users to lock up their native CSPR tokens in exchange for an equivalent value in the fungible `sCSPR` token, determined by the current exchange rate. The `unstake` method begins the process of converting `sCSPR` back into native CSPR; however, the resulting CSPR becomes available for withdrawal only after a 7-era delay. Finally, the `claim` method is used to complete the withdrawal by transferring any CSPR that has completed the unbonding period into the user's account.

## Staking CSPR

To initiate staking, a user must send a minimum of 500 CSPR to the StakedCSPR contract. The number of `sCSPR` tokens minted and sent to the user's account is determined by the current CSPR/sCSPR rate, which is calculated by dividing the total supply of `sCSPR` by the total amount of CSPR staked. This rate is updated every two hours, coinciding with the arrival of rewards from the Casper auction contract. It is important to note that staking or unstaking actions do not affect this rate directly.

Because the contract may distribute staked CSPR across multiple validators, calculating the total stake requires aggregating the delegations to all of them. The total `sCSPR` supply is available as part of the contract's on-chain state.

## Unstaking CSPR

There is no minimum amount required to initiate an unstake. A user can send any amount of `sCSPR` to the contract, which will then burn the tokens and begin the process of unstaking the equivalent amount of native CSPR from one of the validators managed by the contract. The Casper network enforces a mandatory waiting period of seven eras for unstaking operations. As such, the user must wait this duration before being able to retrieve their tokens.

## Withdrawing Unstaked CSPR

Once the unbonding period has elapsed, the user must explicitly call the `claim` method to complete the withdrawal. This action will transfer to the user’s account all eligible CSPR that was previously unstaked and is now available.

## StakedCSPR JavaScript Client

All the scripts can print help instructions using the `--help` argument:

```bash
npm run scripts:install_staking -- --help
```

### Contract Installation

To deploy or install the StakedCSPR contract, use the `install_staking` script:

```bash
npm run scripts:install_staking -- \
  --node_url https://node.testnet.casper.network/rpc \
  --network_name casper-test \
  --wasm ../wasm/StakedCSPR.wwasm \
  --owner_keys_path ./ls-owner.pem \
  --keys_algo secp256k1 \
  --validator <validator-public-key>
```

### Stake CSPR

To stake CSPR, use the `stake` script:

```bash
npm run scripts:stake -- \
  --node_url https://node.testnet.casper.network/rpc \
  --network_name casper-test \
  --owner_keys_path ./ls-owner.pem \
  --keys_algo secp256k1 \
  --proxy_caller ../wasm/proxy_caller.wasm \
  --contract_package_hash <stakedcspr_contract_package_hash> \
  --amount 500000000000
```

The execution of the stake transaction results in some contract events emitted. To parse the events, use the `get_transaction_events` script:

```bash
npm run scripts:get_transaction_events -- \
    --node_url https://node.testnet.casper.network/rpc \
    --contract_hash <contract_hash> \
    --transaction_hash <transaction_hash>
```

The `Staked` event contains the sCPSR amount minted and trasferred to the user.
```
Event:
  Name: Mint
  Contract Hash: 3e945c35a602d049a3ec5960dd729ed7fb338c9c3b0d2e375313825b749d0042
  Contract Package Hash: 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3
  Event ID: 256
  Data:
    recipient: account-hash-1ef371ec8f3626883a90a68c400672df0e988c0f76e7fd28beedfdb283a05bd4
    amount: 1407439892
Event:
  Name: Delegated
  Contract Hash: 3e945c35a602d049a3ec5960dd729ed7fb338c9c3b0d2e375313825b749d0042
  Contract Package Hash: 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3
  Event ID: 257
  Data:
    address: account-hash-76d080b4e769f0b29c77fc6472d6e425710840c2f46a4506e5544d2ce34f43a3
    amount: 500000000000
    validator: 0106ca7c39cd272dbf21a86eeb3b36b7c26e2e9b94af64292419f7862936bca2ca
Event:
  Name: Mint
  Contract Hash: 3e945c35a602d049a3ec5960dd729ed7fb338c9c3b0d2e375313825b749d0042
  Contract Package Hash: 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3
  Event ID: 258
  Data:
    recipient: account-hash-76d080b4e769f0b29c77fc6472d6e425710840c2f46a4506e5544d2ce34f43a3
    amount: 956417120950
Event:
  Name: Staked
  Contract Hash: 3e945c35a602d049a3ec5960dd729ed7fb338c9c3b0d2e375313825b749d0042
  Contract Package Hash: 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3
  Event ID: 259
  Data:
    address: account-hash-76d080b4e769f0b29c77fc6472d6e425710840c2f46a4506e5544d2ce34f43a3
    cspr_amount: 500000000000
    scspr_minted: 956417120950
```

### Unstake CSPR

To unstake CSPR, use the `unstake` script:

```bash
npm run scripts:unstake -- \
  --node_url https://node.testnet.casper.network/rpc \
  --network_name casper-test \
  --owner_keys_path ./ls-owner.pem \
  --keys_algo secp256k1 \
  --contract_package_hash <stakedcspr_contract_package_hash> \
  --amount 3000000000
```

Again, the client can find important result information parsing the events emitted. In this case, the `Unstaked` event contains the CSPR amount the user will get, and the claim time.

```
Event:
  Name: Undelegated
  Contract Hash: 3e945c35a602d049a3ec5960dd729ed7fb338c9c3b0d2e375313825b749d0042
  Contract Package Hash: 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3
  Event ID: 260
  Data:
    address: account-hash-76d080b4e769f0b29c77fc6472d6e425710840c2f46a4506e5544d2ce34f43a3
    amount: 36
    validator: 0106ca7c39cd272dbf21a86eeb3b36b7c26e2e9b94af64292419f7862936bca2ca
Event:
  Name: Burn
  Contract Hash: 3e945c35a602d049a3ec5960dd729ed7fb338c9c3b0d2e375313825b749d0042
  Contract Package Hash: 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3
  Event ID: 261
  Data:
    owner: account-hash-76d080b4e769f0b29c77fc6472d6e425710840c2f46a4506e5544d2ce34f43a3
    amount: 70
Event:
  Name: Unstaked
  Contract Hash: 3e945c35a602d049a3ec5960dd729ed7fb338c9c3b0d2e375313825b749d0042
  Contract Package Hash: 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3
  Event ID: 262
  Data:
    address: account-hash-76d080b4e769f0b29c77fc6472d6e425710840c2f46a4506e5544d2ce34f43a3
    cspr_amount: 36
    scspr_burned: 70
    unstake_id: 41
    claim_time: 1749211477154
```

### Get Address Claims

To get the list of pending claims for an address, use the `get_address_claims` script:

```bash
npm run scripts:get_address_claims -- \
  --node_url https://node.testnet.casper.network/rpc \
  --contract_hash <contract_hash> \
  --account_hash <account_hash>
```

Example output:

```
unstake id 1
  owner 56befc13a6fd62e18f361700a5e08f966901c34df8041b36ec97d54d605c23de
  amount 93360834346
  claimTime 1750168532856
  isClaimed false
unstake id 2
  owner 56befc13a6fd62e18f361700a5e08f966901c34df8041b36ec97d54d605c23de
  amount 182873326928
  claimTime 1750168542439
  isClaimed false
```

### Claim CSPR

To withdraw the CSPR, use the `claim` script:

```bash
npm run scripts:claim -- \
  --node_url https://node.testnet.casper.network/rpc \
  --network_name casper-test \
  --owner_keys_path ./ls-owner.pem \
  --keys_algo secp256k1 \
  --contract_package_hash <stakedcspr_contract_package_hash>
```

The `Claimed` events indicate the amount of CSPR sent to the user's account.

```
Event:
  Name: Claimed
  Contract Hash: 3e945c35a602d049a3ec5960dd729ed7fb338c9c3b0d2e375313825b749d0042
  Contract Package Hash: 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3
  Event ID: 264
  Data:
    address: account-hash-76d080b4e769f0b29c77fc6472d6e425710840c2f46a4506e5544d2ce34f43a3
    cspr_amount: 1854833355
    unstake_id: 10
Event:
  Name: Claimed
  Contract Hash: 3e945c35a602d049a3ec5960dd729ed7fb338c9c3b0d2e375313825b749d0042
  Contract Package Hash: 8ef35d2b2d3f6c3dbdcab9b1d05f0a941b93bedb978272b5c4a11d23e4e8ffd3
  Event ID: 265
  Data:
    address: account-hash-76d080b4e769f0b29c77fc6472d6e425710840c2f46a4506e5544d2ce34f43a3
    cspr_amount: 167070817497
    unstake_id: 29
```

### Get sCSPR balance

To get the sCSPR balance for an account, use the `get_scspr_balance` script:

```bash
npm run scripts:get_scspr_balance -- \
  --node_url https://node.testnet.casper.network/rpc \
  --contract_hash <contract_hash> \
  --account_hash <account_hash>
```

Example output:

```
sCSPR balance: 123456789
```

## Recovering the sCSPR/CSPR rate

The CSPR/sCSPR rate, is calculated by dividing the total supply of `sCSPR` by the total amount of CSPR staked.

The `get_cspr_scpr_rate` script can be used to retrieve the rate:

```bash
npm run scripts:get_cspr_scpr_rate -- \
  --node_url https://node.testnet.casper.network/rpc \
  --contract_hash <stakedcspr_contract_hash>
```

Example output:

```
Contract main purse: uref-3d6f794d6da1e3d0a574753310e92544644658c6d1d33a5cee0c133811aedbb5-007
Validator: 01f58b94526d280881f79744effebc555426190950d5dfdd2f8aaf10ceaec010c6
  Bonding purse: https://testnet.cspr.live/uref/uref-0aa2459444f442a4341d7cbcbb6004d23a74a069c342a5ed63072d7da3ea24d0-007
  Staked amount: 3661538066380
Total CSPR staked: 3661538066380
Total sCSPR supply: 2237515546974
CSPR/sCSPR rate: 1.636430223393012

```

## Estimating the sCSPR/CSPR rate (manual alternative)

The CSPR/sCSPR rate, is calculated by dividing the total supply of `sCSPR` by the total amount of CSPR staked.

A possible method to get the total amount of CSPR staked is using CSPR.cloud Delegations API:

```bash
curl -X 'GET' \
  'https://api.testnet.cspr.cloud/urefs/uref-2765fdc748ee7ab6f0ce1c13fd97e54eeb6403159602c07e2f950c565c07cfd0-007/delegations?limit=2&offset=0' \
  -H 'Accept: application/json' \
  -H 'Authorization: 55f79117-fc4d-4d60-9956-65423f39a06a'
```

The response returns one or more delegated amoounts:

```
{
  "data": [
    {
      "bonding_purse": "uref-b6c8e1e97ae7783f0a65051d68cb61a24fcf90f08615cd78e0e5e0eb6e2192a6-007",
      "delegator_identifier": "uref-2765fdc748ee7ab6f0ce1c13fd97e54eeb6403159602c07e2f950c565c07cfd0-007",
      "delegator_identifier_type_id": 1,
      "stake": "6106911981606",
      "validator_public_key": "01075ec8809c691a1d1b0250ba9ea75da5460e3df43c3172771c6975c989457159"
    },
    {
      "bonding_purse": "uref-66f39b862e1be323ed675ee3a4303e25d28263f246200689bb53a411e1a1d540-007",
      "delegator_identifier": "uref-2765fdc748ee7ab6f0ce1c13fd97e54eeb6403159602c07e2f950c565c07cfd0-007",
      "delegator_identifier_type_id": 1,
      "stake": "499999999964",
      "validator_public_key": "0106ca7c39cd272dbf21a86eeb3b36b7c26e2e9b94af64292419f7862936bca2ca"
    }
  ]
}
```

To get the sCSPR total supply we can use the RPC interface:

```bash
curl -X POST https://node.testnet.casper.network/rpc \
  -H 'Content-Type: application/json' \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "query_global_state",
    "params": {
      "key": "uref-c9108ec62f0ee8db572af71f6529c95d1f444fe4b8bc6ec549847a44f5af039d-007",
      "path": []
    }
  }'
```

The response contains the total supply value:

```
{
  "api_version":"2.0.0",
  "block_header":null,
  "stored_value":{
    "CLValue":{
      "cl_type":"U256",
      "bytes":"06d2792d7f7e0b",
      "parsed":"12637927471570"
    }
  }
}
```

Applying the formula:

```
12637927471570 sCSPR / (6106911981606 + 499999999964) CSPR = 1,9128342419 sCSPR/CSPR
```

We can see that's the same rate used in the stake operation above where 500 CSPR where converted into 956,417120950:

```
956417120950 / 500000000000 = 1,9128342419
```
