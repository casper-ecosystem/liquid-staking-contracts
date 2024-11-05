Feature: Rewards pool
  Scenario: Adding and removing cspr by an admin
    Given Owner has 1000 CSPR
    When Owner adds 100 CSPR to the pool
    Then 100 CSPR is staked
    When Owner removes 50 CSPR from the pool
    Then 50 CSPR is staked