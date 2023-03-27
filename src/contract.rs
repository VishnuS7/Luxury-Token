use ink_lang::contract;

#[contract]
mod token {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    

    pub struct Token {
        pub name: String,
        pub symbol: String,
        pub total_supply: u128,
        pub balances: StorageHashMap<AccountId, u128>,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, total_supply: u128) -> Self {
            let mut balances = StorageHashMap::new();
            balances.insert(Self::env().caller(), total_supply);

            Self {
                name,
                symbol,
                total_supply,
                balances,
            }
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: u128) -> bool {
            let from = Self::env().caller();
            let from_balance = self.balances.get(&from).copied().unwrap_or_default();
            let to_balance = self.balances.get(&to).copied().unwrap_or_default();

            if from_balance < value {
                return false;
            }

            self.balances.insert(from, from_balance - value);
            self.balances.insert(to, to_balance + value);

            true
        }
    }
}
