Feature: Restaking after validator removal
  Background:
    Given Owner deploys a contract with Validator1

  Scenario: Adding a new validator does not trigger restaking
    Given Alice has 100 CSPR
    When Alice stakes 100 CSPR
    Then Alice's token balance is 100 sCSPR
    And Validator1 has 100 CSPR in his staking pool
    When Owner adds Validator2
    Then Validator2 is a validator
    And the contract has 2 validators
    And Validator1 has 100 CSPR in his staking pool
    And Validator2 has 0 CSPR in his staking pool
    
  Scenario: New stakes are assigned to a randomly selected validator
    Given Alice has 100 CSPR
    And Bob has 200 CSPR
    When Alice stakes 100 CSPR
    Then Alice's token balance is 100 sCSPR
    And Validator1 has 100 CSPR in his staking pool
    When Owner adds Validator2
    Then Validator2 is a validator
    And the contract has 2 validators
    When Bob stakes 200 CSPR
    Then Bob's token balance is 200 sCSPR
    And staked CSPR is 300 CSPR
    And total stakes across all validators is 300 CSPR
    And exactly one validator received 200 CSPR since previous stake

