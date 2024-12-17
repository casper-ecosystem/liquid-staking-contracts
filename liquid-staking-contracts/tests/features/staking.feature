Feature: Liquid Staking features

  Scenario: Simple staking and unstaking
    Given Alice has 100 CSPR
    When Alice stakes 100 CSPR
    Then Alice's token balance is 100 sCSPR
    And Alice's CSPR balance is 0 CSPR
    And staked CSPR is 100 CSPR
    When Alice unstakes 50 sCSPR
    Then Alice's token balance is 50 sCSPR
    And Alice's CSPR balance is 0 CSPR
    When 1 auction passes
    And Alice claims unstake with id 0
    Then Alice has 50 CSPR
    And total supply is 50 sCSPR

  Scenario: Staking and collecting rewards
    Given Alice has 100 CSPR
    When Alice stakes 100 CSPR
    And Owner adds 100 CSPR to the pool
    And Alice unstakes 100 sCSPR
    And 1 auction passes
    And Alice claims unstake with id 0
    Then Alice has 200 CSPR

  Scenario: Staking by multiple accounts
    Given Alice has 100 CSPR
    And Bob has 100 CSPR
    When Alice stakes 100 CSPR
    And Bob stakes 100 CSPR
    Then Alice's token balance is 100 sCSPR
    And Bob's token balance is 100 sCSPR
    And Alice's CSPR balance is 0 CSPR
    And Bob's CSPR balance is 0 CSPR
    And staked CSPR is 200 CSPR
    When Alice unstakes 50 sCSPR
    Then Alice's token balance is 50 sCSPR
    And Bob's token balance is 100 sCSPR
    And Alice's CSPR balance is 0 CSPR
    And Bob's CSPR balance is 0 CSPR
    And staked CSPR is 150 CSPR
    When 1 auction passes
    And Alice claims unstake with id 0
    Then Alice has 50 CSPR
    And Bob has 0 CSPR
    And total supply is 150 sCSPR

  Scenario: Staking by multiple accounts and collecting rewards
    Given Alice has 50 CSPR
    And Bob has 150 CSPR
    When Alice stakes 50 CSPR
    And Bob stakes 150 CSPR
    And Owner adds 100 CSPR to the pool
    And Alice unstakes 50 sCSPR
    And Bob unstakes 150 sCSPR
    And 1 auction passes
    And Alice claims unstake with id 0
    And Bob claims unstake with id 1
    Then Alice has 75 CSPR
    And Bob has 225 CSPR