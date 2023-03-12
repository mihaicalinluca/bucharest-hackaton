DEVWALLET="../../devWallet.pem"
ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)
WASM_PATH="/mnt/d/the-bucharest-hackaton/dynamic-staking-auto-compound/output/dynamic-staking-auto-compound.wasm"
PROXY="https://devnet-gateway.multiversx.com"
DEV_CHAIN_ID="D"
SC_ADDRESS_HEX="00000000000000000500e6ae0cb3b583228daab9ad1f0475730bbf96a8717917"

deploy() {
    mxpy --verbose contract deploy \
    --bytecode=${WASM_PATH} \
    --recall-nonce \
    --pem=${DEVWALLET} \
    --gas-limit=500000000 \
    --arguments 0x3799a0e6bdd0e69d80 0x04a175 0x632a5400 0x00 0x015180 0x015180 0x544f4b454e2d31323233 \
    --metadata-payable \
    --proxy=${PROXY} \
    --chain=${DEV_CHAIN_ID} \
    --send || return

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deployForTest() {
    mxpy --verbose contract deploy \
    --bytecode=${WASM_PATH} \
    --recall-nonce \
    --pem=${DEVWALLET} \
    --gas-limit=500000000 \
    --arguments 0x3799a0e6bdd0e69d80 0x04a175 0x632a5400 0x00 0x3c 0x3c 0x544f4b454e2d31323233 \
    --metadata-payable \
    --proxy=${PROXY} \
    --chain=${DEV_CHAIN_ID} \
    --send || return

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}


upgrade() {
    mxpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgqu6hqeva4sv3gm24e450sgatnpwled2r30yts0ewynn \
    --bytecode=${WASM_PATH} \
    --recall-nonce \
    --pem=${DEVWALLET} \
    --gas-limit=500000000 \
    --arguments 0x3799a0e6bdd0e69d80 0x04a175 0x632a5400 0x00 0x015180 0x015180 0x4841434b544f4b454e2d323863393230 \
    --metadata-payable \
    --proxy=${PROXY} \
    --chain=${DEV_CHAIN_ID} \
    --send || return
}

upgradeForTest() {
    mxpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgqu6hqeva4sv3gm24e450sgatnpwled2r30yts0ewynn \
    --bytecode=${WASM_PATH} \
    --recall-nonce \
    --pem=${DEVWALLET} \
    --gas-limit=500000000 \
    --arguments 0x3799a0e6bdd0e69d80 0x04a175 0x632a5400 0x00 0x3c 0x3c 0x4841434b544f4b454e2d323863393230 \
    --metadata-payable \
    --proxy=${PROXY} \
    --chain=${DEV_CHAIN_ID} \
    --send || return
}


# issueToken() {
#     mxpy --verbose tx new \
#     --receiver=erd1qqqqqqqqqqqqqpgqu6hqeva4sv3gm24e450sgatnpwled2r30yts0ewynn \
#     --chain=D --proxy=https://devnet-gateway.multiversx.com \
#     --gas-limit=60000000 \
#     --value=50000000000000000 \
#     --pem=${DEVWALLET} \
#     --recall-nonce \
#     --data=issueToken@5374616b6564546f6b656e@53544f4b454e@4d795374616b6564546f6b656e \
#     --send 
# }

# issueToken@StakedToken@STOKEN
# 0.05 egld for token issuance

issueTokenNew() {
    mxpy --verbose tx new \
    --receiver=erd1qqqqqqqqqqqqqpgqu6hqeva4sv3gm24e450sgatnpwled2r30yts0ewynn \
    --chain=D --proxy=https://devnet-gateway.multiversx.com \
    --gas-limit=60000000 \
    --value=50000000000000000 \
    --pem=${DEVWALLET} \
    --recall-nonce \
    --data=issueToken@5354414b4544544f4b454e32@53544f4b454e32 \
    --send 
}

# setLocalRoles() {
#     mxpy --verbose tx new \
#     --receiver=erd1qqqqqqqqqqqqqpgqu6hqeva4sv3gm24e450sgatnpwled2r30yts0ewynn \
#     --chain=D --proxy=https://devnet-gateway.multiversx.com \
#     --gas-limit=60000000 \
#     --pem=${DEVWALLET} \
#     --recall-nonce \
#     --data=setLocalRoles \
#     --send 
# }

computeRPS() {
    mxpy --verbose tx new \
    --receiver=erd1qqqqqqqqqqqqqpgqu6hqeva4sv3gm24e450sgatnpwled2r30yts0ewynn \
    --chain=D --proxy=https://devnet-gateway.multiversx.com \
    --gas-limit=60000000 \
    --pem=${DEVWALLET} \
    --recall-nonce \
    --data=computeRps \
    --send 
}

stakeHacktoken() {
    mxpy --verbose tx new \
    --receiver=erd1qqqqqqqqqqqqqpgqu6hqeva4sv3gm24e450sgatnpwled2r30yts0ewynn \
    --chain=D --proxy=https://devnet-gateway.multiversx.com \
    --gas-limit=60000000 \
    --pem=${DEVWALLET} \
    --recall-nonce \
    --data=ESDTTransfer@4841434b544f4b454e2d323863393230@056bc75e2d63100000@7374616b65 \
    --send 
}

unstakeStakedToken() {
    mxpy --verbose tx new \
    --receiver=erd10x2dcvqxvgf8urkaanl7cak4ynhewjt8q5xgl0kssngjnjp40ytssdfd8k \
    --chain=D --proxy=https://devnet-gateway.multiversx.com \
    --gas-limit=60000000 \
    --pem=${DEVWALLET} \
    --recall-nonce \
    --data=ESDTNFTTransfer@53544f4b454e322d613636353436@01@056bc75e2d63100000@00000000000000000500e6ae0cb3b583228daab9ad1f0475730bbf96a8717917@756e7374616b65 \
    --send 
}


claimRewards() {
    mxpy --verbose tx new \
        --receiver=erd10x2dcvqxvgf8urkaanl7cak4ynhewjt8q5xgl0kssngjnjp40ytssdfd8k \
        --chain=D --proxy=https://devnet-gateway.multiversx.com \
        --gas-limit=60000000 \
        --pem=${DEVWALLET} \
        --recall-nonce \
        --data=ESDTNFTTransfer@53544f4b454e322d613636353436@04@056bc75e2d63100000@00000000000000000500e6ae0cb3b583228daab9ad1f0475730bbf96a8717917@636c61696d \
        --send 

}

# Data: "ESDTTransfer" +
#           "@" + <token identifier in hexadecimal encoding> +
#           "@" + <value to transfer in hexadecimal encoding>
#           "@" + <contract method>


mergeStakes() {
     mxpy --verbose tx new \
        --receiver=erd10x2dcvqxvgf8urkaanl7cak4ynhewjt8q5xgl0kssngjnjp40ytssdfd8k \
        --chain=D --proxy=https://devnet-gateway.multiversx.com \
        --gas-limit=60000000 \
        --pem=${DEVWALLET} \
        --recall-nonce \
        --data=MultiESDTNFTTransfer@00000000000000000500e6ae0cb3b583228daab9ad1f0475730bbf96a8717917@02@53544f4b454e322d613636353436@05@056bc75e2d63100000@53544f4b454e322d613636353436@06@056bc75e2d63100000@6d65726765 \
        --send 
}

# "MultiESDTNFTTransfer" +
#           "@" + <receiver bytes in hexadecimal encoding> +
#           "@" + <number of tokens to transfer in hexadecimal encoding> +
#           "@" + <token 0 identifier in hexadecimal encoding> +
#           "@" + <token 0 nonce in hexadecimal encoding> +
#           "@" + <token 0 quantity to transfer in hexadecimal encoding> +
#           "@" + <token 1 identifier in hexadecimal encoding> +
#           "@" + <token 1 nonce in hexadecimal encoding> +
#           "@" + <token 1 quantity to transfer in hexadecimal encoding> +  
#           "@" + <method to be called in SC>