Feature: Liquid Staking features

  Scenario: Simple staking and unstaking
    When Alice stakes 100 CSPR
    Then Alice's token balance is 100 sCSPR
    And staked CSPR is 100 CSPR
    When Alice unstakes 50 sCSPR
    Then Alice's token balance is 50 sCSPR