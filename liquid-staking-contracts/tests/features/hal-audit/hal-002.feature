Feature: Fix for HAL-002

  Scenario: Contract lacks mechanism to transfer ownership after deployment
    Given Owner is the contract owner
    When Owner transfers ownership to Alice
    Then Owner is the contract owner
    And Alice is not the contract owner
    When Alice accepts ownership
    Then Owner is not the contract owner
    And Alice is the contract owner