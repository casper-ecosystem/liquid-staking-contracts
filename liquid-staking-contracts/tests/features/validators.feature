Feature: Multiple Validators
  Background:
    Given Owner deploys a contract with Validator1
    Then Validator1 is a validator
    And the contract has 1 validator

  Scenario: Adding and removing validators
    When Owner adds Validator2
    Then Validator2 is a validator
    And contract has 2 validators
    When Owner removes Validator1
    Then Validator1 is not a validator
    And contract has 1 validator

  Scenario: Adding and removing validators with unprivileged account
    When Alice tries to add Validator2
    Then Validator2 is not a validator
    And contract has 1 validator
    When Alice tries to remove Validator1
    Then Validator1 is a validator
    And contract has 1 validator

  Scenario: Adding a validator more than once
    When Owner adds Validator2
    Then Validator2 is a validator
    And contract has 2 validators
    And Owner cannot add Validator2

  Scenario: Removing a validator that is not in the list
    When Owner tries to remove Validator3
    Then contract has 1 validator

  Scenario: Only an admin can remove a validator
    When Owner adds Validator2
    And Alice tries to remove Validator2
    Then Validator2 is a validator
    And contract has 2 validators
    When Owner removes Validator2
    Then Validator2 is not a validator
    And contract has 1 validator

  Scenario: Removing a validator with unbonding period, restaking loose tokens, recording unstaked amount
    Given Alice has 1000 CSPR
    And Owner has 0 CSPR
    When Owner adds Validator2
    And Alice stakes 1000 CSPR
    And 2 auction passes
    Then 1000.000099999 CSPR is staked
    And Owner has 0 sCSPR
    When Owner removes Validator1
    Then Owner has 0.000009998 sCSPR
    And Alice has 1000 sCSPR
    And 1000.000099999 CSPR is staked
    When unbonding period passes
    And Owner restakes loose tokens
    And Alice unstakes everything
    And unbonding period passes
    And Alice claims unstakes
    Then Alice has roughly 1000.00019 CSPR
    And Owner has 0.000009998 sCSPR
    When Owner unstakes everything
    And unbonding period passes
    And Owner claims unstakes
    Then Owner has 0.000009999 CSPR
    And 0 CSPR is staked
    And 0 sCSPR is in the pool
