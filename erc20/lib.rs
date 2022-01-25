#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use ink_storage::{
        collections::{HashMap},
        lazy::Lazy
    };

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Lazy<Balance>,
        balances: HashMap<AccountId, Balance>,
        allowances: HashMap<(AccountId, AccountId), Balance>, // AccountId1 对 AccountId2 的授权金额
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        value: Balance
    }

    #[derive(PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature="std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientApproval,
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = HashMap::new();
            balances.insert(caller, supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: supply
            });

            Self {
                total_supply: Lazy::from(supply) ,
                balances,
                allowances: HashMap::new()
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, origin: AccountId) -> Balance {
            self.balances.get(&origin).copied().unwrap_or(0)
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).copied().unwrap_or(0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
            let caller = self.env().caller();
            self.inner_transfer(caller, to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), Error> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);
            self.env().emit_event(Approval {
                from: Some(owner),
                spender: Some(spender),
                value
            });

            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), Error> {
            let caller = self.env().caller();
            let allow_balance = self.allowance(from, caller);
            if allow_balance < value {
                return Err(Error::InsufficientApproval);
            }
            self.inner_transfer(from, to, value)?;
            self.allowances.insert((from, caller), allow_balance - value);
            Ok(())
        }

        fn inner_transfer(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), Error> {
            let from_balance = self.balance_of(from);
            let to_balance = self.balance_of(to);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, from_balance - value);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value
            });
            Ok(())
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;
        // snip...
        #[ink::test]
        fn new_works() {
            let contract = Erc20::new(777);
            assert_eq!(contract.total_supply(), 777);
        }
    }
}
