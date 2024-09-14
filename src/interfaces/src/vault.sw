library;

abi Vault {
    #[payable]
    #[storage(read, write)]
    fn deposit(receiver: Identity) -> u64;

    #[storage(read, write)]
    fn withdraw(
        receiver: Identity,
        amount: u64, 
    ) -> u64;
}
