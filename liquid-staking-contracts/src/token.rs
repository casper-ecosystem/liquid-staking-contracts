use crate::events::{
    Claimed, CsprAddedToPool, CsprRemovedFromPool, CsprWithdrawnFromContract, Delegated,
    FeeCollected, Staked, Undelegated, Unstaked, ValidatorAdded, ValidatorRemoved,
};
use crate::token::Error::*;
use odra::{
    casper_types::{PublicKey, U256, U512},
    prelude::*,
    uints::{ToU256, ToU512},
};
use odra_modules::access::Ownable2Step;
use odra_modules::{
    cep18::{errors::Error as Cep18Error, utils::Cep18Modality},
    cep18_token::Cep18,
};

pub const BENEFICIAL_VALIDATORS_COUNT: usize = 3;
pub const MAX_FEE_PERCENTAGE: u32 = 10000;

/// Error enum for the StakedCSPR contract
#[odra::odra_error]
pub enum Error {
    /// The unbonding delay has not passed yet
    NotYetClaimable = 61401,
    /// The unstake has already been claimed
    AlreadyClaimed = 61402,
    /// The user is not the owner of the unstake
    NotAnOwnerOfAClaim = 61403,
    /// The unstake was not found
    UnstakeNotFound = 61404,
    /// The user is not the owner of the contract
    NotAnOwner = 61405,
    /// The contract does not have enough balance
    InsufficientBalance = 61406,
    /// The validator is misconfigured
    MisconfiguredValidator = 61407,
    /// The stake is below the minimum
    StakeBelowMinimum = 61408,
    /// Total unstakes overflowed
    TotalUnstakesOverflow = 61409,
    /// The validator is not in the list of validators
    ValidatorNotInList = 61410,
    /// Arithmetics error
    ArithmeticsError = 61411,
    /// Action not allowed
    ActionNotAllowed = 61412,
    /// Fee percentage is above the maximum
    InvalidFeePercentage = 61413,
    /// The validator exists in the list of validators
    ValidatorAlreadyExists = 61414,
    /// The min stake is zero
    InvalidMinStake = 61415,
    /// No backing for redemption
    NoBackingForRedemption = 61416,
}

/// UnstakingInfo struct
/// It is used to store the unstaking information for each user
/// as the unstaking needs to wait for the unbonding delay before it can be claimed
#[odra::odra_type]
struct UnstakingInfo {
    /// The id of the unstake
    unstake_id: u32,
    /// The address of the owner of the unstake
    owner: Address,
    /// The amount of CSPR that was unstaked
    cspr_amount: U512,
    /// The time when the unstake will be claimable
    claimable_from: u64,
    /// Whether the unstake has been claimed
    claimed: bool,
}

/// StakedCSPR contract
#[odra::module(
    events = [Staked, Unstaked, Claimed, Delegated, Undelegated, ValidatorRemoved, ValidatorAdded, CsprAddedToPool, CsprRemovedFromPool, CsprWithdrawnFromContract, FeeCollected],
    errors = Error
)]
pub struct StakedCSPR {
    /// Ownable module
    ownable: SubModule<Ownable2Step>,
    /// Token module
    token: SubModule<Cep18>,
    /// Unstake ids for each user
    unstake_ids: Mapping<Address, Vec<u32>>,
    /// List of unstakes
    unstakes: List<UnstakingInfo>,
    /// Total unstakes
    total_unstakes: Var<U512>,
    /// List of validators
    validators: Var<Vec<PublicKey>>,
    /// Stored configuration of the time it takes for unstaked tokens to be claimable
    claim_time: Var<u64>,
    /// Delegated amount during the last change to the stake
    /// This is used to calculate the rewards granted and consequently, the fee
    last_recorded_delegated_amount: Var<U512>,
    /// Fee percentage to be charged for staking
    fee_percentage: Var<U512>,
    /// Minimum amount of CSPR that can be staked
    min_stake: Var<U512>,
    /// The amount of CSPR that was staked in removed validator
    removed_validator_stake: Var<U512>,
}

#[odra::module]
impl StakedCSPR {
    delegate! {
        to self.token {
            fn name(&self) -> String;
            fn symbol(&self) -> String;
            fn decimals(&self) -> u8;
            fn total_supply(&self) -> U256;
            fn balance_of(&self, account: &Address) -> U256;
            fn allowance(&self, owner: &Address, spender: &Address) -> U256;
            fn transfer(&mut self, recipient: &Address, amount: &U256);
            fn transfer_from(&mut self, owner: &Address, recipient: &Address, amount: &U256);
            fn approve(&mut self, spender: &Address, amount: &U256);
            fn decrease_allowance(&mut self, spender: &Address, decr_by: &U256);
            fn increase_allowance(&mut self, spender: &Address, inc_by: &U256);
        }

        to self.ownable {
            fn get_owner(&self) -> Address;
            fn transfer_ownership(&mut self, new_owner: &Address);
            fn accept_ownership(&mut self);
        }
    }

    pub fn init(
        &mut self,
        validator_address: PublicKey,
        claim_time: u64,
        fee_percentage: U512,
        min_stake: U512,
    ) {
        // Check if the fee percentage is above the maximum
        if fee_percentage > MAX_FEE_PERCENTAGE.into() {
            self.revert(InvalidFeePercentage);
        }

        // Check if the min stake is above the minimum
        if min_stake == U512::zero() {
            self.revert(InvalidMinStake);
        }

        let admin = self.env().caller();

        // Grant the admin role
        self.ownable.init(admin);

        // Initialize the validator address.
        self.validators.set(vec![validator_address]);
        // Initialize the claim time.
        self.claim_time.set(claim_time);
        // Initialize the token.
        self.token.init(
            String::from("sCSPR"),
            String::from("Staked CSPR"),
            9,
            U256::zero(),
            vec![],
            vec![],
            Some(Cep18Modality::None),
        );

        self.fee_percentage.set(fee_percentage);
        self.last_recorded_delegated_amount.set(U512::zero());
        self.min_stake.set(min_stake);
    }

    /// We override the default implementation of the change_security function
    /// to be compatible with the cep18 token, but we don't allow any changes
    #[allow(unused_variables)]
    pub fn change_security(
        &mut self,
        admin_list: Vec<Address>,
        minter_list: Vec<Address>,
        none_list: Vec<Address>,
    ) {
        self.revert(ActionNotAllowed);
    }

    /// We override the default implementation of the mint function
    /// to be compatible with the cep18 token, but we don't allow any minting
    #[allow(unused_variables)]
    pub fn mint(&mut self, owner: &Address, amount: &U256) {
        self.revert(ActionNotAllowed);
    }

    /// We override the default implementation of the burn function
    /// to be compatible with the cep18 token, but we don't allow any burning
    #[allow(unused_variables)]
    pub fn burn(&mut self, owner: &Address, amount: &U256) {
        self.revert(ActionNotAllowed);
    }

    /// Stakes CSPR
    /// This function is payable, the attached value is the amount of CSPR to stake
    #[odra(payable)]
    pub fn stake(&mut self) {
        let caller = self.env().caller();
        let cspr_amount = self.env().attached_value();

        let staked_cspr_before = self.staked_cspr() + self.removed_validator_stake.get_or_default();

        self.assert_min_stake(cspr_amount);
        self.collect_fee(staked_cspr_before);

        self.delegate(self.get_random_validators(1)[0].clone(), cspr_amount);

        let scspr_amount = self.cspr_to_scspr(cspr_amount, staked_cspr_before);

        self.token.raw_mint(&caller, &scspr_amount);

        self.env().emit_event(Staked {
            address: caller,
            cspr_amount,
            scspr_minted: scspr_amount,
        });

        let staked_cspr_after = staked_cspr_before + cspr_amount;
        self.last_recorded_delegated_amount.set(staked_cspr_after);
    }

    /// Unstakes sCSPR
    ///
    /// # Arguments
    ///
    /// * `scspr_amount` - The amount of sCSPR to unstake
    pub fn unstake(&mut self, scspr_amount: U256) {
        self.collect_fee(self.staked_cspr());
        let block_time = self.env().get_block_time();

        let caller = self.env().caller();

        if self.token.balance_of(&caller) < scspr_amount {
            self.env().revert(Cep18Error::InsufficientBalance);
        }

        let cspr_amount = self.scspr_to_cspr(scspr_amount);
        let actual_unstaked = self.undelegate_from_validators(cspr_amount);

        // If we couldn't undelegate the full amount, revert
        if actual_unstaked < cspr_amount {
            self.env().revert(InsufficientBalance);
        }

        self.token.raw_burn(&caller, &scspr_amount);

        // To keep track of the unstakes, we assign them an id
        let mut account_unstake_ids = self.unstake_ids.get_or_default(&caller);
        let new_unstake_id = self.unstakes.len();
        account_unstake_ids.push(new_unstake_id);

        let next_claim_time = self.next_claim_time(block_time);

        self.unstakes.push(UnstakingInfo {
            unstake_id: new_unstake_id,
            owner: caller,
            cspr_amount,
            claimable_from: next_claim_time.clone(),
            claimed: false,
        });

        self.unstake_ids.set(&caller, account_unstake_ids);

        self.total_unstakes.add(cspr_amount);

        self.env().emit_event(Unstaked {
            address: caller,
            cspr_amount,
            scspr_burned: scspr_amount,
            unstake_id: new_unstake_id,
            claim_time: next_claim_time,
        });

        self.last_recorded_delegated_amount.set(self.staked_cspr());
    }

    /// Claims unstaked CSPR
    /// It checks all claims that are claimable and claims them for the caller
    pub fn claim(&mut self) {
        let caller = self.env().caller();
        let mut unstake_ids = self.unstake_ids.get_or_default(&self.env().caller());
        let mut indices_to_remove = Vec::new();
        let mut transfers_to_make = Vec::new();

        // First, process all unstakes and track which indices need to be removed
        for (index, unstake_id) in unstake_ids.clone().iter().enumerate() {
            let mut unstake = self
                .unstakes
                .get(*unstake_id)
                .unwrap_or_revert_with(self, UnstakeNotFound);
            if unstake.claimable_from > self.env().get_block_time() || unstake.claimed {
                continue;
            }

            let cspr_amount = unstake.cspr_amount;
            let owner = unstake.owner;

            // First update all state
            unstake.claimed = true;
            self.unstakes.replace(*unstake_id, unstake);
            indices_to_remove.push(index);
            self.total_unstakes.set(
                self.total_unstakes
                    .get_or_default()
                    .checked_sub(cspr_amount)
                    .unwrap_or_revert_with(self, AlreadyClaimed),
            );

            // Store transfer for later
            transfers_to_make.push((owner, cspr_amount, *unstake_id));
        }

        // Remove the indices in reverse order to avoid shifting problems
        indices_to_remove.sort_unstable_by(|a, b| b.cmp(a));
        for index in indices_to_remove {
            unstake_ids.remove(index);
        }

        // Update storage
        self.unstake_ids.set(&caller, unstake_ids);

        // Now perform all transfers after storage has been updated
        for (recipient, amount, unstake_id) in transfers_to_make {
            self.env().transfer_tokens(&recipient, &amount);
            self.env().emit_event(Claimed {
                address: caller,
                cspr_amount: amount,
                unstake_id,
            });
        }
    }

    /// Returns the total amount of CSPR that is staked
    pub fn staked_cspr(&self) -> U512 {
        let validators = self
            .validators
            .get()
            .unwrap_or_revert_with(self, MisconfiguredValidator);
        if validators.is_empty() {
            self.env().revert(MisconfiguredValidator);
        }

        // Sum up delegations from all validators and add currently removed validator stake
        validators.iter().fold(U512::zero(), |acc, validator| {
            acc + self.env().delegated_amount(validator.clone())
        }) + self.removed_validator_stake.get_or_default()
    }

    /// Returns the minimum stake
    pub fn get_min_stake(&self) -> U512 {
        self.min_stake.get_or_default()
    }

    /// Sets the minimum stake
    pub fn set_min_stake(&mut self, min_stake: U512) {
        self.ownable.assert_owner(&self.env().caller());
        self.min_stake.set(min_stake);
    }

    /// Adds CSPR to the pool
    /// This function is payable, the attached value is the amount of CSPR to add to the pool
    /// No sCSPR is minted, only CSPR is added to the pool, which affect the price of sCSPR
    #[odra(payable)]
    pub fn add_to_the_pool_without_staking(&mut self) {
        let attached_value = self.env().attached_value();
        self.assert_min_stake(attached_value);
        let staked_cspr = self.staked_cspr();
        self.collect_fee(staked_cspr);

        // Get random validators and verify that we have at least one
        let validators = self.get_random_validators(1);
        if validators.is_empty() {
            self.env().revert(MisconfiguredValidator);
        }

        self.delegate(validators[0].clone(), self.env().attached_value());

        // Emit event for adding CSPR to the pool
        self.env().emit_event(CsprAddedToPool {
            amount: attached_value,
        });

        self.last_recorded_delegated_amount
            .set(staked_cspr + attached_value);
    }

    /// Removes CSPR from the pool
    pub fn remove_from_the_pool(&mut self, amount: U512) {
        self.ownable.assert_owner(&self.env().caller());
        let staked_cspr = self.staked_cspr();
        let actual_unstaked = self.undelegate_from_validators(amount);

        // If we couldn't undelegate the full amount, revert
        if actual_unstaked < amount {
            self.env().revert(InsufficientBalance);
        }

        self.collect_fee(staked_cspr);

        // Emit event for removing CSPR from the pool
        self.env().emit_event(CsprRemovedFromPool {
            amount: actual_unstaked,
        });

        self.last_recorded_delegated_amount
            .set(staked_cspr - amount);
    }

    /// Withdraws CSPR from the contract
    pub fn withdraw_from_the_contract(&mut self, amount: U512) {
        let caller = self.env().caller();
        self.ownable.assert_owner(&caller);

        // Only allow withdrawing loose tokens
        if amount > self.get_loose_tokens() {
            self.env().revert(InsufficientBalance);
        }

        self.env().transfer_tokens(&self.env().caller(), &amount);
        self.env().emit_event(CsprWithdrawnFromContract {
            amount,
            recipient: caller,
        });
    }

    /// Adds a validator to the list of validators
    pub fn add_validator(&mut self, public_key: PublicKey) {
        self.ownable.assert_owner(&self.env().caller());

        let mut validators = self.validators.get_or_default();

        // Check if the validator already exists in the list
        if !validators.contains(&public_key) {
            validators.push(public_key.clone());
            self.validators.set(validators);

            // Emit event for adding a validator
            self.env().emit_event(ValidatorAdded {
                validator: public_key,
            });
        } else {
            self.env().revert(ValidatorAlreadyExists);
        }
    }

    /// Removes a validator from the list of validators
    pub fn remove_validator(&mut self, public_key: PublicKey) {
        self.ownable.assert_owner(&self.env().caller());
        let staked_cspr = self.staked_cspr();
        self.collect_fee(staked_cspr);

        let mut validators = self.validators.get_or_default();
        let mut removed = false;
        let mut total_unstaked = U512::zero();

        // Find the position of the validator in the list
        if let Some(position) = validators.iter().position(|v| v == &public_key) {
            // Only remove if the validator exists
            validators.remove(position);
            self.validators.set(validators);

            // Emit event for removing a validator
            self.env().emit_event(ValidatorRemoved {
                validator: public_key.clone(),
            });

            // Unstake all the stake from the validator, but only if there is some stake
            let cspr_amount = self.env().delegated_amount(public_key.clone());
            if cspr_amount > U512::zero() {
                self.undelegate(public_key, cspr_amount);
                total_unstaked = total_unstaked
                    .checked_add(cspr_amount)
                    .unwrap_or_revert_with(self, TotalUnstakesOverflow);
            }
            removed = true;
        }

        // If validator doesn't exist, throw
        if !removed {
            self.env().revert(ValidatorNotInList);
        }

        // Track last recorded delegated amount
        self.last_recorded_delegated_amount
            .set(staked_cspr.saturating_sub(total_unstaked));

        // Track removed validator stake
        self.removed_validator_stake.set(
            self.removed_validator_stake
                .get_or_default()
                .saturating_add(total_unstaked),
        );
    }

    /// Restakes loose tokens
    /// Checks the amount of loose tokens (CSPR on the contract which is not staked
    /// or claimable) and delegates it to 3 random validators, equally divided
    pub fn restake_loose_tokens(&mut self) {
        self.ownable.assert_owner(&self.env().caller());
        let min_stake = self.get_min_stake();
        let loose_tokens = self.get_loose_tokens();

        if loose_tokens < min_stake {
            self.env().revert(NotAnOwner);
        }

        let validators_count = if loose_tokens < min_stake * BENEFICIAL_VALIDATORS_COUNT {
            (loose_tokens / min_stake).as_usize()
        } else {
            BENEFICIAL_VALIDATORS_COUNT
        };

        let validators = self.get_random_validators(validators_count);
        let amount_to_delegate = loose_tokens / validators.len();

        let mut delegated_amount = U512::zero();
        for validator in validators.iter() {
            self.delegate(validator.clone(), amount_to_delegate);
            delegated_amount += amount_to_delegate;
        }
        self.last_recorded_delegated_amount.set(self.staked_cspr());

        // Reduce the removed validator stake by the again delegated amount
        let removed_validator_stake = self.removed_validator_stake.get_or_default();
        if !removed_validator_stake.is_zero() {
            self.removed_validator_stake
                .set(removed_validator_stake.saturating_sub(delegated_amount));
        }
    }

    /// Returns the list of validators
    pub fn get_validators(&self) -> Vec<PublicKey> {
        self.validators.get_or_default()
    }

    /// Returns the amount of CSPR that is delegated to a validator
    pub fn get_validator_stake(&self, validator: PublicKey) -> U512 {
        self.env().delegated_amount(validator)
    }

    /// Returns the total amount of stake on the contract
    pub fn get_total_stake(&self) -> U512 {
        let validators = self.get_validators();

        // Early return if there are no validators
        if validators.is_empty() {
            return U512::zero();
        }

        validators.iter().fold(U512::zero(), |acc, validator| {
            acc + self.get_validator_stake(validator.clone())
        })
    }

    /// Returns the amount of loose tokens (CSPR on the contract which is not staked
    /// or claimable)
    pub fn get_loose_tokens(&self) -> U512 {
        // We allow saturating sub because there may be cspr we are still waiting to be unstaked
        self.env()
            .self_balance()
            .saturating_sub(self.total_unstakes.get_or_default())
    }

    /// Returns the claim time
    pub fn get_claim_time(&self) -> u64 {
        self.claim_time.get_or_default()
    }

    /// Sets the claim time
    pub fn set_claim_time(&mut self, claim_time: u64) {
        self.ownable.assert_owner(&self.env().caller());
        self.claim_time.set(claim_time);
    }
}

impl StakedCSPR {
    fn delegate(&self, public_key: PublicKey, amount: U512) {
        self.env().delegate(public_key.clone(), amount);
        self.env().emit_event(Delegated {
            address: self.env().caller(),
            amount,
            validator: public_key,
        });
    }

    fn undelegate(&self, public_key: PublicKey, amount: U512) {
        self.env().undelegate(public_key.clone(), amount);
        self.env().emit_event(Undelegated {
            address: self.env().caller(),
            amount,
            validator: public_key,
        });
    }

    fn next_claim_time(&self, block_time: u64) -> u64 {
        block_time + self.get_claim_time()
    }

    fn cspr_to_scspr(&self, cspr_stake: U512, staked_cspr: U512) -> U256 {
        // If there's no existing stake, the conversion is 1:1
        if staked_cspr.is_zero() {
            return cspr_stake.to_u256().unwrap_or_revert(self);
        }

        let scspr_total_supply = self.token.total_supply().to_u512();

        // If there's no existing supply, the conversion is also 1:1
        if scspr_total_supply.is_zero() {
            return cspr_stake.to_u256().unwrap_or_revert(self);
        }

        // Calculate sCSPR amount using the formula: cspr_stake * scspr_total_supply / staked_cspr
        // To minimize rounding errors, we multiply first, then divide
        // If there are some value leakage, it can be restaked using the restake_loose_tokens function
        (cspr_stake * scspr_total_supply / staked_cspr)
            .to_u256()
            .unwrap_or_revert(self)
    }

    fn scspr_to_cspr(&self, scspr: U256) -> U512 {
        // If there's no sCSPR being converted, return 0
        if scspr.is_zero() {
            return U512::zero();
        }

        let scspr_total_supply = self.token.total_supply().to_u512();

        if scspr_total_supply.is_zero() {
            self.env().revert(NoBackingForRedemption);
        }

        let staked_cspr = self.staked_cspr() + self.removed_validator_stake.get_or_default();

        // If there's no staked CSPR, conversion would be 1:1
        if staked_cspr.is_zero() {
            self.env().revert(NoBackingForRedemption);
        }

        // Calculate CSPR amount using the formula: scspr * staked_cspr / scspr_total_supply
        // To minimize rounding errors, we multiply first, then divide
        // If there are some value leakage, it can be restaked using the restake_loose_tokens function
        (scspr).to_u512() * staked_cspr / scspr_total_supply
    }

    /// Calculates the rewards since last collection and mints the fee
    fn collect_fee(&mut self, current_delegated: U512) {
        let last_recorded = self.last_recorded_delegated_amount.get_or_default();

        // First time delegation, set the last recorded delegation and bail, as there is no reward
        if last_recorded.is_zero() {
            self.last_recorded_delegated_amount.set(current_delegated);
            return;
        }

        // Only calculate rewards if delegation has increased.
        if current_delegated > last_recorded {
            let reward = current_delegated - last_recorded;
            let fee_percent = self.fee_percentage.get_or_default();
            // Fee calculation: fee = reward * fee_percentage / 10000 (basis points)
            let fee = reward * fee_percent / U512::from(10000u64);
            // Calculate fee_scspr based on total staked amount of cspr and total liquidity of scspr
            let total_scspr_liquidity = self.token.total_supply().to_u512();
            let fee_scspr = if total_scspr_liquidity.is_zero() {
                U256::zero()
            } else {
                (fee * total_scspr_liquidity / current_delegated)
                    .to_u256()
                    .unwrap_or_revert(self)
            };
            // Mint sCSPR to admin (using the deployer/admin stored in DEFAULT_ADMIN_ROLE)
            let admin = self.ownable.get_owner();
            self.token.raw_mint(&admin, &fee_scspr);

            self.env().emit_event(FeeCollected {
                amount: fee_scspr,
                recipient: admin,
            });

            // Update the last recorded delegation.
            self.last_recorded_delegated_amount.set(current_delegated);
        }
    }

    fn get_random_validator_indices(
        &self,
        amount: usize,
        validators_count: usize,
        seed: usize,
    ) -> Vec<usize> {
        // Early return if validators_count is 0
        if validators_count == 0 {
            return Vec::new();
        }

        // Cap the amount at the number of validators
        let validators_cap = amount.min(validators_count);

        let mut all_indices: Vec<usize> = (0..validators_count).collect();
        let mut selected_indices = Vec::with_capacity(validators_cap);

        // Select unique indices using a deterministic but pseudo-random approach
        for i in 0..validators_cap {
            // Determine next position based on seed and current iteration
            let pos = if all_indices.is_empty() {
                break;
            } else {
                (seed + i * 17) % all_indices.len()
            };

            // Take the index at that position
            let selected = all_indices.remove(pos);
            selected_indices.push(selected);
        }

        selected_indices
    }

    // Extract u32 from 4 bytes at specified offset
    fn extract_u32_from_bytes(&self, bytes: &[u8]) -> usize {
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize
    }

    fn get_random_validators(&self, amount: usize) -> Vec<PublicKey> {
        let validators = self.validators.get_or_revert_with(MisconfiguredValidator);

        if validators.is_empty() {
            self.env().revert(MisconfiguredValidator);
        }

        // Use pseudorandom_bytes(32) to get a seed value
        let random_bytes = self.env().pseudorandom_bytes(4);
        // Use first 4 bytes as seed by interpreting them as a u32
        let seed = self.extract_u32_from_bytes(&random_bytes);

        let indices = self.get_random_validator_indices(amount, validators.len(), seed);

        indices.iter().map(|i| validators[*i].clone()).collect()
    }

    // Update undelegate_from_validators to use simpler random selection
    fn undelegate_from_validators(&mut self, total_amount: U512) -> U512 {
        let mut remaining_amount = total_amount;
        let validators = self.validators.get_or_revert_with(MisconfiguredValidator);
        if validators.is_empty() {
            self.env().revert(MisconfiguredValidator);
        }

        // Use pseudorandom_bytes(32) to get a random index - reuse the same random bytes for multiple purposes
        let random_bytes = self.env().pseudorandom_bytes(4);
        // Use bytes at offset 4 as seed by interpreting them as a u32
        let seed = self.extract_u32_from_bytes(&random_bytes);

        // Start from a random index
        let start_idx = self.get_random_validator_indices(1, validators.len(), seed)[0];
        let len = validators.len();

        // Try each validator starting from the random index, wrapping around
        for i in 0..len {
            let idx = (start_idx + i) % len;
            let validator = validators[idx].clone();
            let delegated = self.env().delegated_amount(validator.clone());

            if delegated > U512::zero() {
                let amount_to_undelegate: U512 = if delegated >= remaining_amount {
                    remaining_amount
                } else {
                    delegated
                };

                self.undelegate(validator, amount_to_undelegate);
                remaining_amount = remaining_amount
                    .checked_sub(amount_to_undelegate)
                    .unwrap_or_revert_with(self, ArithmeticsError);

                if remaining_amount.is_zero() {
                    break;
                }
            }
        }

        total_amount - remaining_amount // Return the actual amount undelegated
    }

    fn assert_min_stake(&self, stake: U512) {
        if stake < self.get_min_stake() {
            self.env().revert(StakeBelowMinimum);
        }
    }
}

#[cfg(test)]
mod tests {

    const MIN_STAKE: u128 = 500_000_000_000;

    // Add this at the beginning of the tests module
    #[cfg(test)]
    mod state_writer {
        use super::*;
        pub trait StateWriter {
            fn write_state(&mut self, step: &str, token: &StakedCSPRHostRef, delays: u64);
        }

        pub struct NoOpStateWriter;

        impl StateWriter for NoOpStateWriter {
            fn write_state(&mut self, _step: &str, _token: &StakedCSPRHostRef, _delays: u64) {}
        }

        // Hidden behind a feature flag
        // It will dump the state of the contract and the balances of the accounts
        // into a csv file, for easier debugging and analysis
        #[cfg(feature = "csv-report")]
        pub struct CsvStateWriter {
            file: std::fs::File,
        }

        #[cfg(feature = "csv-report")]
        impl CsvStateWriter {
            pub fn new(filename: &str) -> Self {
                let file = std::fs::File::create(filename).unwrap();
                let mut writer = Self { file };
                writeln!(
                    writer.file,
                    "step,auction_delays,total_scspr,alice_scspr,bob_scspr,admin_scspr,alice_cspr,bob_cspr,admin_cspr,contract_cspr,delegated"
                ).unwrap();
                writer
            }
        }

        #[cfg(feature = "csv-report")]
        impl StateWriter for CsvStateWriter {
            fn write_state(&mut self, step: &str, token: &StakedCSPRHostRef, delays: u64) {
                let env = odra_test::env();
                let alice = env.get_account(1);
                let bob = env.get_account(2);
                let admin = env.get_account(0);

                let total_scspr = token.total_supply();
                let alice_scspr = token.balance_of(&alice);
                let bob_scspr = token.balance_of(&bob);
                let admin_scspr = token.balance_of(&admin);

                let alice_cspr = env.balance_of(&alice);
                let bob_cspr = env.balance_of(&bob);
                let admin_cspr = env.balance_of(&admin);
                let contract_cspr = token.self_balance();
                let delegated = token.staked_cspr();

                writeln!(
                    self.file,
                    "{},{},{},{},{},{},{},{},{},{},{}",
                    step,
                    delays,
                    total_scspr,
                    alice_scspr,
                    bob_scspr,
                    admin_scspr,
                    alice_cspr,
                    bob_cspr,
                    admin_cspr,
                    contract_cspr,
                    delegated
                )
                .unwrap();
            }
        }
    }
    use super::*;
    use crate::token::tests::state_writer::StateWriter;
    use odra::host::{Deployer, HostRef};

    #[test]
    fn test_initialization() {
        let env = odra_test::env();
        let token = StakedCSPR::deploy(
            &env,
            StakedCSPRInitArgs {
                validator_address: env.get_validator(0),
                claim_time: env.auction_delay() * 8,
                fee_percentage: 1000.into(),
                min_stake: U512::from(MIN_STAKE),
            },
        );
        assert!(token.get_owner() == env.caller());
    }

    #[test]
    fn test_fee_collection() {
        let env = odra_test::env();
        let auction_delay = env.auction_delay();
        let token = StakedCSPR::deploy(
            &env,
            StakedCSPRInitArgs {
                validator_address: env.get_validator(0),
                claim_time: env.auction_delay() * 8,
                fee_percentage: 100.into(),
                min_stake: U512::from(MIN_STAKE),
            },
        );
        // Given Alice and Bob.
        let alice = env.get_account(1);
        let bob = env.get_account(2);
        let admin = env.get_account(0);

        // When Alice stakes 10 CSPR.
        let deposit_amount_u512 = U512::from(1_000_000_000_000u64);
        env.set_caller(alice);
        token.with_tokens(deposit_amount_u512).stake();

        // And some time passes
        // First auction is for the delay to kick in the staking
        env.advance_with_auctions(auction_delay * 2);

        // And Bob stakes 10 CSPR.
        env.set_caller(bob);
        token.with_tokens(deposit_amount_u512).stake();

        // Then Admin has 998 sCSPR
        assert_eq!(token.balance_of(&admin), U256::from(998u64));
    }

    #[test]
    fn test_staking() {
        let env = odra_test::env();
        let auction_delay = env.auction_delay();
        let unbonding_delay = auction_delay * 8;

        // Setup accounts
        let alice = env.get_account(1);
        let admin = env.get_account(0);
        let alice_initial_cspr_balance = env.balance_of(&alice);

        // For debugging purposes
        // Create state writer (NoOp by default, CsvStateWriter when feature enabled)
        #[cfg(feature = "csv-report")]
        let mut state_writer = state_writer::CsvStateWriter::new("staking_report.csv");
        #[cfg(not(feature = "csv-report"))]
        let mut state_writer = state_writer::NoOpStateWriter;

        let mut token = StakedCSPR::deploy(
            &env,
            StakedCSPRInitArgs {
                validator_address: env.get_validator(0),
                claim_time: unbonding_delay,
                fee_percentage: 1000.into(),
                min_stake: U512::from(MIN_STAKE),
            },
        );

        // Track number of auction delays
        let mut total_delays = 0u64;

        // Initial state
        state_writer.write_state("initial", &token, total_delays);

        // When Alice stakes 10 CSPR
        let deposit_amount_u512 = U512::from(1_000_000_000_000u64);
        let deposit_amount_u256 = U256::from(1_000_000_000_000u64);
        env.set_caller(alice);
        token.with_tokens(deposit_amount_u512).stake();

        state_writer.write_state("after_alice_stake", &token, total_delays);

        // After time passes
        total_delays += 2;
        env.advance_with_auctions(auction_delay * 2);
        state_writer.write_state("after_delay", &token, total_delays);

        // When Alice unstakes
        token.unstake(deposit_amount_u256);
        state_writer.write_state("after_alice_unstake", &token, total_delays);

        // And one auction passes
        total_delays += 1;
        env.advance_with_auctions(auction_delay);
        state_writer.write_state("after_one_auction", &token, total_delays);

        // After unbonding delay
        total_delays += 7;
        env.advance_with_auctions(auction_delay * 7);
        state_writer.write_state("after_unbonding", &token, total_delays);

        // After Alice claims
        token.claim();
        state_writer.write_state("after_alice_claim", &token, total_delays);

        // Verify final state
        assert_eq!(token.balance_of(&admin), U256::from(9998));
        assert_eq!(token.staked_cspr(), U512::from(109998));
        assert_eq!(token.total_supply(), U256::from(9998));
        assert_eq!(
            env.balance_of(&alice),
            alice_initial_cspr_balance + U512::from(99999) - U512::from(9999)
        );
    }

    #[test]
    fn test_scspr_transfer() {
        // Given a deployed StakedCSPR contract.
        let env = odra_test::env();
        let auction_delay = env.auction_delay();
        let unbonding_delay = auction_delay * 8;
        let mut token = StakedCSPR::deploy(
            &env,
            StakedCSPRInitArgs {
                validator_address: env.get_validator(0),
                claim_time: env.auction_delay() * 8,
                fee_percentage: 1000.into(),
                min_stake: U512::from(MIN_STAKE),
            },
        );

        // Given Alice and Bob.
        let alice = env.get_account(1);
        let bob = env.get_account(2);
        let alice_initial_cspr_balance = env.balance_of(&alice);
        let bob_initial_cspr_balance = env.balance_of(&bob);

        // When Alice stakes 10 CSPR.
        let deposit_amount_u512 = U512::from(1_000_000_000_000u64);
        let deposit_amount_u256 = U256::from(1_000_000_000_000u64);
        env.set_caller(alice);
        token.with_tokens(deposit_amount_u512).stake();

        // Then staked CSPR should be 10 CSPR.
        assert_eq!(token.staked_cspr(), deposit_amount_u512);

        // Then Alice's balance should be less by 10 CSPR.
        let expected_amount = alice_initial_cspr_balance - deposit_amount_u512;
        assert_eq!(env.balance_of(&alice), expected_amount);

        // Then Alice's balance should be 10 sCSPR.
        assert_eq!(token.balance_of(&alice), deposit_amount_u256);

        // When Alice transfers 10 sCSPR to Bob.
        env.set_caller(alice);
        token.transfer(&bob, &deposit_amount_u256);

        // When time passes
        env.advance_with_auctions(auction_delay);

        // When Bob unstakes 10 sCSPR.
        env.set_caller(bob);
        token.unstake(deposit_amount_u256);

        // Then Bob's balance should be 0 sCSPR.
        assert_eq!(token.balance_of(&bob), U256::zero());

        // When time passes.
        env.advance_with_auctions(unbonding_delay);

        // And bob claims his unstake.
        env.set_caller(bob);
        token.claim();

        // // Then Bob's CSPR balance should be 10 CSPR more.
        let expected_amount = bob_initial_cspr_balance + deposit_amount_u512;
        assert_eq!(env.balance_of(&bob), expected_amount);
    }

    #[test]
    fn test_validators_management() {
        // Given a deployed StakedCSPR contract.
        let env = odra_test::env();
        let initial_validator = env.get_validator(0);
        let mut token = StakedCSPR::deploy(
            &env,
            StakedCSPRInitArgs {
                validator_address: initial_validator.clone(),
                claim_time: env.auction_delay() * 8,
                fee_percentage: 1000.into(),
                min_stake: U512::from(MIN_STAKE),
            },
        );

        // Given an admin account.
        let admin = env.get_account(0);
        env.set_caller(admin);

        // Then the initial validator should be in the list of validators and no other validator should be there.
        let validators = token.get_validators();
        assert_eq!(validators.len(), 1);
        assert!(validators.contains(&initial_validator));

        // When adding a new validator.
        let new_validator = env.get_validator(1);
        token.add_validator(new_validator.clone());

        // Then the new validator should be in the list of validators.
        let validators = token.get_validators();
        assert_eq!(validators.len(), 2);
        assert!(validators.contains(&new_validator));
        assert!(validators.contains(&initial_validator));
    }
}
