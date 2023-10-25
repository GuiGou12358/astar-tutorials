#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod calculator {

    use addition::{*};
    use subtraction::{*};

    use ink::env::call::{ExecutionInput, Selector};

    #[ink(storage)]
    pub struct Calculator {
        /// The 'addition' smart contract
        addition_contract: AdditionRef,
        /// The 'subtraction' smart contract
        subtraction_contract: SubtractionRef,
    }

    #[derive(Debug, Eq, PartialEq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CalculatorError {
        AdditionError(addition::OperatorError),
        SubtractionError(subtraction::OperatorError),
    }

    /// convertor from addition::OperatorError to CalculatorError
    impl From<addition::OperatorError> for CalculatorError {
        fn from(error: addition::OperatorError) -> Self {
            CalculatorError::AdditionError(error)
        }
    }
/*
    /// convertor from subtraction::OperatorError to CalculatorError
    impl From<subtraction::OperatorError> for CalculatorError {
        fn from(error: subtraction::OperatorError) -> Self {
            CalculatorError::SubtractionError(error)
        }
    }
*/

    impl Calculator {

        #[ink(constructor)]
        pub fn new(
            version: u32,
            addition_code_hash: Hash,
            subtraction_code_hash: Hash,
        ) -> Self {

            let salt = version.to_le_bytes();

            let addition_contract = AdditionRef::new()
                .code_hash(addition_code_hash)
                .endowment(0)
                .salt_bytes(salt)
                .instantiate();

            let subtraction_contract = ink::env::call::build_create::<SubtractionRef>()
                .code_hash(subtraction_code_hash)
                .gas_limit(0)
                .endowment(0)
                .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!("new"))))
                .salt_bytes(salt)
                .returns::<SubtractionRef>()
                .instantiate();

            Self {
                addition_contract,
                subtraction_contract,
            }
        }

        #[ink(message)]
        pub fn operate_add(&self, left: u128, right: u128) -> Result<u128, CalculatorError> {
            let result = self.addition_contract.compute(left, right)?;
            Ok(result)
        }

        #[ink(message)]
        pub fn operate_sub(&self, left: u128, right: u128) -> Result<u128, CalculatorError> {
            let result = self.subtraction_contract.compute(left, right)?;
            Ok(result)
        }

        #[ink(message)]
        pub fn operate_with_any_contract(&self, address: AccountId, left: u128, right: u128) -> Result<u128, CalculatorError> {

            ink::env::call::build_call::<Environment>()
                .call(address)
                .gas_limit(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("Operator::compute")))
                        .push_arg(left)
                        .push_arg(right)
                )
                .returns::<Result<u128, CalculatorError>>()
                .invoke()

        }


    }


}
