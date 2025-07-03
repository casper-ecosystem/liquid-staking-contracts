Feature: Bug fixes and edge cases

  Scenario: Multiple unstake claims in a single transaction
    Given Alice has 1000 CSPR
    When Alice stakes 1000 CSPR
    And Alice unstakes 300 sCSPR
    And Alice unstakes 300 sCSPR 
    And Alice unstakes 300 sCSPR
    And unbonding period passes
    # This will try to claim all three unstakes at once and panic before the bug was fixed
    And Alice claims unstakes
    Then Alice has 900 CSPR
    And Alice's token balance is 100 sCSPR 

  Scenario: Unstaking very small amount
    # Initial condition.
    Given Alice has 500 CSPR
    When Alice stakes 500 CSPR
    Then Alice has 500 sCSPR
    
    # First Alice unstakes a very small amount.
    When Alice unstakes 100 sCSPR
    And 20 auction passes
    Then 400 CSPR is staked
    Then the contract has 500 CSPR
    When Alice claims unstakes
    Then Alice has 100 CSPR
    And Alice's token balance is 400 sCSPR
    And the contract has 400 CSPR

    # Then Alice unstakes the rest.
    When Alice unstakes 499.999999999 sCSPR
    And unbonding period passes
    And Alice claims unstakes
    Then Alice has roughly 500.000 CSPR
    And Alice's token balance is 0 sCSPR
    And the contract has 0 CSPR
