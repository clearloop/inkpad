#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod args {
    #[ink(storage)]
    pub struct Args {}

    impl Args {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn test_boolean(&mut self, v: bool) -> bool {
            v
        }

        #[ink(message)]
        pub fn test_number(&mut self, v: i32) -> i32 {
            v
        }

        #[ink(message)]
        pub fn test_hash(&mut self, v: Hash) -> Hash {
            v
        }
    }
}
