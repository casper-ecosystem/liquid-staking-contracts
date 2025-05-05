Feature: Fix for HAL-007

  Scenario: Lack of error handling when adding an already registered validator
    Given Owner deploys a contract with Validator1
    When Owner adds Validator2
    Then Owner cannot add Validator2