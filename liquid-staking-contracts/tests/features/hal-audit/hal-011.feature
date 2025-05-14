Feature: Fix for HAL-011

  # This scenario is now not possible, as we don't allow withdrawing funds
  Scenario: Incorrect sCSPR-to-CSPR conversion when no backing stake or supply exists
    # Given Alice has 1000 CSPR
    # When Alice stakes 1000 CSPR
    # And Owner removes everything from the pool
    # Then 0 CSPR is staked
    # And Alice cannot unstake 1000 sCSPR because there's no backing for redemption