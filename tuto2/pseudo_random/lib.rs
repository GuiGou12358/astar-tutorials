#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod pseudo_random {

    use ink::env::hash::{Keccak256, HashOutput};
    use ink::prelude::vec::Vec;
    use ink::env::debug_println;

    #[ink(storage)]
    pub struct PseudoRandom {
        salt: u64,
    }

    #[derive(Debug, Eq, PartialEq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum PseudoRandomError {
        DivByZero,
        MulOverFlow,
        AddOverFlow,
        SubOverFlow,
    }

    impl PseudoRandom {

        #[ink(constructor)]
        pub fn new(salt: u64) -> Self {
            Self { salt }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self { salt: 1 }
        }

        #[ink(message)]
        pub fn get_pseudo_random(&mut self, min: u128, max: u128) -> Result<u128, PseudoRandomError> {
            let seed = self.env().block_timestamp();
            let salt = self.salt;
            let mut input: Vec<u8> = Vec::new();
            input.extend_from_slice(&seed.to_be_bytes());
            input.extend_from_slice(&salt.to_be_bytes());
            let mut output = <Keccak256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Keccak256>(&input, &mut output);
            self.salt = salt + 1;

            let a = output[0] as u128;

            //(a  as u32) * (max - min) / (u32::MAX) + min
            let b = max.checked_sub(min).ok_or(PseudoRandomError::SubOverFlow)?;
            let c = a.checked_mul(b).ok_or(PseudoRandomError::MulOverFlow)?;
            let d = c.checked_div(u8::MAX as u128).ok_or(PseudoRandomError::DivByZero)?;
            let e = d.checked_add(min).ok_or(PseudoRandomError::AddOverFlow)?;

            debug_println!("random {}", e);

            Ok(e)
        }

        #[ink(message)]
        pub fn get_salt(&mut self) -> u64 {
            self.salt
        }

    }

    #[cfg(test)]
    mod tests {

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn test_pseudo_random() {

            let mut pseudo_random = PseudoRandom::new(7);
            assert_eq!(7, pseudo_random.get_salt());

            match pseudo_random.get_pseudo_random(1, 10){
                Ok(r) => assert!(r >= 1 && r <= 10),
                Err(_) => panic!("Error when generate the random number!")
            }
            assert_eq!(8, pseudo_random.get_salt());

            match pseudo_random.get_pseudo_random(10, 100){
                Ok(r) => assert!(r >= 10 && r <= 100),
                Err(_) => panic!("Error when generate the random number!")
            }
            assert_eq!(9, pseudo_random.get_salt());

        }
    }

}
