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
