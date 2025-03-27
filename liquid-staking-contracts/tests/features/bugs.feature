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