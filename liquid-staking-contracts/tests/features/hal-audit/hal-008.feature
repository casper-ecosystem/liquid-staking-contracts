Feature: Fix for HAL-008

  Scenario: Unused constant MIN_STAKE and lack of validation allow unsafe initialization
    Given the contract is deployed with 500 CSPR min_stake
    And the contract cannot be deployed with 0 CSPR min_stake