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
    And Owner has more than 0 sCSPR

  Scenario: Fee collection with exact numbers - small reward
    Given Alice has 200000000 CSPR
    And Owner has 0 CSPR
    When Alice stakes 200000000 CSPR
    And 1 auction passes
    Then 200000079 CSPR is staked
    And Owner has 0 sCSPR
    When Alice unstakes everything
    And unbonding period passes
    And Alice claims unstake with id 0
    Then Alice has 200000072 CSPR
    And Owner has 6 sCSPR
    When Owner unstakes everything
    And unbonding period passes
    And Owner claims unstake with id 1
    Then Owner has 7 CSPR
    And 0 CSPR is staked
    And 0 sCSPR is in the pool

  Scenario: Fee collection with exact numbers - BIG reward
    Given Alice has 200000000 CSPR
    And Owner has 0 CSPR
    When Alice stakes 200000000 CSPR
    And 100 auction passes
    Then 200007999 CSPR is staked
    And Owner has 0 sCSPR
    When Alice unstakes everything
    And unbonding period passes
    And Alice claims unstake with id 0
    Then Alice has 200007200 CSPR
    And Owner has 798 sCSPR
    When Owner unstakes everything
    And unbonding period passes
    And Owner claims unstake with id 1
    Then Owner has 799 CSPR
    And 0 CSPR is staked
    And 0 sCSPR is in the pool
