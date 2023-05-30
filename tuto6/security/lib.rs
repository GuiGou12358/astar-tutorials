#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod security {

    use openbrush::contracts::ownable::*;
    use openbrush::contracts::access_control::*;
    use openbrush::traits::Storage;

    const ROLE_1: RoleType = ink::selector_id!("ROLE_1");
    const ROLE_2: RoleType = ink::selector_id!("ROLE_2");

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct SecurityContract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access: access_control::Data,
    }

    impl Ownable for SecurityContract {}

    impl AccessControl for SecurityContract {}

    impl SecurityContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            // set the owner of this contract via the Ownable trait/default implementation
            instance._init_with_owner(caller);
            // set the admin of this contract via the AccessControl trait/default implementation
            instance._init_with_admin(caller);
            instance.grant_role(ROLE_1, caller).expect("Should grant the role ROLE_1");
            instance
        }

        #[ink(message)]
        pub fn for_every_body(&self)  {
            ink::env::debug_println!("The method 'for_every_body' has been called");
        }

        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn only_owner(&mut self) -> Result<(), OwnableError>{
            ink::env::debug_println!("The method 'only_owner' has been called");
            Ok(())
        }

        #[ink(message)]
        #[openbrush::modifiers(only_role(ROLE_1))]
        pub fn only_role_1(&mut self) -> Result<(), AccessControlError>{
            ink::env::debug_println!("The method 'only_role_1' has been called");
            Ok(())
        }

        #[ink(message)]
        #[openbrush::modifiers(only_role(ROLE_2))]
        pub fn only_role_2(&mut self) -> Result<(), AccessControlError>{
            ink::env::debug_println!("The method 'only_role_2' has been called");
            Ok(())
        }

        #[ink(message)]
        pub fn get_role_1(&self) -> RoleType {
            ROLE_1
        }

        #[ink(message)]
        pub fn get_role_2(&self) -> RoleType {
            ROLE_2
        }
    }

}




