```shell
near view larry.testnet get_balance_a
near view larry.testnet get_balance_b

near call test-token-a.testnet storage_deposit --accountId larry.testnet --amount 0.00125
near call test-token-a.testnet ft_transfer '{"receiver_id": "larry.testnet", "amount": "200"}' --accountId test-token-a.testnet --depositYocto 1
near call larry.testnet add_liquidity '{"token": "test-token-a.testnet", "amount_in": 200}' --accountId larryal.testnet

near call test-token-b.testnet storage_deposit --accountId larry.testnet --amount 0.00125
near call test-token-b.testnet ft_transfer '{"receiver_id": "larry.testnet", "amount": "2000"}' --accountId test-token-b.testnet --depositYocto 1
near call larry.testnet add_liquidity '{"token": "test-token-b.testnet", "amount_in": 2000}' --accountId larryal.testnet

near call test-token-a.testnet storage_deposit --accountId user-test-amm-1.testnet --amount 0.00125
near call test-token-b.testnet storage_deposit --accountId user-test-amm-1.testnet --amount 0.00125

near call test-token-b.testnet ft_transfer '{"receiver_id": "larry.testnet", "amount": "40"}' --accountId test-token-b.testnet --depositYocto 1
near call larry.testnet swap '{"token_in": "test-token-b.testnet", "amount_in": 40}' --accountId user-test-amm-1.testnet --depositYocto 1
```
