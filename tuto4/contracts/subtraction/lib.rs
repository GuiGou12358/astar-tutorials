#![cfg_attr(not(feature = "std"), no_std)]


pub use self::subtraction::{Subtraction, SubtractionRef};
pub use logics::traits::operator::{Operator, OperatorError};

#[ink::contract]
mod subtraction {

    use ink::env::debug_println;
    pub use logics::traits::operator::{Operator, OperatorError};

    #[ink(storage)]
    pub struct Subtraction {}

    impl Subtraction {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

    }

    impl Operator for Subtraction {

        #[ink(message)]
        fn compute(&self, left: u128, right: u128) -> Result<u128, OperatorError> {
            let result = left.checked_sub(right).ok_or(OperatorError::SubOverFlow)?;
            debug_println!("{} - {} = {}", left, right, result);
            Ok(result)
        }

    }

    #[cfg(test)]
    mod tests {

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn test_operate() {

            let contract = Subtraction::new();

            match contract.compute(5, 10){
                Ok(_) => panic!("This operation should return an error!"),
                Err(e) => assert_eq!(OperatorError::SubOverFlow, e)
            };

            match contract.compute(10, 5){
                Ok(r) => assert_eq!(5, r),
                Err(_) => panic!("Error when generate the operation!")
            }

        }
    }

}
