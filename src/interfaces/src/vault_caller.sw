library;

abi VaultCaller {
    #[storage(read, write)]
    fn call_vault_contract(counter_id: ContractId, amount: u64) -> u64;

    #[storage(read)]
    fn get_counter_contract_id() -> ContractId;
}
