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

  Scenario: Counting removed validator stake
    Given the contract is deployed with 1000 basis points fee
    And Alice has 1000 CSPR
    And Bob has 1000 CSPR
    When Alice stakes 1000 CSPR
    And 10 auction passes
    Then 1000.000899991 CSPR is staked
    When Owner adds Validator2
    When Owner removes Validator1
    Then removed validator stake is 1000.000899991 CSPR
    And 1000.000899991 CSPR is staked
    Then Alice has 1000 sCSPR
    And Bob has 0 sCSPR
    When Bob stakes 1000 CSPR
    # Before the fix, Bob would have around 500 sCSPR instead
    Then Bob has 999.999190007 sCSPR
