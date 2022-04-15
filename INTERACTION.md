## Devnet contract address: 
```JavaScript
"erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2"
```

#### Deploy Transaction: 
```erdpy contract deploy --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --pem=wallets/newusers/blue.pem --gas-limit=80000000 --bytecode=output/egld-esdt-swap.wasm --send```

#### **`UpgradeContract`** Transaction
```erdpy contract upgrade erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2 --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --pem=wallets/newusers/blue.pem --gas-limit=80000000 --bytecode=output/egld-esdt-swap.wasm --send```

## Endpoints NFT:

#### **IssueToken** issue_token - only owner
`erdpy contract call erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2 --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=issueToken --arguments str:Sapiens str:SAP --gas-limit=75000000 --value=50000000000000000 --pem=wallets/newusers/blue.pem --send`

#### **SetNftLocalRoles** set_nft_local_roles - only owner
`erdpy contract call erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2 --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=setNftLocalRoles --gas-limit=75000000 --pem=wallets/newusers/blue.pem --send`


## Endpoints ESDT:

#### issue_wrapped_egld - only owner
`erdpy contract call erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2 --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=issueWrappedEgld --arguments str:Geld str:GELD --gas-limit=75000000 --value=50000000000000000 --pem=wallets/newusers/blue.pem --send`

#### set_local_roles - only owner
`erdpy contract call erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2 --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=setLocalRoles --gas-limit=75000000 --value=0 --pem=wallets/newusers/blue.pem --send`

#### wrap_egld - payable
`erdpy contract call erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2 --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=wrapEgld --gas-limit=75000000 --value=1000000000000000000 --pem=wallets/newusers/blue.pem --send`

#### unwrap_egld - payable
Translated

`ESDTTransfer@CGLD-447ee4@1000000000000000000@unwrapEgld`

`ESDTTransfer@43474C442D343437656534@31303030303030303030303030303030303030@756E7772617045676C64`
> Even no of chars in an hex encoded number should be prefixed with '0'

`erdpy tx new --chain=D --proxy=https://devnet-gateway.elrond.com --receiver=erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2 --data=ESDTTransfer@43474C442D343437656534@0DE0B6B3A7640000@756E7772617045676C64 --recall-nonce --gas-limit=75000000 --pem=wallets/newusers/blue.pem --send`

#### get_locked_egld_balance - view
`erdpy contract call erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2 --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=getLockedEgldBalance --gas-limit=75000000 --pem=wallets/newusers/blue.pem --send`

### wrapped_egld_token_id - view
`erdpy contract call erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2 --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=getWrappedEgldTokenIdentifier --gas-limit=75000000 --pem=wallets/newusers/blue.pem --send`


#### is_white_listed - view
`erdpy contract call erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2 --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=getWhitelistStatus --gas-limit=60000000 --pem=wallets/newusers/blue.pem --send`

#### setWhitelistStart - endpoints: Sets whitelist start to current timestamp.
`erdpy contract call erd1qqqqqqqqqqqqqpgq7vysekzk8s3c29yfnjr99avz27my9ghjrxesd762y2 --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=setWhitelistStart --gas-limit=50000000 --pem=wallets/newusers/blue.pem --send`