use odra::{
    casper_types::{PublicKey, U256, U512},
    prelude::*,
};

/// Event emitted when a user stakes CSPR
#[odra::event]
pub struct Staked {
    /// The address of the user who staked the CSPR
    pub address: Address,
    /// The amount of CSPR that was staked
    pub cspr_amount: U512,
    /// The amount of sCSPR that was minted
    pub scspr_minted: U256,
}

/// Event emitted when a user unstakes CSPR
#[odra::event]
pub struct Unstaked {
    /// The address of the user who unstaked the CSPR
    pub address: Address,
    /// The amount of CSPR that was unstaked
    pub cspr_amount: U512,
    /// The amount of sCSPR that was burned
    pub scspr_burned: U256,
    /// The id of the unstake
    pub unstake_id: u32,
    /// The time when the unstake will be claimable
    pub claim_time: u64,
}

/// Event emitted when a user claims their unstaked CSPR
#[odra::event]
pub struct Claimed {
    /// The address of the user who claimed the unstake
    pub address: Address,
    /// The amount of CSPR that was claimed
    pub cspr_amount: U512,
    /// The id of the unstake
    pub unstake_id: u32,
}

/// Event emitted when a user delegates CSPR to a validator
#[odra::event]
pub struct Delegated {
    /// The address of the user who delegated the CSPR
    pub address: Address,
    /// The amount of CSPR that was delegated
    pub amount: U512,
    /// The validator that the CSPR was delegated to
    pub validator: PublicKey,
}

/// Event emitted when a user undelegates CSPR from a validator
#[odra::event]
pub struct Undelegated {
    /// The address of the user who undelegated the CSPR
    pub address: Address,
    /// The amount of CSPR that was undelegated
    pub amount: U512,
    /// The validator that the CSPR was undelegated from
    pub validator: PublicKey,
}

/// Event emitted when a validator is removed
#[odra::event]
pub struct ValidatorRemoved {
    /// The address of the validator that was removed
    pub validator: PublicKey,
}

/// Event emitted when a validator is added
#[odra::event]
pub struct ValidatorAdded {
    /// The address of the validator that was added
    pub validator: PublicKey,
}

/// Event emitted when cspr is added to the pool
#[odra::event]
pub struct CsprAddedToPool {
    /// The amount of CSPR that was added to the pool
    pub amount: U512,
}

/// Event emitted when cspr is removed from the pool
#[odra::event]
pub struct CsprRemovedFromPool {
    /// The amount of CSPR that was removed from the pool
    pub amount: U512,
}

/// This event is emitted when CSPR is withdrawn from the contract.
#[odra::event]
pub struct CsprWithdrawnFromContract {
    /// The amount of CSPR that was withdrawn.
    pub amount: U512,
    /// The recipient address.
    pub recipient: Address,
}
