use openbrush::traits::{AccountId, Balance};

#[derive(Debug, Eq, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    NotExistingCar,
    CarAlreadyAdd,
    AlreadyRent,
    NotRented,
    BadLeaseholder,
    InsufficientValue,
    TransferError,
}

pub type CarId = AccountId;

#[openbrush::trait_definition]
pub trait Renter {
    #[ink(message)]
    fn add_car(&mut self, car_id: CarId, owner: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn update_rental_price(
        &mut self,
        car_id: CarId,
        rental_price: Option<Balance>,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn get_leaseholder(&self, car_id: CarId) -> Result<Option<AccountId>, Error>;

    #[ink(message)]
    fn is_rent(&self, car_id: CarId) -> Result<bool, Error>;

    #[ink(message, payable)]
    fn rent(&mut self, car_id: CarId) -> Result<(), Error>;

    #[ink(message)]
    fn give_back(&mut self, car_id: CarId) -> Result<(), Error>;
}
