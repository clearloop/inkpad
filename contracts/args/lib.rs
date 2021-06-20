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

        #[ink(message)]
        pub fn test_boolean_and_number(&mut self, b: bool, n: i32) -> (bool, i32) {
            (b, n)
        }

        #[ink(message)]
        pub fn test_boolean_and_hash(&mut self, b: bool, h: Hash) -> (bool, Hash) {
            (b, h)
        }

        #[ink(message)]
        pub fn test_number_and_number(&mut self, a: i32, b: i32) -> (i32, i32) {
            (a, b)
        }

        #[ink(message)]
        pub fn test_number_and_hash(&mut self, n: i32, h: Hash) -> (i32, Hash) {
            (n, h)
        }

        #[ink(message)]
        pub fn test_hash_and_hash(&mut self, x: Hash, y: Hash) -> (Hash, Hash) {
            (x, y)
        }

        #[ink(message)]
        pub fn test_all(&mut self, b: bool, n: i32, h: Hash) -> (bool, i32, Hash) {
            (b, n, h)
        }
    }
}
