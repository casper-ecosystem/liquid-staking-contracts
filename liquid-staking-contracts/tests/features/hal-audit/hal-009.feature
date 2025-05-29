Feature: Fix for HAL-009

  Scenario: Unmodifiable claim_time prevents governance from adapting unstaking delays
    Given the contract is deployed with 3600 seconds claim_time
    Then the claim_time is 3600 seconds
    When Owner changes the claim_time to 7200 seconds
    Then the claim_time is 7200 seconds
    When Alice tries to change the claim_time to 1800 seconds
    Then the claim_time is still 7200 seconds