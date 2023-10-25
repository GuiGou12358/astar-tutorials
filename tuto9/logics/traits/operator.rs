#[ink::trait_definition]
pub trait Operator {

    #[ink(message)]
    fn compute(&self, left: u128, right: u128) -> Result<u128, OperatorError>;

}

#[derive(Debug, Eq, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OperatorError {
    AddOverFlow,
    SubOverFlow
}