## Devnet contract address: 
```JavaScript
"erd1qqqqqqqqqqqqqpgqpz4gqyvh0eua6qzyysre4k03d3jwn7ylu7wqpgcn5q"
```

#### Deploy Transaction: 
```erdpy contract deploy --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --pem=wallets/alpha-users/cornelius.pem --gas-limit=70000000 --bytecode=output/elrond_wyvern_project_smart_contract.wasm --arguments str:bafybeialptdt5rhojbeuadp5wlofynmdwbekrk3vu5s6xd2pwupchb7gbu str:bafybeihocx2c7llprvyauw3jhgvaz63wkoebjjjsacrn2gpve3q32kqhem 10000 3 1000 1 --send```

#### Issue NFT Token Tx (TBD Token Name & ACRONIM)
```erdpy contract call erd1qqqqqqqqqqqqqpgqpz4gqyvh0eua6qzyysre4k03d3jwn7ylu7wqpgcn5q --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=issueToken --arguments str:Sapiens str:SAP --gas-limit=75000000 --value=50000000000000000 --pem=wallets/alpha-users/cornelius.pem --send```

#### Set NFT Local Roles Tx
```erdpy contract call erd1qqqqqqqqqqqqqpgqpz4gqyvh0eua6qzyysre4k03d3jwn7ylu7wqpgcn5q --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=setLocalRoles --gas-limit=75000000 --pem=wallets/alpha-users/cornelius.pem --send```

#### Populate indexes
erdpy contract call erd1qqqqqqqqqqqqqpgqpz4gqyvh0eua6qzyysre4k03d3jwn7ylu7wqpgcn5q --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=populateIndexes --arguments 1000 --gas-limit=75000000 --pem=wallets/alpha-users/cornelius.pem --send

#### Mint NFT Tx
```erdpy contract call erd1qqqqqqqqqqqqqpgqpz4gqyvh0eua6qzyysre4k03d3jwn7ylu7wqpgcn5q --chain=D --proxy=https://devnet-gateway.elrond.com --recall-nonce --function=mint --arguments 1 --value=1 --gas-limit=75000000 --pem=wallets/alpha-users/cornelius.pem --send```


### `erdpy` commands
