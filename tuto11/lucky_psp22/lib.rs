#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP22, PSP22Metadata, PSP22Mintable, PSP22Burnable, PSP22Capped, Ownable)]
#[openbrush::contract]
mod lucky_psp22 {

    use ink::codegen::Env;
    use ink::codegen::EmitEvent;
    use openbrush::contracts::psp22;
    use openbrush::contracts::psp22::extensions::metadata;
    use openbrush::contracts::psp22::extensions::capped;
    use openbrush::contracts::psp22::extensions::mintable;
    use openbrush::contracts::ownable::*;
    use openbrush::modifiers;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct LuckyPsp22 {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        capped: capped::Data,
    }

    #[ink(event)]
    pub struct Psp22Transfer {
        contract_id: AccountId,
        from: Option<AccountId>,
        to: Option<AccountId>,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Psp22Approval {
        contract_id: AccountId,
        owner: AccountId,
        spender: AccountId,
        amount: Balance,
    }

    impl LuckyPsp22 {
        #[ink(constructor)]
        pub fn new(
            name: Option<String>,
            symbol: Option<String>,
            decimal: u8,
            initial_supply: Balance,
            cap: Balance,
        ) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();

            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&decimal);

            ownable::Internal::_init_with_owner(&mut instance, caller);
            capped::Internal::_init_cap(&mut instance, cap).expect("Should cap the total supply");
            mintable::PSP22Mintable::mint(&mut instance, caller, initial_supply)
                .expect("Should mint initial supply");

            instance
        }
    }

    #[default_impl(PSP22Mintable)]
    #[modifiers(ownable::only_owner)]
    fn mint() {}

    #[default_impl(PSP22Burnable)]
    #[modifiers(ownable::only_owner)]
    fn burn() {}

    #[overrider(psp22::Internal)]
    fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, amount: Balance) {
        let contract_id = self.env().account_id();
        self.env().emit_event(Psp22Transfer{contract_id, from, to, amount});
    }

    #[overrider(psp22::Internal)]
    fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
        let contract_id = self.env().account_id();
        self.env().emit_event(Psp22Approval { contract_id, owner, spender, amount });
    }

}
