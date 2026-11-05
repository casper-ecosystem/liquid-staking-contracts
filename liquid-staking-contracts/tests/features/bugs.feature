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

  Scenario: Undelegating below the contract minimum but above the validator minimum
    # Validators accept delegations of 500 CSPR or more, the contract is configured higher.
    Given the contract is deployed with 1000 CSPR min_stake
    And Alice has 1500 CSPR
    And Bob has 1000 CSPR
    When Alice stakes 1500 CSPR
    And Alice unstakes 700 sCSPR
    # 800 CSPR stay delegated, nothing was force-unbonded, so nothing is in flight.
    # Before the fix removed validator stake was 800 CSPR and 1600 CSPR counted as staked.
    Then Validator1 has 800 CSPR in his staking pool
    And removed validator stake is 0 CSPR
    And 800 CSPR is staked

    # Before the fix Bob received only 500 sCSPR for his 1000 CSPR.
    When Bob stakes 1000 CSPR
    Then Bob has 1000 sCSPR
    And 1800 CSPR is staked

    # Before the fix Alice would have received 1600 CSPR for her remaining 800 sCSPR.
    When Alice unstakes everything
    Then 1000 CSPR is staked
    When unbonding period passes
    And Alice claims unstakes
    Then Alice has 1500 CSPR
    When Bob unstakes everything
    And unbonding period passes
    And Bob claims unstakes
    Then Bob has more than 1000 CSPR

  Scenario: Undelegating below the validator minimum but above the contract minimum
    # Validators accept delegations of 500 CSPR or more, the contract is configured lower.
    Given the contract is deployed with 100 CSPR min_stake
    And Alice has 1000 CSPR
    And Bob has 1000 CSPR
    And Owner has 200 CSPR
    When Alice stakes 1000 CSPR
    And Alice unstakes 700 sCSPR
    # Only 300 CSPR would remain, so the validator unbonds all of it. Those 300 CSPR
    # are on their way back to the contract and must still count as staked.
    # Before the fix removed validator stake was 0 CSPR and nothing counted as staked.
    Then Validator1 has 0 CSPR in his staking pool
    And removed validator stake is 300 CSPR
    And 300 CSPR is staked

    # Before the fix Bob's stake was priced against an empty pool and diluted Alice.
    When Bob stakes 1000 CSPR
    Then Bob has 1000 sCSPR
    And 1300 CSPR is staked

    # The force-unbonded 300 CSPR arrive as loose tokens.
    When unbonding period passes
    Then 300 CSPR is loose
    When Alice claims unstakes
    Then Alice has 700 CSPR
    And 300 CSPR is loose

    # Restaking the loose tokens clears the removed validator stake again.
    # Bob's delegation earned rewards meanwhile, hence "more than".
    When Owner adds 200 CSPR to the pool
    Then removed validator stake is 0 CSPR
    And 0 CSPR is loose
    And more than 1500 CSPR is staked

    When Alice unstakes everything
    And unbonding period passes
    And Alice claims unstakes
    Then Alice has more than 1000 CSPR
