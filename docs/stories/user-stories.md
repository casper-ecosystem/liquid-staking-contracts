### User stories

1. As a user, I should be able to sign in with CSPR.click so that a wallet is connected to the application.
1. As a user, I should be able to see the current rate sCSPR/CSPR.
1. As a user, I should be able to see my current sCSPR balance and its CSPR equivalence value.

1. As a user, I should be able to stake CSPR so that I get sCSPR in return.
    1. As a user, I should be able to enter an amount of CSPR to stake so that the application shows an sCPSR value approximation to get after execution.
    1. As the StakeCSPR app, I should raise an error if the user tries to stake less than 500 CSPR.
    1. As the StakeCSPR app, I should verify the user has enough CSPR to submit the transaction.
    1. As a user, I should be able to approve and deploy the stake transaction via CSPR.click, so that it's executed.
    1. As the StakeCSPR app, I should monitor stake transaction execution, so that when the results are available they're displayed to the user.
    1. As a user, I should be able to see the execution results and my new sCSPR balance.

1. As an sCSPR holder, I should be able to initiate an unstake CSPR operation, so that I get a receipt id in return.
    1. As a user, I should be able to enter an amount of sCSPR so that the application shows a CSPR value approximation to get after unstake is completed.
    1. As the StakeCSPR app, I should verify the user has enough sCSPR to exchange as well as CSPR to pay for the fees to submit the transaction.
    1. As a user, I should be able to approve and deploy the unstake transaction via CSPR.click, so that it's executed.
    1. As the StakeCSPR app, I should monitor unstake transaction execution, so that when the results are available the receipt id is displayed to the user.

1. As a user, I should be able to claim the unstaked CSPR after 7 eras using the receipt id.
    1. As a user, I should be able to enter the receipt id got in a previous unstake transaction
    1. As the StakeCSPR app, I should lookup the receipt id and display the operation details.
    1. As the StakeCSPR app, I should not permit the user to initiate the claim if 7 eras has not passed yet.
    1. As a user, I should be able to approve and deploy the claim transaction via CSPR.click, so that it's executed.
    1. As the StakeCSPR app, I should monitor claim transaction execution, so that when the results are available the CSPR amount transferred is displayed to the user.

1. As an external smart contract, I should be able to call the StakeCSPR smart contract to get the current rate sCSPR/CSPR.

TBD

1. As a StakeCSPR smart contract admin, I should be able to add CSPR to the delegated amount, so that the sCSPR/CSPR rate increases.

