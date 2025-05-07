Feature: Contract is configurable

  Scenario: Owner can set the fee_percentage
    Given Owner is the contract owner
    And the fee_percentage is 1000
    When Owner sets the fee_percentage to 2000
    Then the fee_percentage is 2000

  Scenario: Non-owner cannot set the fee_percentage
    Given Alice is not the contract owner
    Then Alice cannot set the fee_percentage to 2000

  Scenario: Owner cannot set the fee_percentage above the maximum
    Given Owner is the contract owner
    Then Owner cannot set the fee_percentage to 10001

  Scenario: Owner can set the claim_time
    Given the contract is deployed with 3600 seconds claim_time
    And the claim_time is 3600 seconds
    When Owner changes the claim_time to 2000 seconds  
    Then the claim_time is 2000 seconds

  Scenario: Non-owner cannot set the claim_time
    Given Alice is not the contract owner
    Then Alice cannot set the claim_time to 2000 seconds

  Scenario: Owner can set the min_stake
    Given the contract is deployed with 500 CSPR min_stake
    And the min_stake is 500 CSPR
    When Owner changes the min_stake to 1000 CSPR
    Then the min_stake is 1000 CSPR

  Scenario: Non-owner cannot set the min_stake
    Given Alice is not the contract owner
    Then Alice cannot set the min_stake to 1000 CSPR
