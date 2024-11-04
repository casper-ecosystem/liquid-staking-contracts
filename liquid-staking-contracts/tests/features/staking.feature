Feature: Liquid Staking features

  Scenario: Simple staking
    When Alice stakes 100 CSPR
    Then Alice's token balance is 100 sCSPR
    And staked CSPR is 100 CSPR