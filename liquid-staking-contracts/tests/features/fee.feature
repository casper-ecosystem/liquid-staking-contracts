Feature: Administration fees
  Scenario: Simple fee collection
    Given Alice has 100 CSPR
    And Owner has 0 CSPR
    When Alice stakes 100 CSPR
    And 2 auction passes
    Then more than 100 CSPR is staked
    When Alice unstakes everything
    And unbonding period passes
    And Alice claims unstake with id 0
    Then Alice has more than 100 CSPR
    And Owner has more than 0 sCSPR

  Scenario: Fee collection with exact numbers - small reward
    Given Alice has 100 CSPR
    And Owner has 0 CSPR
    When Alice stakes 100 CSPR
    And 2 auction passes
    Then 100.00001 CSPR is staked
    And Owner has 0 sCSPR
    When Alice unstakes everything
    And unbonding period passes
    And Alice claims unstake with id 0
    Then Alice has 100.000009 CSPR
    And Owner has 0.000000998 sCSPR
    When Owner unstakes everything
    And unbonding period passes
    And Owner claims unstake with id 1
    Then Owner has 0.00001 CSPR
    And 0.0000009 CSPR is staked
    And 0.00000009 sCSPR is in the pool

  Scenario: Fee collection with exact numbers - BIG reward
    Given Alice has 100 CSPR
    And Owner has 0 CSPR
    When Alice stakes 100 CSPR
    And 101 auction passes
    Then 100.001 CSPR is staked
    And Owner has 0 sCSPR
    When Alice unstakes everything
    And unbonding period passes
    And Alice claims unstake with id 0
    Then Alice has 100.001 CSPR
    And Owner has 0.000099989 sCSPR
    When Owner unstakes everything
    And unbonding period passes
    And Owner claims unstake with id 1
    Then Owner has 0.0001 CSPR
    And 0.00000099 CSPR is staked
    And 0.0000009 sCSPR is in the pool
