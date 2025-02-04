Feature: Administration fees
  Scenario: Simple fee collection
    Given Alice has 200000000 CSPR
    And Owner has 0 CSPR
    When Alice stakes 200000000 CSPR
    And 1 auction passes
    Then more than 200000000 CSPR is staked
    When Alice unstakes everything
    And unbonding period passes
    And Alice claims unstake with id 0
    Then Alice has more than 200000000 CSPR
    And Owner has more than 0 CSPR