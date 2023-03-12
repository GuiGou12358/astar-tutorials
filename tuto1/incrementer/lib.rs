#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod incrementer {

    #[ink(storage)]
    pub struct Incrementer {
        value: i32,
    }

    #[ink(event)]
    pub struct Incremented {
        by: i32,
        new_value: i32,
        who: AccountId,
    }


    impl Incrementer {

        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self { value: init_value }
        }


        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
            ink::env::debug_println!("incerment by {}, new value {}", by, self.value);
            let signer = self.env().caller();
            self.env().emit_event(Incremented{ by, new_value: self.value, who: signer });
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }
    }

}
