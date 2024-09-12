library;

abi Incrementor {
    #[storage(read, write)]
    fn increment_other_contract(counter_id: ContractId, amount: u64) -> u64;

    #[storage(read)]
    fn get_counter_contract_id() -> ContractId;
}
