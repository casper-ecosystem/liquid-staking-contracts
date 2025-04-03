Feature: CEP18 Features

    Scenario: Is initialized correctly
        Then name is "Staked CSPR"
        And symbol is "sCSPR"
        And decimals is 9
        And total supply is 0 sCSPR

    Scenario: Transfer of sCSPR tokens allows unstaking by the new owner
        Given Alice has 1000 CSPR
        When Alice stakes 1000 CSPR
        Then Alice's token balance is 1000 sCSPR
        And Alice's CSPR balance is 0 CSPR
        When Alice transfers 500 sCSPR to Bob
        Then Alice's token balance is 500 sCSPR
        And Bob's token balance is 500 sCSPR
        When Bob unstakes 500 sCSPR
        Then Bob's token balance is 0 sCSPR
        And unbonding period passes
        When Bob claims unstakes
        Then Bob has 500 CSPR
        And total supply is 500 sCSPR

    Scenario: Transfer of sCSPR tokens with rewards collection
        Given Alice has 1000 CSPR
        When Alice stakes 1000 CSPR
        And Owner adds 1000 CSPR to the pool
        Then staked CSPR is 2000 CSPR
        When Alice transfers 1000 sCSPR to Bob
        Then Bob's token balance is 1000 sCSPR
        When Bob unstakes 1000 sCSPR
        And unbonding period passes
        And Bob claims unstakes
        Then Bob has 2000 CSPR
        And total supply is 0 sCSPR

    Scenario: Multiple transfers of sCSPR tokens
        Given Alice has 1000 CSPR
        When Alice stakes 1000 CSPR
        And Alice transfers 300 sCSPR to Bob
        And Alice transfers 300 sCSPR to Charlie
        Then Alice's token balance is 400 sCSPR
        And Bob's token balance is 300 sCSPR
        And Charlie's token balance is 300 sCSPR
        When Bob unstakes 150 sCSPR
        And Charlie unstakes 300 sCSPR
        And Bob transfers 150 sCSPR to Dave
        Then Bob's token balance is 0 sCSPR
        And Dave's token balance is 150 sCSPR
        When unbonding period passes
        And Bob claims unstakes
        And Charlie claims unstakes
        And Dave unstakes 150 sCSPR
        And unbonding period passes
        And Dave claims unstakes
        Then Bob has 150 CSPR
        And Charlie has 300 CSPR
        And Dave has 150 CSPR
        And total supply is 400 sCSPR
