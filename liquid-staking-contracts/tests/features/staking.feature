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
    When 7 eras pass
    And Alice claims unstake with id 0
    Then Alice's token balance is 50 sCSPR
