## Devnet contract address: 
```JavaScript
"erd1qqqqqqqqqqqqqpgq7v6rrxlgwrdgw3gkk9j69mkw6kvdrxg93xaqfentpc"
```

#### Deploy Transaction: 
```erdpy contract deploy --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --pem=wallets/users/carol.pem --gas-limit=50000000 --bytecode=output/egld-esdt-swap.wasm --send```

## Endpoints:

#### issue_wrapped_egld - only owner
`erdpy contract call erd1qqqqqqqqqqqqqpgq7v6rrxlgwrdgw3gkk9j69mkw6kvdrxg93xaqfentpc --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=issueWrappedEgld --arguments str:Cgld str:CGLD --gas-limit=75000000 --value=50000000000000000 --pem=wallets/users/carol.pem --send`

#### set_local_roles - only owner
`erdpy contract call erd1qqqqqqqqqqqqqpgq7v6rrxlgwrdgw3gkk9j69mkw6kvdrxg93xaqfentpc --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=setLocalRoles --gas-limit=75000000 --value=0 --pem=wallets/users/carol.pem --send`

#### wrap_egld - payable
`erdpy contract call erd1qqqqqqqqqqqqqpgq7v6rrxlgwrdgw3gkk9j69mkw6kvdrxg93xaqfentpc --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=wrapEgld --gas-limit=75000000 --value=1000000000000000000 --pem=wallets/users/carol.pem --send`

#### unwrap_egld - payable
Translated

`ESDTTransfer@CGLD-447ee4@1000000000000000000@unwrapEgld`

`ESDTTransfer@43474C442D343437656534@31303030303030303030303030303030303030@756E7772617045676C64`
> Even no of chars in an hex encoded number should be prefixed with '0'

`erdpy tx new --chain=D --proxy=https://devnet-gateway.elrond.com --receiver=erd1qqqqqqqqqqqqqpgq7v6rrxlgwrdgw3gkk9j69mkw6kvdrxg93xaqfentpc --data=ESDTTransfer@43474C442D343437656534@0DE0B6B3A7640000@756E7772617045676C64 --recall-nonce --gas-limit=75000000 --pem=wallets/users/carol.pem --send`

#### get_locked_egld_balance - view
`erdpy contract call erd1qqqqqqqqqqqqqpgq7v6rrxlgwrdgw3gkk9j69mkw6kvdrxg93xaqfentpc --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=getLockedEgldBalance --gas-limit=75000000 --pem=wallets/users/carol.pem --send`

### wrapped_egld_token_id - view
`erdpy contract call erd1qqqqqqqqqqqqqpgq7v6rrxlgwrdgw3gkk9j69mkw6kvdrxg93xaqfentpc --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=getWrappedEgldTokenIdentifier --gas-limit=75000000 --pem=wallets/users/carol.pem --send`