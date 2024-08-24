use fuels::tx::TxParameters;
use fuels::{prelude::*, types::ContractId};

abigen!(
    Contract(
        name = "Counter",
        abi = "src/counter/out/debug/counter-abi.json"
    ),
    Contract(
        name = "Incrementor",
        abi = "src/incrementor/out/debug/incrementor-abi.json"
    ),
);

// Declare the global wallet variable
static mut WALLET: Option<WalletUnlocked> = None;

async fn initialize_wallet() -> WalletUnlocked {
    // Launch a local network and initialize the wallet
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

    wallets.pop().unwrap() // Return the initialized wallet
}

async fn get_wallet() -> &'static WalletUnlocked {
    unsafe {
        if WALLET.is_none() {
            WALLET = Some(initialize_wallet().await);
        }
        WALLET.as_ref().unwrap()
    }
}

async fn get_counter_contract_instance() -> (Counter<WalletUnlocked>, ContractId) {
    let wallet = get_wallet().await;

    let id = Contract::load_from(
        "../../src/counter/out/debug/counter.bin",
        LoadConfiguration::default().with_storage_configuration(
            StorageConfiguration::new(false, vec![]), // Use `new` instead of `load_from`
        ),
    )
    .unwrap()
    .deploy(wallet, TxPolicies::default()) // Use `TxParameters` here instead of `CallParameters`
    .await
    .unwrap();

    let instance = Counter::new(id, wallet.clone());

    (instance.clone(), instance.contract_id().into())
}

async fn get_incrementor_contract_instance(
    counter_id: ContractId,
) -> (Incrementor<WalletUnlocked>, ContractId) {
    let wallet = get_wallet().await;

    // Configure the Incrementor contract with the Counter contract ID
    let configurables = IncrementorConfigurables::default()
        .with_COUNTER_ID(counter_id)
        .unwrap();

    let id = Contract::load_from(
        "../../src/incrementor/out/debug/incrementor.bin",
        LoadConfiguration::default()
            .with_configurables(configurables) // Use the configurables with Counter ID
            .with_storage_configuration(StorageConfiguration::new(false, vec![])),
    )
    .unwrap()
    .deploy(wallet, TxPolicies::default()) // Use `TxParameters` here instead of `CallParameters`
    .await
    .unwrap();

    let instance = Incrementor::new(id, wallet.clone());

    (instance.clone(), instance.contract_id().into())
}

#[tokio::test]
async fn deploy_and_use_incrementor() {
    // First deploy the Counter contract and get its ID
    let (counter_instance, counter_id) = get_counter_contract_instance().await;

    println!("counterID {:?}", counter_id);

    // Initialize the counter to ensure it's set to a known value
    let _ = counter_instance
        .methods()
        .initialize_counter(0)
        .call()
        .await
        .unwrap();

    // Now deploy the Incrementor contract using the Counter contract ID
    let (incrementor_instance, _incrementor_id) =
        get_incrementor_contract_instance(counter_id).await;

    // Verify the counter contract ID from the incrementor contract
    let incrementor_counter_id = incrementor_instance
        .methods()
        .get_counter_contract_id()
        .call()
        .await
        .unwrap()
        .value;

    println!(
        "Incrementor contract is linked to counterID {:?}",
        incrementor_counter_id
    );
    assert_eq!(counter_id, incrementor_counter_id);

    // Now increment the counter through the Incrementor contract
    let result = incrementor_instance
        .methods()
        .increment_other_contract(counter_id, 5)
        .with_contracts(&[&counter_instance]) // Ensure cross-contract interaction
        .call()
        .await
        .unwrap();

    println!("Counter increment result: {:?}", result.value);

    // Check the final counter value directly from the Counter contract
    let counter_value = counter_instance
        .methods()
        .get_count()
        .call()
        .await
        .unwrap()
        .value;
    println!("Final counter value: {:?}", counter_value);

    // Ensure the counter was incremented by the correct amount
    assert_eq!(counter_value, 5);
}
