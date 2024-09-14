contract;

mod errors;

use interfaces::vault::Vault;
use std::{asset::transfer, context::msg_amount, hash::Hash, call_frames::msg_asset_id};

use errors::DepositError;

configurable {
    ASSET_ID: AssetId = AssetId::zero(),
}

storage {
    balances: StorageMap<Identity, u64> = StorageMap::<Identity, u64> {},
    vaults: StorageMap<ContractId, bool> = StorageMap::<ContractId, bool> {},
}

impl Vault for Contract {
    #[payable]
    #[storage(read, write)]
    fn deposit(receiver: Identity) -> u64 {
        let amount: u64 = msg_amount();
        let asset = msg_asset_id();

        require(asset == ASSET_ID, DepositError::InvalidAsset);

        storage.balances.insert(receiver, amount);

        return amount;
    }

    #[storage(read, write)]
    fn withdraw(receiver: Identity, amount: u64) -> u64 {
        let debitor = msg_sender().unwrap();
        let balance = storage.balances.get(debitor).try_read().unwrap();

        let new_balance = balance - amount;

        if new_balance > 0 {
            storage.balances.insert(debitor, new_balance);

            transfer(receiver, AssetId::base(), amount);
        } else {
            revert(0);
        }

        return amount;
    }
}
