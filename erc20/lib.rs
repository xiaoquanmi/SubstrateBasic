#![cfg_attr(not(feature = "std"), no_std)]

// use scale::{Encode, Decode};

#[ink::contract]
mod erc20 {
    // use ink::primitives::AccountId;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        approval: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval{
        owner: AccountId,
        apender: AccountId,
        value: Balance
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            // let sender = Self::env().sender();
            let sender = Self::env().caller();

            balances.insert(&sender, &total_supply);

            Self::env().emit_event(
                Transfer{
                    from: AccountId::default(),
                    to: sender,
                    value: total_supply
                }
            );

            Self {
                total_supply,
                balances,
                approval: Default::default(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            self.balances.get(&who).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value:Balance) -> core::result::Result<(), Error> {
            // let from = self.env().sender();
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            if from_balance < value {
                // return Err("insufficient balance".to_owned());
                return Err(Error::InsufficientBalance)
            }

            self.balances.insert(&from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(&to, &(to_balance + value));

            Self::env().emit_event(
                Transfer{
                    from,
                    to,
                    value,
                }
            );

            Ok(())
        }
    }
}
