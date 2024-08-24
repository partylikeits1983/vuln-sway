contract;

use interfaces::vault::Vault;
use std::{asset::transfer, context::msg_amount, hash::Hash};

storage {
    balances: StorageMap<Identity, u64> = StorageMap::<Identity, u64> {},
    vaults: StorageMap<ContractId, bool> = StorageMap::<ContractId, bool> {},
}

impl Vault for Contract {
    #[payable]
    #[storage(read, write)]
    fn deposit(receiver: Identity, vault_sub_id: SubId) -> u64 {
        let amount: u64 = msg_amount();

        storage.balances.insert(receiver, amount);

        return amount;
    }

    #[storage(read, write)]
    fn withdraw(
        receiver: Identity,
        underlying_asset: AssetId,
        vault_sub_id: SubId,
    ) -> u64 {
        let amount = storage.balances.get(receiver).try_read().unwrap();

        if amount > 0 {
            transfer(receiver, AssetId::base(), amount);
        } else {
            revert(0);
        }

        return amount;
    }
}
