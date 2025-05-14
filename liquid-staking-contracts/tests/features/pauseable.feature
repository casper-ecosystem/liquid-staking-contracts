Feature: Pauseable

  Scenario: Pausing and unpausing
    When the contract is paused
    Then the contract is paused
    When the contract is unpaused
    Then the contract is not paused

  Scenario: Pausing locks the contract
    Given Owner has 1000 CSPR
    And Owner stakes 1000 CSPR
    When the contract is paused
    Then transfer fails because the contract is paused
    And transfer_from fails because the contract is paused
    And increase_allowance fails because the contract is paused
    And decrease_allowance fails because the contract is paused
    And approve fails because the contract is paused
    And claim fails because the contract is paused
    And stake fails because the contract is paused
    And unstake fails because the contract is paused

  Scenario: Unpausing unlocks the contract
    Given Owner has 1000 CSPR
    And Owner stakes 1000 CSPR
    When the contract is unpaused
    Then transfer succeeds because the contract is unpaused
    And transfer_from succeeds because the contract is unpaused
    And increase_allowance succeeds because the contract is unpaused
    And decrease_allowance succeeds because the contract is unpaused
    And approve succeeds because the contract is unpaused
    And stake succeeds because the contract is unpaused
    And unstake succeeds because the contract is unpaused
    And claim succeeds because the contract is unpaused

  Scenario: Pausing does not lock admin functions
    When the contract is paused
    Then add_validator succeeds despite the contract being paused
    And remove_validator succeeds despite the contract being paused
    And set_min_stake succeeds despite the contract being paused
    And set_claim_time succeeds despite the contract being paused
    And set_fee_percentage succeeds despite the contract being paused
    And add_loose_tokens succeeds despite the contract being paused
    And restake_loose_tokens succeeds despite the contract being paused
    