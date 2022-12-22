#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Incrementer {
        owner: AccountId,
        contract_value: i32,
        account_values: ink_storage::Mapping<AccountId, i32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
          ink_lang::utils::initialize_contract(|contract: &mut Self| {
            let caller = Self::env().caller();
            contract.owner = caller;
            contract.contract_value = init_value;
            contract.account_values.insert(&caller, &0);
          })
        }

        pub fn default() -> Self {
          ink_lang::utils::initialize_contract(|contract: &mut Self| {
              contract.contract_value = Default::default();
          })
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
          self.owner
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
          self.contract_value
        }

        #[ink(message)]
        pub fn inc(&mut self) {
          self.contract_value += 1
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            self.account_values.get(&self.env().caller()).unwrap_or_default()
        }

        #[ink(message)]
        pub fn inc_mine(&mut self) {
            let curr = self.account_values.get(&self.env().caller()).unwrap_or_default();
            self.account_values.insert(&self.env().caller(), &(curr + 1));
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn my_value_works() {
          let contract = Incrementer::new(11);
          assert_eq!(contract.get(), 11);
          assert_eq!(contract.get_mine(), 0);
        }
    }
}
