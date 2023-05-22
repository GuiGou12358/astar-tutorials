#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
pub mod dapps_staking_contract_example {

    use dapps_staking_extension::*;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct DAppsStakingContract {
        stakers: Mapping<AccountId, Balance>,
    }

    /// Errors occurred in the contract
    #[derive(Debug, Eq, PartialEq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ContractError {
        TransferError,
        AddOverFlow,
        SubOverFlow,
        DSError(DSError)
    }

    /// convertor from DSError to ContractError
    impl From<DSError> for ContractError {
        fn from(error: DSError) -> Self {
            ContractError::DSError(error)
        }
    }

    /// Event emitted when a value is staked
    #[ink(event)]
    pub struct Staked {
        #[ink(topic)]
        account: AccountId,
        era: u32,
        amount: Balance,
    }

    /// Event emitted when a value is unstaked
    #[ink(event)]
    pub struct Unstaked {
        #[ink(topic)]
        account: AccountId,
        era: u32,
        amount: Balance,
    }

    impl DAppsStakingContract {

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {stakers: Mapping::default()}
        }

        #[ink(message)]
        pub fn read_current_era(&self) -> u32 {
            DappsStaking::read_current_era()
        }

        #[ink(message)]
        pub fn get_staked_amount(&self, account: AccountId) -> Option<Balance> {
            self.stakers.get(&account)
        }


        /// read the amount staked on this contract by this contract
        #[ink(message)]
        pub fn read_staked_amount_on_contract(&self) -> Balance {
            let contract = self.env().account_id();
            // read the amount staked on this contract by this contract
            DappsStaking::read_staked_amount_on_contract(contract, contract)
        }

        /// read the total amount staked on this contract
        #[ink(message)]
        pub fn read_contract_stake(&self) -> Balance {
            let contract = self.env().account_id();
            DappsStaking::read_contract_stake(contract)
        }

        #[ink(message, payable)]
        pub fn bond_and_stake(&mut self) -> Result<(), ContractError> {

            let caller = self.env().caller();
            let value = self.env().transferred_value();

            // compute the new stake
            let new_stake = match self.stakers.get(&caller){
                Some(existing) => {existing.checked_add(value).ok_or(ContractError::AddOverFlow)?}
                _ => {value}
            };

            // save the new amount staked by the caller
            self.stakers.insert(&caller, &new_stake);

            // Stake on this contract.
            let contract = self.env().account_id();
            // Here the staker will be the contract for the pallet dAppStaking
            DappsStaking::bond_and_stake(contract, value)?;

            // get the current era
            let era = DappsStaking::read_current_era();

            // emmit the event
            self.env().emit_event(Staked { account: caller, era, amount: value });

            Ok(())
        }

        #[ink(message)]
        pub fn unbond_and_unstake(&mut self, value: Balance) -> Result<(), ContractError> {
            let caller = self.env().caller();

            // compute the new stake
            let new_stake = match self.stakers.get(&caller){
                Some(existing) => {existing.checked_sub(value).ok_or(ContractError::SubOverFlow)?}
                _ => {value}
            };

            // save the new amount staked by the caller
            if new_stake == 0 {
                self.stakers.remove(&caller);
            } else {
                self.stakers.insert(&caller, &new_stake);
            }

            // Unbond and unstake on the contract
            let contract = self.env().account_id();
            DappsStaking::unbond_and_unstake(contract, value)?;

            // get back the fund to the user but normally we should respect the unbounding period
            // it means this method could fail if the fund are still locked
            self.env().transfer(caller, value).map_err(|_| ContractError::TransferError)?;

            // get the current era
            let era = DappsStaking::read_current_era();

            // emmit the event
            self.env().emit_event(Unstaked { account: caller, era, amount: value });

            Ok(())
        }

        #[ink(message)]
        pub fn withdraw_unbonded(&mut self) -> Result<(), ContractError> {
            DappsStaking::withdraw_unbonded()?;
            Ok(())
        }


    }
}




