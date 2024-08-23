contract;

use interfaces::{counter::Counter, incrementor::Incrementor};

configurable {
    COUNTER_ID: ContractId = ContractId::zero(),
}

impl Incrementor for Contract {
    #[storage(read, write)]
    fn increment_other_contract(counter_id: ContractId, amount: u64) -> u64 {
        // Create a handler for the TestContract
        let contract_id: b256 = counter_id.into();

        let handler = abi(Counter, contract_id);

        // Call the increment_counter function in the TestContract
        let new_count = handler.increment_counter(amount);

        log(new_count);

        new_count
    }

    #[storage(read)]
    fn get_counter_contract_id() -> ContractId {
        let counter_id = COUNTER_ID;
        counter_id
    }
}
