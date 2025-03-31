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
    When Owner adds Validator2 again
    Then Validator2 is a validator
    And contract has 2 validators

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
