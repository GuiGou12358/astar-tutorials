#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Metadata, PSP34Mintable, PSP34Burnable, PSP34Enumerable, Ownable)]
#[openbrush::contract]
mod lucky_psp34 {

    use ink::codegen::Env;
    use ink::codegen::EmitEvent;
    use ink::prelude::string::ToString;
    use openbrush::contracts::psp34;
    use openbrush::contracts::psp34::extensions::metadata;
    use openbrush::contracts::psp34::extensions::enumerable;
    use openbrush::contracts::ownable::*;
    use openbrush::contracts::psp34::Id;
    use openbrush::modifiers;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct LuckyPsp34 {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        enumerable: enumerable::Data,
    }

    #[ink(event)]
    pub struct Psp34Transfer {
        contract_id: AccountId,
        from: Option<AccountId>,
        to: Option<AccountId>,
        id: Id,
    }

    #[ink(event)]
    pub struct Psp34Approval {
        contract_id: AccountId,
        id: Option<Id>,
        from: AccountId,
        to: AccountId,
        approved: bool,
    }

    impl LuckyPsp34 {
        #[ink(constructor)]
        pub fn new(
            id: Id,
            name: String,
            symbol: String,
            uri: String,
        ) -> Self {
            let mut instance = Self::default();

            metadata::Internal::_set_attribute(&mut instance, id.clone(), "name".to_string(), name);
            metadata::Internal::_set_attribute(&mut instance, id.clone(), "symbol".to_string(), symbol);
            metadata::Internal::_set_attribute(&mut instance, id.clone(), "uri".to_string(), uri);

            let caller = instance.env().caller();
            ownable::Internal::_init_with_owner(&mut instance, caller);

            instance
        }
    }

    #[default_impl(PSP34Mintable)]
    #[modifiers(ownable::only_owner)]
    fn mint() {}

    #[default_impl(PSP34Burnable)]
    #[modifiers(ownable::only_owner)]
    fn burn() {}

    #[overrider(psp34::Internal)]
    fn _emit_transfer_event(&self, id: Id, from: Option<AccountId>, to: Option<AccountId>) {
        let contract_id = self.env().account_id();
        self.env().emit_event(Psp34Transfer{contract_id, from, to, id});
    }


    #[overrider(psp34::Internal)]
    fn _emit_approval_event(&self, id: Option<Id>, from: AccountId, to: AccountId, approved: bool) {
        let contract_id = self.env().account_id();
        self.env().emit_event(Psp34Approval { contract_id, id, from, to, approved });
    }

}
