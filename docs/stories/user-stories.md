## User stories

### RWY Protocol Web Application

1. As the RWY Web Application, I must have a header with logo and CSPR.click navigation top bar.
1. As a user, I should be able to sign in with CSPR.click so that a wallet is connected to the application.

#### Dashboard

1. As the Dashboard screen, I must show current epoch number, with real-time TVL and target TVL.
1. As the Dashboard screen, I must show next epoch number, with target TVL, and points rewards.
1. As the Dashboard screen, I must show points rewards for the connected account, with a breakdown in daily rewards and projected annual rewards.
1. As the Dashboard screen, I must show the user's connected account portfolio value.
1. As the Dashboard screen, I must show a chart with user's connected account revenue over time. 
1. As the Dashboard screen, I must show a chart with user's connected account points balance over time. 

#### Portfolio

1. As the Portfolio screen, I should have a list of assets in the connected account that includes native CSPR token, sCSPR, and all other CEP-18 tokens.
1. As the CSPR native token row in the portfolio, I should display the TVL in the platform, and the account balance,, and a Stake button.
1. As the CSPR native token row in the portfolio, I should display a Stake button that links to the staking screen.
1. As the sCSPR token row in the portfolio, I should display the TVL in the platform, the account balance, and current point multiplier.
1. As the sCSPR token row in the portfolio, I should display a button 'Unstake/Claim' that links to the staking screen with Unstake tab active.
1. As a CEP-18 listed token row in the portfolio, I should display the TVL in the platform, the account balance, and current point multiplier, if any. 
1. As a CEP-18 unlisted token row in the portfolio, I should display only the account balance. 

#### Stake

1. As the Stake screen, I should display three tabs for Stake, Unstake, and Claim transactions.
1. As the Stake tab, I should display the account's CSPR balance with fiat equivalent amount.
1. As the Stake tab, I should display the current sCSPR/CSPR rate, the current APY/APR.

1. As a user, I should be able to stake CSPR so that I get sCSPR in return.
    1. As a user, I should be able to enter an amount of CSPR to stake so that the application shows an sCPSR value approximation to get after execution.
    1. As the Stake tab, I should raise an error if the user tries to stake less than 500 CSPR.
    1. As the Stake tab, I should verify the user has enough CSPR to submit the transaction, and display an error otherwise.
    1. As a user, I should be able to approve and deploy the stake transaction via CSPR.click, so that it's send to the network for execution.
    1. As the Stake tab, I should display a pending execution status, so that the user knows she needs to wait for the results.
    1. As the Stake tab, I should display a completed execution status which includes CSPR amount staked, sCSPR amount get in return, and the transaction fees.

1. As an sCSPR holder, I should be able to initiate an unstake CSPR operation, so that I get a receipt id in return.
    1. As the Unstake tab, I should clearly indicate the user that after sending the unstake request he must wait 12 eras (24 hrs) to claim the unstaked CSPR.
    1. As a user, I should be able to enter an amount of sCSPR so that the application shows a CSPR value approximation to get after the unstake is completed.
    1. As the Unstake tab, I should verify the user has enough sCSPR to exchange as well as CSPR to pay for the fees to submit the transaction, and display an error otherwise.
    1. As a user, I should be able to approve and deploy the unstake transaction via CSPR.click, so that it's send to the network for execution.
    1. As the Unstake tab, I should display a pending execution status, so that the user knows she needs to wait for the results.
    1. As the Unstake tab, I should display a completed execution status which includes sCSPR amount sent, CSPR amount unstaked, the claim/receipt id, the claim availability timestamp, and the transaction fees.

1. As a user, I should be able to claim the unstaked CSPR after 12 eras (24 hrs) using the receipt id.
    1. As the Claim tab, I should list all pending claim operations, with the Unstake request timestamp, the sCPSR sent, and the CSPR that will be withdrawn.
    1. As the Claim tab, I should have a Claim now button for every available claim operation.
    1. As the Claim tab, I should have a countdown timer for every pending claim that shows when the operation will be available.  
    1. As a user, I should be able to click on Claim now, and approve the transaction via CSPR.click, so that it's send to the network for execution.
    1. As the StakeCSPR app, I should monitor claim transaction execution, so that when the results are available the CSPR amount transferred is displayed to the user.
    1. As the Claim tab, I should display a 'No pending claims' text when no claim operation is pending.
    1. As the Claim tab, I should display a 'List completed claims in the last 30 days' link, so that the user can see completed claims.
    1. As a completed claim operation row, I should display to the user unstake and claim timestamps, deploy hashes, and links to CSPR.live deploy pages, as well as the sCSPR sent and the CSPR withdrawn. 

#### Third-party smart contract interaction with StakeCSPR contract

   1. As an external smart contract, I should be able to call the StakeCSPR smart contract to get the current rate sCSPR/CSPR.
   1. As an external smart contract, I should be able to call the StakeCSPR smart contract to stake CSPR, so that I get sCSPR in return.
   1. As an external smart contract, I should be able to call the StakeCSPR smart contract to unstake sCSPR, so that I get a receipt id in return.
   1. As an external smart contract, I should be able to claim the unstaked CSPR after 12 eras using the receipt id.

### Admin interfaces (not for web app)

#### sCSPR value increase

   1. As a StakeCSPR smart contract admin, I should be able to add CSPR to the delegated amount, so that the sCSPR/CSPR rate increases.
