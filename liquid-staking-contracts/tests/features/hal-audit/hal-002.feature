Feature: Fix for HAL-002 bug

  Scenario: Contract lacks mechanism to transfer ownership after deployment
    Given Owner is the contract owner
    When Owner transfers ownership to Alice
    Then Owner is not the contract owner
    And Alice is the contract owner