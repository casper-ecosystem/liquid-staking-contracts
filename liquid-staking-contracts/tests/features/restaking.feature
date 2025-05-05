Feature: Restaking after validator removal
  Background:
    Given Owner deploys a contract with Validator1

  Scenario: Adding a new validator does not trigger restaking
    Given Alice has 1000 CSPR
    When Alice stakes 1000 CSPR
    Then Alice's token balance is 1000 sCSPR
    And Validator1 has 1000 CSPR in his staking pool
    When Owner adds Validator2
    Then Validator2 is a validator
    And the contract has 2 validators
    And Validator1 has 1000 CSPR in his staking pool
    And Validator2 has 0 CSPR in his staking pool
    
  Scenario: New stakes are assigned to a randomly selected validator
    Given Alice has 1000 CSPR
    And Bob has 2000 CSPR
    When Alice stakes 1000 CSPR
    Then Alice's token balance is 1000 sCSPR
    And Validator1 has 1000 CSPR in his staking pool
    When Owner adds Validator2
    Then Validator2 is a validator
    And the contract has 2 validators
    When Bob stakes 2000 CSPR
    Then Bob's token balance is 2000 sCSPR
    And staked CSPR is 3000 CSPR
    And total stakes across all validators is 3000 CSPR
    And exactly one validator received 2000 CSPR since previous stake

  Scenario: Restaking after validator removal - not enough CSPR for 3 validators (min stake)
    Given Alice has 1000 CSPR
    When Alice stakes 1000 CSPR
    And Owner adds Validator2
    And Owner adds Validator3
    And Owner adds Validator4
    And Owner adds Validator5
    And Owner removes Validator1
    And unbonding period passes
    Then staked CSPR is 1000 CSPR
    But 1000 CSPR is loose
    When Owner restakes loose tokens
    Then 2 validators received total 1000 CSPR since last remove in equal amounts


  Scenario: Restaking after validator removal - enough CSPR for 3 validators
    Given Alice has 2000 CSPR
    When Alice stakes 2000 CSPR
    And Owner adds Validator2
    And Owner adds Validator3
    And Owner adds Validator4
    And Owner adds Validator5
    And Owner removes Validator1
    And unbonding period passes
    Then staked CSPR is 2000 CSPR
    But 2000 CSPR is loose
    When Owner restakes loose tokens
    Then 3 validators received total 2000 CSPR since last remove in equal amounts