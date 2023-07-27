use openbrush::traits::AccountId;

#[derive(Debug, Eq, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    NotExistingCar,
    CarAlreadyAdd,
    AlreadyRent,
    NotRented,
    BadLeaseholder,
}

pub type CarId = AccountId;

#[openbrush::trait_definition]
pub trait Renter {
    #[ink(message)]
    fn add_car(&mut self, car_id: CarId, owner: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn rent(&mut self, car_id: CarId) -> Result<(), Error>;

    #[ink(message)]
    fn give_back(&mut self, car_id: CarId) -> Result<(), Error>;
}
