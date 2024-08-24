contract;

use interfaces::counter::Counter;

storage {
    counter: u64 = 0,
    authorized: Identity = Identity::Address(Address::zero()),
}

pub enum AuthError {
    Unauthorized: Identity,
}

impl Counter for Contract {
    #[storage(write)]
    fn initialize_counter(value: u64) -> u64 {
        storage.counter.write(value);
        value
    }

    #[storage(write)]
    fn initialize_owner(new_owner: Identity) -> Identity {
        /*         
        if (storage.authorized.read() == Identity::Address(Address::zero())) {
            storage.authorized.write(new_owner);
        } else {
            let sender = msg_sender().unwrap();
            require(sender == storage.authorized.read(), AuthError::Unauthorized(sender));
        } 
        */
        storage.authorized.write(new_owner);

        new_owner
    }

    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64 {
        let sender = msg_sender().unwrap();
        require(
            sender == storage
                .authorized
                .read(),
            AuthError::Unauthorized(sender),
        );

        let incremented = storage.counter.read() + amount;
        storage.counter.write(incremented);
        incremented
    }

    #[storage(read)]
    fn get_count() -> u64 {
        storage.counter.read()
    }
}
