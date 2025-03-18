# Staked CSPR (sCSPR)

## Overview

The Staked CSPR (sCSPR) is a CEP-18 compliant token that represents staked CSPR on the Casper network. It allows users to maintain liquidity while earning staking rewards. When users stake their CSPR, they receive an equivalent amount of sCSPR tokens that can be freely transferred, traded, or used in DeFi applications.

## Key Features

- **Liquidity**: sCSPR tokens are transferable and can be used while your CSPR is staked
- **Staking Rewards**: sCSPR holders automatically receive staking rewards as the value of sCSPR increases relative to CSPR
- **Validator Diversification**: Staked CSPR is distributed across multiple validators for increased security
- **Unbonding Period**: When unstaking, users must wait through the standard unbonding period before claiming their CSPR

## How It Works

1. **Staking**: Users stake CSPR and receive sCSPR tokens in return
2. **Reward Accumulation**: As staking rewards accumulate, the exchange rate between sCSPR and CSPR improves
3. **Unstaking**: Users can burn their sCSPR tokens to initiate an unstaking process
4. **Claiming**: After the unbonding period, users can claim their original CSPR plus any earned rewards

## Contract Functions

### For Token Holders

- **`stake()`**: Stake CSPR and receive sCSPR tokens. The amount of sCSPR you receive is calculated based on the current exchange rate between CSPR and sCSPR.

- **`unstake(scspr_amount)`**: Burn your sCSPR tokens to initiate an unstaking process. This starts the unbonding period during which your CSPR remains locked. The amount of CSPR you'll receive after unbonding is calculated based on the current exchange rate.

- **`claim(unstake_id)`**: Claim your CSPR after the unbonding period is complete. Each unstaking operation is assigned a unique ID that you use to claim the corresponding CSPR. You cannot claim before the unbonding period ends (default 8 eras).

- **Standard CEP-18 functions**: Use `transfer()`, `balance_of()`, etc. to manage your sCSPR tokens just like any other token on Casper. This makes the token fully compatible with the Casper ecosystem (like wallets, explorers, etc.).

### For Contract Administrators

- **`add_validator(public_key)`**: Add a new validator to the validator set. Once added, a portion of new stakes will be delegated to this validator based on the contract's delegation strategy. This helps diversify the staked CSPR across multiple validators.

- **`remove_validator(public_key)`**: Remove a validator from the validator set. When a validator is removed, all delegated funds to that validator are automatically undelegated and redistributed among the validators in the set. This process does not affect the total amount of staked CSPR or the value of sCSPR tokens.

- **`add_to_the_pool()`**: Add CSPR to the staking pool without minting sCSPR. This is typically used by the contract administrator to increase the pool's capital efficiency or to add rewards to the system.

- **`remove_from_the_pool(amount)`**: Remove CSPR from the staking pool.

- **`withdraw_from_the_pool(amount)`**: Withdraw CSPR from the contract. This allows the administrator to withdraw fees or manage contract-owned CSPR that isn't part of the staking pool.

- **`restake_loose_tokens()`**: Restake any loose tokens in the contract. This ensures that all rewards and available CSPR are put to work earning staking rewards for the benefit of all sCSPR holders.

## Fee Structure

The contract charges a small fee on the staking rewards, which is customizable during contract deployment. The fee is collected in sCSPR tokens and sent to the contract administrator. For example, with a 10% fee, if the network generates 100 CSPR in rewards, 10 CSPR worth of sCSPR would go to the administrator, and 90 CSPR worth of value would be distributed to all sCSPR holders through the appreciation of the token's value.

## Technical Specifications

- Token Name: Staked CSPR
- Token Symbol: sCSPR
- Decimals: 9
- Implementation: CEP-18 standard with additional staking functionality
- Unbonding Period: Configurable (default is 8 eras on Casper, approximately 8 days)

## Technical Details

### Exchange Rate Mechanism

The sCSPR token uses a dynamic exchange rate that automatically adjusts as staking rewards accrue. This is how the exchange rate calculations work:

1. **Initial Exchange Rate**: When the contract is first deployed, 1 sCSPR = 1 CSPR.

2. **Dynamic Exchange Rate**: As staking rewards accumulate in the pool, the value of each sCSPR token increases relative to CSPR. The exchange rate is calculated as:
   ```
   exchange_rate = total_staked_cspr / total_scspr_supply
   ```

3. **Staking Calculation**: When a user stakes CSPR, the amount of sCSPR they receive is calculated using:
   ```
   scspr_amount = cspr_amount * total_scspr_supply / total_staked_cspr
   ```
   If there are no tokens in circulation yet, the user receives 1 sCSPR for each CSPR staked.

4. **Unstaking Calculation**: When a user unstakes sCSPR, the amount of CSPR they'll receive after the unbonding period is calculated using:
   ```
   cspr_amount = scspr_amount * total_staked_cspr / total_scspr_supply
   ```
### Staking Process

When a user stakes CSPR through the `stake()` function:

1. The contract first collects any pending fees (details below).
2. It calculates the amount of sCSPR to mint based on the current exchange rate.
3. It delegates the CSPR to one validator in the validator set using a pseudorandom selection mechanism.
4. It mints the calculated amount of sCSPR tokens to the user.
5. It emits a `Staked` event with details of the transaction.
6. The contract updates its record of the total delegated amount.

### Unstaking Process

The unstaking process occurs in two separate steps:

1. **Initiating the Unstake** (`unstake()` function):
   - The contract collects any pending fees.
   - It calculates the amount of CSPR to return based on the current exchange rate.
   - It undelegates the CSPR from one or more validators, starting from a random validator and proceeding until the full amount is undelegated.
   - The sCSPR tokens are burned.
   - The contract creates an unstaking record with:
     - A unique unstake ID
     - The owner's address
     - The CSPR amount to return
     - The timestamp when the funds become claimable (current time + unbonding period)
     - A "claimed" flag (initially set to false)
   - The contract emits an `Unstaked` event.
   - The function returns the unstake ID that the user will need for claiming.

2. **Claiming Unstaked CSPR** (`claim()` function):
   - The user calls the function with their unstake ID.
   - The contract verifies:
     - The unstake record exists
     - The unbonding period has passed
     - The funds haven't already been claimed
     - The caller is the owner of the unstake record
   - If all checks pass, the contract transfers the CSPR to the user.
   - The unstake record is marked as claimed.
   - The contract emits a `Claimed` event.

### Validator Management

The contract supports delegation to multiple validators for better security and decentralization:

1. **Adding Validators** (`add_validator()` function):
   - Only the contract owner can add validators.
   - If the validator already exists in the set, no changes are made.
   - Otherwise, the validator is added to the set.

2. **Removing Validators** (`remove_validator()` function):
   - Only the contract owner can remove validators.
   - When a validator is removed:
     - First, the validator is removed from the validator set.
     - Then, all CSPR delegated to that validator is automatically undelegated.
     - These funds are then restaked to other validators in the set.
   - This process is transparent to users and doesn't affect the value of their sCSPR tokens.

3. **Delegation Strategy**:
   - When users stake CSPR, the contract uses a deterministic pseudorandom algorithm to select validators for delegation.
   - The selection algorithm uses the block timestamp as a seed for randomness.
   - This approach helps distribute stake evenly across validators while maintaining unpredictability.

4. **Undelegation Strategy**:
   - When undelegating (during unstaking or validator removal), the contract tries to maintain balance.
   - It starts from a randomly selected validator and undelegates in sequence until the required amount is reached.
   - If a single validator doesn't have enough staked CSPR, the contract undelegates from multiple validators.

5. **Restaking Strategy**:
   - When restaking, the contract selects multiple validators (up to 3) using the pseudorandom selection algorithm.
   - It divides the loose tokens equally among the selected validators and delegates them.

### Fee Collection

The contract charges a fee on the rewards earned through staking:

1. **Fee Calculation Logic** (`collect_fee()` function):
   - The fee is collected automatically when users stake, unstake, or when the owner adds to the pool.
   - The contract tracks the last recorded total staked CSPR amount.
   - When the current staked amount exceeds the last recorded amount (indicating rewards were earned), the contract:
     - Calculates the reward as the difference between current and last recorded amounts
     - Applies the fee percentage (set during contract initialization) to the reward
     - Converts the fee from CSPR to sCSPR tokens based on the current exchange rate
     - Mints the calculated amount of sCSPR directly to the contract owner

2. **Fee Percentage**:
   - The fee percentage is set when the contract is deployed and cannot be changed afterward.
   - It's expressed in basis points (1/100th of a percent), where 10000 = 100%.
   - For example, a fee_percentage of 1000 means a 10% fee.

3. **Example**:
   - If the last recorded delegated amount was 1000 CSPR and is now 1100 CSPR (100 CSPR in rewards)
   - With a 10% fee (1000 basis points)
   - The fee would be 10 CSPR worth of sCSPR (100 * 10% = 10)
   - If there are 900 sCSPR in circulation against 1100 CSPR staked
   - The fee in sCSPR would be approximately 8.18 sCSPR (10 * 900 / 1100)

### Restaking Process

The contract includes a mechanism to maximize yield by ensuring all available tokens are staked:

1. **Loose Tokens**: These are tokens in the contract that aren't part of an unstaking process and aren't currently delegated.

2. **Restaking** (`restake_loose_tokens()` function):
   - The function identifies loose tokens in the contract.
   - It selects multiple validators (up to 3) using the pseudorandom selection algorithm.
   - It divides the loose tokens equally among the selected validators and delegates them.
   - This ensures all available CSPR is earning staking rewards, maximizing returns for all sCSPR holders.

This restaking functionality is particularly useful after the removal of a validator from the validator set. After the unbonding period, the tokens can be restaked to other validators in the set.

