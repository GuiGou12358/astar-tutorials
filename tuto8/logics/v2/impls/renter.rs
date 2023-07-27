use ink::storage::Mapping;
use openbrush::traits::{AccountId, Balance, Storage};

pub use crate::traits::renter::*;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    owners: Mapping<CarId, AccountId>,
    leaseholders: Mapping<CarId, AccountId>,
    rental_prices: Mapping<CarId, Balance>,
    received_rents: Mapping<(CarId, AccountId), Balance>,
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

    default fn update_rental_price(
        &mut self,
        car_id: CarId,
        rental_price: Option<Balance>,
    ) -> Result<(), Error> {
        if self.data().owners.get(&car_id).is_none() {
            return Err(Error::NotExistingCar);
        }

        match rental_price {
            Some(value) => {
                self.data().rental_prices.insert(&car_id, &value);
            }
            None => self.data().rental_prices.remove(&car_id),
        };
        Ok(())
    }


    fn get_leaseholder(&self, car_id: CarId) -> Result<Option<AccountId>, Error> {
        if self.data().owners.get(&car_id).is_none() {
            return Err(Error::NotExistingCar);
        }

        Ok(self.data().leaseholders.get(&car_id))
    }

    default fn is_rent(&self, car_id: CarId) -> Result<bool, Error> {
        let leaseholder = self.get_leaseholder(car_id)?;
        Ok(leaseholder.is_some())
    }

    default fn rent(&mut self, car_id: CarId) -> Result<(), Error> {
        let caller = Self::env().caller();
        let transferred_value = Self::env().transferred_value();

        if self.data().owners.get(&car_id).is_none() {
            return Err(Error::NotExistingCar);
        }

        match self.data().leaseholders.get(&car_id) {
            Some(_) => return Err(Error::AlreadyRent),
            None => {
                // save the leaseholder
                self.data().leaseholders.insert(&car_id, &caller);

                // get the rental price
                let rental_price = self.data().rental_prices.get(&car_id).unwrap_or(0);
                if transferred_value < rental_price {
                    // the transferred amount is not enough
                    return Err(Error::InsufficientValue);
                }
                if transferred_value > 0 {
                    // save the transferred value to be able to give back
                    self.data()
                        .received_rents
                        .insert(&(car_id, caller), &transferred_value);
                }
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

        // pay back the received rent
        if let Some(value) = self.data().received_rents.get(&(car_id, caller)) {
            Self::env()
                .transfer(caller, value)
                .map_err(|_| Error::TransferError)?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[openbrush::contract]
mod tests {
    use crate::impls::renter::*;
    use ink::env::pay_with_call;
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

        change_caller(accounts.bob);

        contract
            .add_car(car1_id, accounts.django)
            .expect("Error when adding a car");

        contract
            .update_rental_price(car1_id, Some(100))
            .expect("Error when updating the rental price");

        assert_eq!(
            false,
            contract
                .is_rent(car1_id)
                .expect("Error when check the renting")
        );

        pay_with_call!(contract.rent(car1_id), 100).expect("Error when renting the car");
        assert_eq!(
            true,
            contract
                .is_rent(car1_id)
                .expect("Error when check the renting")
        );

        contract
            .give_back(car1_id)
            .expect("Error when giving back the car");
        assert_eq!(
            false,
            contract
                .is_rent(car1_id)
                .expect("Error when check the renting")
        );
    }
}
