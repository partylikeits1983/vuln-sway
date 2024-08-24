library;

abi Counter {
    #[storage(write)]
    fn initialize_counter(value: u64) -> u64;

    #[storage(write)]
    fn initialize_owner(new_owner: Identity) -> Identity;

    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64;

    #[storage(read)]
    fn get_count() -> u64;
}
