#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::addition::{Addition, AdditionRef};
pub use logics::traits::operator::{Operator, OperatorError};

#[ink::contract]
mod addition {

    use ink::env::debug_println;
    use logics::traits::operator::{Operator, OperatorError};

    #[ink(storage)]
    pub struct Addition {}

    impl Addition {

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

    }

    impl Operator for Addition {

        #[ink(message)]
        fn compute(&self, left: u128, right: u128) -> Result<u128, OperatorError> {
            let result = left.checked_add(right).ok_or(OperatorError::AddOverFlow)?;
            debug_println!("{} + {} = {}", left, right, result);
            Ok(result)
        }

    }

    #[cfg(test)]
    mod tests {

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn test() {

            let addition = Addition::new();

            match addition.compute(5, 10){
                Ok(r) => assert_eq!(15, r),
                Err(_) => panic!("Error when generate the operation!")
            }

            match addition.compute(u128::MAX, 1){
                Ok(_) => panic!("This operation should return an error!"),
                Err(e) => assert_eq!(OperatorError::AddOverFlow, e)
            }
        }
    }

}
