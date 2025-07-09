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
