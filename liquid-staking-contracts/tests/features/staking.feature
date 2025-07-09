Feature: Liquid Staking features

  Scenario: Simple staking and unstaking
    Given Alice has 1000 CSPR
    When Alice stakes 1000 CSPR
    Then Alice's token balance is 1000 sCSPR
    And Alice's CSPR balance is 0 CSPR
    And staked CSPR is 1000 CSPR
    When Alice unstakes 500 sCSPR
    Then Alice's token balance is 500 sCSPR
    And Alice's CSPR balance is 0 CSPR
    When unbonding period passes
    And Alice claims unstakes
    Then Alice has 500 CSPR
    And total supply is 500 sCSPR

  Scenario: Staking and collecting rewards
    Given Alice has 1000 CSPR
    When Alice stakes 1000 CSPR
    And Owner adds 1000 CSPR to the pool
    And Alice unstakes 1000 sCSPR
    And unbonding period passes
    And Alice claims unstakes
    Then Alice has 2000 CSPR

  Scenario: Staking by multiple accounts
    Given Alice has 1000 CSPR
    And Bob has 1000 CSPR
    When Alice stakes 1000 CSPR
    And Bob stakes 1000 CSPR
    Then Alice's token balance is 1000 sCSPR
    And Bob's token balance is 1000 sCSPR
    And Alice's CSPR balance is 0 CSPR
    And Bob's CSPR balance is 0 CSPR
    And staked CSPR is 2000 CSPR
    When Alice unstakes 500 sCSPR
    Then Alice's token balance is 500 sCSPR
    And Bob's token balance is 1000 sCSPR
    And Alice's CSPR balance is 0 CSPR
    And Bob's CSPR balance is 0 CSPR
    And staked CSPR is 1500 CSPR
    When unbonding period passes
    And Alice claims unstakes
    And Bob claims unstakes
    Then Alice has 500 CSPR
    And Bob has 0 CSPR
    And total supply is 1500 sCSPR

  Scenario: Staking by multiple accounts and collecting rewards
    Given Alice has 500 CSPR
    And Bob has 1500 CSPR
    When Alice stakes 500 CSPR
    And Bob stakes 1500 CSPR
    And Owner adds 1000 CSPR to the pool
    And Alice unstakes 500 sCSPR
    And Bob unstakes 1500 sCSPR
    And unbonding period passes
    And Alice claims unstakes
    And Bob claims unstakes
    Then Alice has 750 CSPR
    And Bob has 2250 CSPR

  Scenario: Staking with rewards and fee collection
    Given Alice has 1000 CSPR
    When Alice stakes 1000 CSPR
    Then Alice's token balance is 1000 sCSPR
    And Alice's CSPR balance is 0 CSPR
    And staked CSPR is 1000 CSPR
    When 1 auctions pass
    Then staked CSPR is 1000.000099999 CSPR
    When Alice unstakes 1000 sCSPR
    Then Alice's token balance is 0 sCSPR
    And Owner's token balance is 0.000009998 sCSPR
    And staked CSPR is 0.000009999 CSPR
    When unbonding period passes
    Then staked CSPR is 0.000009999 CSPR
    When Alice claims unstakes
    Then Alice has 1000.0000900 CSPR
    And total supply is 0.000009998 sCSPR
    And staked CSPR is 0.000009999 CSPR