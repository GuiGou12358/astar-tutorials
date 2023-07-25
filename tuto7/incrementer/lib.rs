#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod incrementer {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Incrementer {
        value: i32,
        updates_by_user: Mapping<AccountId, u32>,
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
            Self {
                value: init_value,
                updates_by_user: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            // update the value
            self.value += by;
            // get the caller
            let signer = self.env().caller();
            // set the number of updates for this user
            let nb_updates = self.updates_by_user.get(&signer).unwrap_or(0);
            self.updates_by_user.insert(&signer, &(nb_updates + 1));
            // emit the event
            self.env().emit_event(Incremented {
                by,
                new_value: self.value,
                who: signer,
            });
        }

        #[ink(message)]
        pub fn get_value(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn get_nb_updates(&self) -> u32 {
            // get the caller
            let signer = self.env().caller();
            self.updates_by_user.get(&signer).unwrap_or(0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use openbrush::test_utils::{accounts, change_caller};

        #[test]
        fn test_constructor() {
            let accounts = accounts();
            let contract = Incrementer::new(10);
            assert_eq!(10, contract.get_value());
        }

        #[ink::test]
        fn test() {
            let accounts = accounts();
            let mut contract = Incrementer::new(10);

            // alice increments the value
            change_caller(accounts.alice);
            assert_eq!(0, contract.get_nb_updates());
            contract.inc(3);
            assert_eq!(13, contract.get_value());
            assert_eq!(1, contract.get_nb_updates());

            // bob increments the value
            change_caller(accounts.bob);
            assert_eq!(0, contract.get_nb_updates());
            contract.inc(5);
            assert_eq!(18, contract.get_value());
            assert_eq!(1, contract.get_nb_updates());
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn test(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // given
            let constructor = IncrementerRef::new(10);
            let contract_id = client
                .instantiate("incrementer", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // when alice increments the value
            let inc_value = build_message::<IncrementerRef>(contract_id.clone())
                .call(|contract| contract.inc(3));
            client
                .call(&ink_e2e::alice(), inc_value, 0, None)
                .await
                .expect("inc failed");

            // then : the value has been updated
            let get_value = build_message::<IncrementerRef>(contract_id.clone())
                .call(|contract| contract.get_value());
            let result = client
                .call_dry_run(&ink_e2e::alice(), &get_value, 0, None)
                .await;
            assert_eq!(13, result.return_value());

            // the number of updates for alice as well
            let get_nb_updates = build_message::<IncrementerRef>(contract_id.clone())
                .call(|contract| contract.get_nb_updates());
            let result = client
                .call_dry_run(&ink_e2e::alice(), &get_nb_updates, 0, None)
                .await;
            assert_eq!(1, result.return_value());

            Ok(())
        }
    }
}
