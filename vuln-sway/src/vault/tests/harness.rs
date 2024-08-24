use fuels::tx::TxParameters;
use fuels::{prelude::*, types::ContractId};

abigen!(Contract(
    name = "Vault",
    abi = "src/vault/out/debug/vault-abi.json"
),);

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

async fn get_vault_contract_instance() -> (Vault<WalletUnlocked>, ContractId) {
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

    let instance = Vault::new(id, wallet.clone());

    (instance.clone(), instance.contract_id().into())
}

#[tokio::test]
async fn deploy_and_use_incrementor() {
    // First deploy the Counter contract and get its ID
    let (_vault_instance, counter_id) = get_vault_contract_instance().await;

    println!("vault {:?}", counter_id);
}
