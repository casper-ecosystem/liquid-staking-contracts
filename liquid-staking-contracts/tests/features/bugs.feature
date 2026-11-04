Feature: Bug fixes and edge cases

  Scenario: Multiple unstake claims in a single transaction
    Given Alice has 1000 CSPR
    When Alice stakes 1000 CSPR
    And Alice unstakes 300 sCSPR
    And Alice unstakes 300 sCSPR 
    And unbonding period passes
    # This will try to claim all three unstakes at once and panic before the bug was fixed
    And Alice claims unstakes
    Then Alice has 600 CSPR
    And Alice's token balance is 400 sCSPR

  Scenario: Unstaking under minimum bidding
    # Alice stakes minimum bidding amount
    Given Alice has 500 CSPR
    And Bob has 500 CSPR
    When Alice stakes 500 CSPR
    Then Alice has 500 sCSPR
    
    # First Alice unstakes amount making her stake below minimum
    When Alice unstakes 100 sCSPR
    And unbonding period passes

    # Everything is unstaked, but is taken into account
    Then 400 CSPR is staked
    Then the contract has 500 CSPR
    When Alice claims unstakes
    Then Alice has 100 CSPR
    And Alice's token balance is 400 sCSPR
    And the contract has 400 CSPR
    And 400 CSPR is staked

    # For Alice to unstake the rest, we need to stake some more tokens
    When Owner stakes 500 CSPR to the Validator1
    Then 900 CSPR is staked
    And 900 sCSPR is in the pool
    When Alice unstakes 400 sCSPR
    And unbonding period passes
    And Alice claims unstakes
    Then Alice has roughly 500.000 CSPR
    And Alice's token balance is 0 sCSPR
    And the contract has 500 CSPR

  Scenario: Removing a validator does not charge the fee on the removed stake
    # The contract is deployed with a 10% fee. Nothing is earned during this
    # scenario, so no fee may be collected at any point.
    Given Alice has 1000 CSPR
    And Bob has 1000 CSPR
    And Owner has 1000 CSPR
    When Alice stakes 1000 CSPR
    And Owner adds Validator2
    And Owner stakes 1000 CSPR to the Validator2
    Then 2000 CSPR is staked
    And 2000 sCSPR is in the pool
    And Owner has 1000 sCSPR

    # Removing Validator1 moves its 1000 CSPR into removed validator stake,
    # so the total staked amount does not change.
    When Owner removes Validator1
    Then removed validator stake is 1000 CSPR
    And 2000 CSPR is staked
    And Owner has 1000 sCSPR

    # The next fee collection must not treat the removed stake as a reward.
    # Before the fix the Owner was minted 100 sCSPR here (10% of 1000 CSPR).
    When Bob stakes 1000 CSPR
    Then Owner has 1000 sCSPR
    And Bob has 1000 sCSPR
    And 3000 CSPR is staked
    And 3000 sCSPR is in the pool

    # Before the fix Alice would only get 952.380952380 CSPR back.
    When Alice unstakes everything
    Then unstake event says Alice will receive 1000 CSPR
    When unbonding period passes
    And Alice claims unstakes
    Then Alice has 1000 CSPR

  Scenario: After a validator removal the fee is charged on rewards only
    Given Alice has 1000 CSPR
    And Bob has 1000 CSPR
    And Owner has 1000 CSPR
    When Alice stakes 1000 CSPR
    And Owner adds Validator2
    And Owner stakes 1000 CSPR to the Validator2
    And Owner removes Validator1
    Then Owner has 1000 sCSPR
    And 2000 CSPR is staked

    # Only the 1000 CSPR left on Validator2 earns rewards.
    When 1 auction passes
    Then 2000.000099999 CSPR is staked

    # The fee is 10% of the 0.000099999 CSPR reward, paid in sCSPR.
    # Before the fix the removed 1000 CSPR was also treated as a reward and the
    # Owner ended up with 1100.000004999 sCSPR.
    When Bob stakes 1000 CSPR
    Then Owner has 1000.000009998 sCSPR
