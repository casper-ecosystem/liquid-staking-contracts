Feature: Fix for HAL-001

  Scenario: Validator removal bypasses fee collection and breaks delegation tracking
    Given Alice has 1000 CSPR
    And Owner has 0 CSPR
    When Alice stakes 1000 CSPR
    And 2 auction passes
    Then 1000.000099999 CSPR is staked
    And Owner has 0 sCSPR
    When Owner removes Validator1
    And Owner adds Validator2
    # Following step would fail as the contract would not calculate the fee correctly (Owner would have 0)
    Then Owner has 0.000009998 sCSPR
    And Alice has 1000 sCSPR
    And 1000.000099999 CSPR is staked 
    When unbonding period passes
    And Owner restakes loose tokens
    And Alice unstakes everything
    And unbonding period passes
    And Alice claims unstakes
    Then Alice has roughly 1000.00019 CSPR
    And Owner has 0.000009998 sCSPR
    When Owner unstakes everything
    And unbonding period passes
    And Owner claims unstakes
    Then Owner has 0.000009999 CSPR
    And 0 CSPR is staked
    And 0 sCSPR is in the pool
