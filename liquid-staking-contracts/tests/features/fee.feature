Feature: Administration fees
  Scenario: Simple fee collection
    Given Alice has 1000 CSPR
    And Owner has 0 CSPR
    When Alice stakes 1000 CSPR
    And 2 auction passes
    Then more than 1000 CSPR is staked
    When Alice unstakes everything
    And unbonding period passes
    And Alice claims unstakes
    Then Alice has more than 1000 CSPR
    And Owner has more than 0 sCSPR

  Scenario: Fee collection with exact numbers - small reward
    Given Alice has 1000 CSPR
    And Owner has 500 CSPR
    When Alice stakes 1000 CSPR
    And 1 auction passes
    Then 1000.000099999 CSPR is staked
    And Owner has 0 sCSPR
    When Alice unstakes everything
    And unbonding period passes
    And Alice claims unstakes
    Then Alice has 1000.00009 CSPR
    And Owner has 0.000009998 sCSPR
    When Owner adds 500 CSPR to the pool
    When Owner unstakes everything
    And unbonding period passes
    And Owner claims unstakes
    Then Owner has 500.000009999 CSPR
    And 0 CSPR is staked
    And 0 sCSPR is in the pool

  Scenario: Fee collection with exact numbers - BIG reward
    Given Alice has 1000 CSPR
    And Owner has 500 CSPR
    When Alice stakes 1000 CSPR
    And 101 auction passes
    Then 1000.010099899 CSPR is staked
    And Owner has 0 sCSPR
    When Alice unstakes everything
    And unbonding period passes
    And Alice claims unstakes
    Then Alice has 1000.009089911 CSPR
    And Owner has 0.001009978 sCSPR
    When Owner adds 500 CSPR to the pool
    And Owner unstakes everything
    And unbonding period passes
    And Owner claims unstakes
    Then Owner has 500.001009988 CSPR
    And 0 CSPR is staked
    And 0 sCSPR is in the pool
