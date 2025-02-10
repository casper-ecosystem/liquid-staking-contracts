Feature: Rewards pool
  Scenario: Adding and removing cspr by an admin
    Given Owner has 1000 CSPR
    When Owner adds 100 CSPR to the pool
    Then 100 CSPR is staked
    When Owner removes 50 CSPR from the pool
    Then 50 CSPR is staked

  Scenario: Check basic staking
    Given Alice has 200000 CSPR
    When Alice stakes 200000 CSPR
    And 2 auction passes
    Then more than 200000 CSPR is staked
