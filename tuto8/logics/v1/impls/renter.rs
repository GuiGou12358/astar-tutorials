use ink::storage::Mapping;
use openbrush::traits::AccountId;
use openbrush::traits::Storage;

pub use crate::traits::renter::*;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    owners: Mapping<CarId, AccountId>,
    leaseholders: Mapping<CarId, AccountId>,
}

impl<T> Renter for T
where
    T: Storage<Data>,
{
    default fn add_car(&mut self, car_id: CarId, owner: AccountId) -> Result<(), Error> {
        match self.data().owners.get(&car_id) {
            None => self.data().owners.insert(&car_id, &owner),
            _ => return Err(Error::CarAlreadyAdd),
        };
        Ok(())
    }

    default fn rent(&mut self, car_id: CarId) -> Result<(), Error> {
        let caller = Self::env().caller();

        if self.data().owners.get(&car_id).is_none() {
            return Err(Error::NotExistingCar);
        }

        match self.data().leaseholders.get(&car_id) {
            Some(_) => return Err(Error::AlreadyRent),
            None => {
                // save the leaseholder
                self.data().leaseholders.insert(&car_id, &caller);
            }
        };
        Ok(())
    }

    default fn give_back(&mut self, car_id: CarId) -> Result<(), Error> {
        let caller = Self::env().caller();

        match self.data().leaseholders.get(&car_id) {
            None => return Err(Error::NotRented),
            Some(leaseholder) => {
                if leaseholder != caller {
                    return Err(Error::BadLeaseholder);
                }
                // remove the leaseholder
                self.data().leaseholders.remove(&car_id);
            }
        };

        Ok(())
    }
}

#[cfg(test)]
#[openbrush::contract]
mod tests {
    use crate::impls::renter::*;
    use openbrush::test_utils::{accounts, change_caller};
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyContract {
        #[storage_field]
        renter: Data,
    }

    impl Renter for MyContract {}

    impl MyContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self::default();
            instance
        }
    }

    #[ink::test]
    fn test() {
        let accounts = accounts();
        let mut contract = MyContract::new();

        let car1_id: CarId = accounts.bob;

        contract
            .add_car(car1_id, accounts.django)
            .expect("Error when adding a car");

        change_caller(accounts.bob);

        contract.rent(car1_id).expect("Error when renting the car");

        contract
            .give_back(car1_id)
            .expect("Error when giving back the car");
    }
}
