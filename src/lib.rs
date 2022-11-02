pub mod ft;

use ft::FtContract;
use std::default;

use near_contract_standards::fungible_token::core::ext_ft_core;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_contract_standards::fungible_token::resolver::ext_ft_resolver;
use near_contract_standards::fungible_token::{
    core::FungibleTokenCore, metadata::FungibleTokenMetadata, FungibleToken,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{
    env, ext_contract, log, near_bindgen, require, AccountId, PanicOnDefault, Promise,
    PromiseResult,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AmmDemo {
    token_a: AccountId,
    token_b: AccountId,
    owner: AccountId,
    balance_a: u128,
    balance_b: u128,
}

#[near_bindgen]
impl AmmDemo {
    #[init]
    pub fn new(owner_id: AccountId, token_a: AccountId, token_b: AccountId) -> Self {
        Self {
            token_a,
            token_b,
            owner: owner_id,
            balance_a: 0,
            balance_b: 0,
        }
    }

    #[payable]
    pub fn swap(&mut self, token_in: AccountId, amount_in: u128) -> Promise {
        // ext_ft_core::ext(env::current_account_id()).ft_transfer(
        //     env::signer_account_id(),
        //     U128(amount_in),
        //     Some("uniswap out".to_string()),
        // );

        require!(
            token_in == self.token_a || token_in == self.token_b,
            "TOKEN_NOT_SUPPORTED"
        );

        let (balance_in, balance_out) = if token_in == self.token_a {
            (self.balance_a, self.balance_b)
        } else {
            (self.balance_b, self.balance_a)
        };
        let token_out = if token_in == self.token_a {
            self.token_b.clone()
        } else {
            self.token_a.clone()
        };
        // let caller = env::signer_account_id();
        // let contract_account_id = env::current_account_id();
        require!(amount_in < balance_in, "INSUFFICIENT_OUTPUT_AMOUNT");
        // if amount_a_out > 0 {
        //     safe_transfer(self.token_a.clone(), caller.clone(), amount_a_out);
        // }
        // if amount_b_out > 0 {
        //     safe_transfer(self.token_b.clone(), caller.clone(), amount_b_out);
        // }

        let invariant = self.balance_a * self.balance_b;
        let new_balance_in = balance_in + amount_in;
        let new_balance_out = invariant / new_balance_in;

        let amount_out = balance_out - new_balance_out;

        if token_in == self.token_a {
            self.balance_a = new_balance_in;
            self.balance_b = new_balance_out;
            log!(
                "Swap {} token A to {} token B, Invariant is {}",
                amount_in,
                amount_out,
                invariant
            );
            ext_ft_core::ext(self.token_b.clone())
                .with_attached_deposit(1)
                .ft_transfer(
                    env::signer_account_id(),
                    U128(amount_out),
                    Some("uniswap out".to_string()),
                )
        } else {
            self.balance_a = new_balance_out;
            self.balance_b = new_balance_in;
            log!(
                "Swap {} token B to {} token A, Invariant is {}",
                amount_in,
                amount_out,
                invariant
            );
            ext_ft_core::ext(self.token_b.clone())
                .with_attached_deposit(1)
                .ft_transfer(
                    env::signer_account_id(),
                    U128(amount_out),
                    Some("uniswap out".to_string()),
                )
        }
    }

    #[payable]
    pub fn add_liquidity(&mut self, token: AccountId, amount_in: u128) {
        require!(
            env::predecessor_account_id() == self.owner,
            "NOT_CONTRACT_OWNER"
        );
        require!(
            token == self.token_a || token == self.token_b,
            "TOKEN_NOT_SUPPORTED"
        );
        if token == self.token_a {
            self.balance_a += amount_in;
            log!("Added {} token A", amount_in);
        } else {
            self.balance_b += amount_in;
            log!("Added {} token B", amount_in);
        }
    }

    pub fn get_balance_a(&self) -> u128 {
        self.balance_a
    }

    pub fn get_balance_b(&self) -> u128 {
        self.balance_b
    }

    pub fn get_token_a(&self) -> AccountId {
        self.token_a.clone()
    }

    pub fn get_token_b(&self) -> AccountId {
        self.token_b.clone()
    }

    pub fn get_invariant(&self) -> u128 {
        self.balance_a * self.balance_b
    }
}

impl FungibleTokenReceiver for AmmDemo {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> near_sdk::PromiseOrValue<U128> {
        log!(
            "Received {:?} tokens {} from {}, message is {}",
            amount,
            env::predecessor_account_id(),
            sender_id,
            msg
        );
        if sender_id == self.owner {
            if self.token_a == env::predecessor_account_id() {
                self.balance_a += amount.0;
                near_sdk::PromiseOrValue::Value(U128::from(0))
            } else if self.token_b == env::predecessor_account_id() {
                self.balance_b += amount.0;
                near_sdk::PromiseOrValue::Value(U128::from(0))
            } else {
                near_sdk::PromiseOrValue::Value(amount)
            }
        } else {
            near_sdk::PromiseOrValue::Value(amount)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::serde_json::from_str;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};
    use near_sdk::{AccountId, Gas, MockedBlockchain, PublicKey};

    #[test]
    fn test_swap() {
        let mut context = VMContextBuilder::new();
        context.predecessor_account_id(accounts(0));
        testing_env!(context.build());

        let amm_owner_id = accounts(0);
        let token_a_account = accounts(1);
        let token_b_account = accounts(2);
        let amm_contract_id = accounts(3);

        // Deploying Token A
        testing_env!(context
            .current_account_id(token_a_account.clone())
            .signer_account_id(amm_owner_id.clone())
            .predecessor_account_id(amm_owner_id.clone())
            .build());
        let mut token_a_contract = FtContract::new(
            accounts(0),
            U128::from(1000000000000000),
            FungibleTokenMetadata {
                spec: "ft-1.0.0".to_string(),
                name: "Test Token A".to_string(),
                symbol: "TESTA".to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals: 8,
            },
        );

        // Deploying Token B
        testing_env!(context
            .current_account_id(token_b_account.clone())
            .signer_account_id(amm_owner_id.clone())
            .predecessor_account_id(amm_owner_id.clone())
            .build());
        let mut token_a_contract = FtContract::new(
            accounts(0),
            U128::from(1000000000000000),
            FungibleTokenMetadata {
                spec: "ft-1.0.0".to_string(),
                name: "Test Token B".to_string(),
                symbol: "TESTB".to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals: 8,
            },
        );

        // Deploying SwapContract
        testing_env!(context
            .current_account_id(amm_contract_id.clone())
            .signer_account_id(amm_owner_id.clone())
            .predecessor_account_id(amm_owner_id.clone())
            .build());
        let mut demo_contract = AmmDemo::new(
            amm_owner_id.clone(),
            token_a_account.clone(),
            token_b_account.clone(),
        );

        let init_amount_a = demo_contract.balance_a;
        let init_amount_b = demo_contract.balance_b;
        demo_contract.add_liquidity(token_a_account, 2000);
        demo_contract.add_liquidity(token_b_account, 4000);
        assert_eq!(init_amount_a + 2000, demo_contract.get_balance_a());
        assert_eq!(init_amount_b + 4000, demo_contract.get_balance_b());
        assert_eq!(
            demo_contract.get_invariant(),
            (init_amount_a + demo_contract.get_balance_a())
                * (init_amount_b + demo_contract.get_balance_b())
        );
    }
}
