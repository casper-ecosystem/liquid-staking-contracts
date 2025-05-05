Feature: Fix for HAL-003

  Scenario: Lack of upper bound and validation on fee_percentage allows protocol-level inflation abuse
    Given the contract is deployed with 1000 basis points fee
    And the contract cannot be deployed with 10001 basis points fee