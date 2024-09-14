contract;

use interfaces::{vault::Vault, vault_caller::VaultCaller};

use std::context::balance_of;

configurable {
    ASSET_ID: AssetId = AssetId::zero(),
    VAULT_ID: ContractId = ContractId::zero(),
}

impl VaultCaller for Contract {
    #[storage(read, write)]
    fn call_vault_contract(vault_id: ContractId, amount: u64) -> u64 {
        // Create a handler for the TestContract
        let contract_id: b256 = vault_id.into();

        let vault = abi(Vault, contract_id);

        let balance = balance_of(ContractId::this(), ASSET_ID);

        let asset_id = b256::from(ASSET_ID);

        let amount = vault.deposit {
            gas: 1_000_000,
            asset_id: asset_id,
            coins: 3000,
        }(Identity::ContractId(ContractId::this()));

        amount
    }

    #[storage(read)]
    fn get_counter_contract_id() -> ContractId {
        let vault_id = VAULT_ID;
        vault_id
    }
}
