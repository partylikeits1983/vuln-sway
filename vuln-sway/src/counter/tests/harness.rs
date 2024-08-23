use fuels::{prelude::*, types::ContractId};

// Load ABI from JSON
abigen!(Contract(
    name = "Counter",
    abi = "src/counter/out/debug/counter-abi.json"
));

async fn get_contract_instance() -> (Counter<WalletUnlocked>, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None, // Option<ChainConfig> - no custom configuration
        None, // Third argument for chain config
    )
    .await
    .expect("Failed to launch custom provider and get wallets");

    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from(
        "./out/debug/counter.bin",
        LoadConfiguration::default().with_storage_configuration(
            StorageConfiguration::new(false, vec![]), // Use `new` instead of `load_from`
        ),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default()) // Use `TxParameters` here instead of `CallParameters`
    .await
    .unwrap();

    let instance = Counter::new(id, wallet); // Convert id to `ContractId` using `.into()`

    (instance.clone(), instance.contract_id().into())
}

#[tokio::test]
async fn initialize_and_increment() {
    let (contract_instance, _id) = get_contract_instance().await;
    // Now you have an instance of your contract you can use to test each function

    let result = contract_instance
        .methods()
        .initialize_counter(42)
        .call()
        .await
        .unwrap();

    assert_eq!(42, result.value);

    // Call `increment_counter()` method in our deployed contract.
    let result = contract_instance
        .methods()
        .increment_counter(10)
        .call()
        .await
        .unwrap();

    assert_eq!(52, result.value);

    println!("value: {:?}", result.value);
}
