contract;

use interfaces::counter::Counter;

storage {
    counter: u64 = 0,
}

impl Counter for Contract {
    #[storage(write)]
    fn initialize_counter(value: u64) -> u64 {
        storage.counter.write(value);
        value
    }

    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64 {
        let incremented = storage.counter.read() + amount;
        storage.counter.write(incremented);
        incremented
    }

    #[storage(read)]
    fn get_count() -> u64 {
        storage.counter.read()
    }
}
