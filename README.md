# A simple AMM contract with the following methods:  
## Initialization method
>Input is the address of the contract owner and the addresses of two tokens (token A and token B).
>The method requests and stores the metadata of tokens (name, decimals)  
>Creates wallets for tokens А & В.

## View method
>The method for getting information about the contract (ticker, decimals, ratio of tokens A and B)

## Deposit method
>The user can transfer a certain number of tokens A to the contract account and in return must receive a certain number of tokens B (similarly in the other direction). The contract supports a certain ratio of tokens A and B. X * Y = K (K is some constant value, X and Y are the number of tokens A and B respectively.

## Add liquidity method
>The owner of the contract can transfer a certain amount of tokens A or B to the contract account, thereby changing the ratio K.

# Steps Guide
## Setup Token A
```
deposit scheme pulse lucky feed barely weather color velvet bunker trophy virus
```
1. Initialization
```
near call test-token-a.testnet new '{"owner_id": "'test-token-a.testnet'", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Test Token A", "symbol": "TESTA", "decimals": 8 }}' --accountId test-token-a.testnet
```
2. Transfer
```shell
near call test-token-a.testnet ft_transfer '{"receiver_id": "user.testnet", "amount": "200"}' --accountId test-token-a.testnet --depositYocto 1
```
* If Error reported, then you need to register first
```
Smart contract panicked: The account user.testnet is not registered
```
3. Register
```shell
near call test-token-a.testnet storage_deposit --accountId user.testnet --depositYocto 1
```
## Setup Token B
```
dumb earth inject vendor salad glass near crazy vague connect hood radio
```
1. Initialization
```shell
near call test-token-b.testnet new '{"owner_id": "'test-token-b.testnet'", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Test Token B", "symbol": "TESTB", "decimals": 8 }}' --accountId test-token-b.testnet
```

## Setup AMM contract：
```
punch rally scrap client dream industry egg unfold cart tilt chaos kite
```
## Setup Test User
```
flavor forget tape sponsor main artefact stamp belt endless best erosion crane
```

* View balance of two tokens

```shell
near view user.testnet get_balance_a
near view user.testnet get_balance_b
```

## Bring everything together
1. Register user with both tokens

```
near call test-token-a.testnet storage_deposit --accountId user.testnet --amount 0.00125
near call test-token-b.testnet storage_deposit --accountId user.testnet --amount 0.00125
```

2. Transfer 2000 Token A to user ，and add liquidity

```
near call test-token-a.testnet ft_transfer '{"receiver_id": "user.testnet", "amount": "200"}' --accountId test-token-a.testnet --depositYocto 1
near call user.testnet add_liquidity '{"token": "test-token-a.testnet", "amount_in": 200}' --accountId owner.testnet
```

3. Transfer 2000 Token B to user ，and add liquidity

```shell
near call test-token-b.testnet ft_transfer '{"receiver_id": "user.testnet", "amount": "2000"}' --accountId test-token-b.testnet --depositYocto 1
near call user.testnet add_liquidity '{"token": "test-token-b.testnet", "amount_in": 2000}' --accountId owner.testnet
```

4. Register another user `user-test-amm-1` with both tokens

```shell
near call test-token-a.testnet storage_deposit --accountId user-test-amm-1.testnet --amount 0.00125
near call test-token-b.testnet storage_deposit --accountId user-test-amm-1.testnet --amount 0.00125
```

5. Transfer 2000 Token B to test user
```shell
near call test-token-b.testnet ft_transfer '{"receiver_id": "user.testnet", "amount": "40"}' --accountId test-token-b.testnet --depositYocto 1
```

6. Execute Swap
```
near call user.testnet swap '{"token_in": "test-token-b.testnet", "amount_in": 40}' --accountId user-test-amm-1.testnet --depositYocto 1
```