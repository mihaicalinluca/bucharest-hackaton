### Devnet 

- cd interaction
- source devnet.snippets.sh

## Deployment

- deployForTest  

Use deployForTest & upgradeForTest in order to set ONE_DAY to 60 seconds

## Issue token

- issueTokenNew

## Stake tokens

- stakeHacktoken 

## Unstake tokens

- unstakeStakedTokens

## Claim rewards (after min 1 day/ 60 seconds in dev)

- claimRewards

## Merge stakes

- mergeStakes

Use this function to merge your staked tokens.
You receive a new staked token for each stake in a new day/ 60 sec in dev

## Compute RPS

- computeRps 

This endpoint will be called by the python bot once per day/ 60 sec in dev to compound

# Already deployed DEV SC ADDRESS: erd1qqqqqqqqqqqqqpgqu6hqeva4sv3gm24e450sgatnpwled2r30yts0ewynn

Run the python bot with python main.py --proxy https://devnet-gateway.multiversx.com --pem devWallet.pem