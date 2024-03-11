use near_workspaces::{types::NearToken, Account, Contract};
use serde_json::json;
use std::{env, fs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    println!("wasm_arg: {wasm_arg}");
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = near_workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    println!("wasm loaded {} bytes", wasm.len());
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount("alice")
        .initial_balance(NearToken::from_near(30))
        .transact()
        .await?
        .into_result()?;

    // begin tests
    test_default_message(&alice, &contract).await?;
    test_changes_message(&alice, &contract).await?;
    test_default_encryption_pub_key(&alice, &contract).await?;
    test_changes_encryption_pub_key(&alice, &contract).await?;
    test_increment(&alice, &contract).await?;
    test_multiply(&alice, &contract).await?;
    Ok(())
}

async fn test_default_message(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    let greeting: String = user
        .call(contract.id(), "get_greeting")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(greeting, "Hello".to_string());
    println!("      Passed ✅ gets default greeting");
    Ok(())
}

async fn test_changes_message(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    user.call(contract.id(), "set_greeting")
        .args_json(json!({"greeting": "Howdy"}))
        .transact()
        .await?
        .into_result()?;

    let greeting: String = user
        .call(contract.id(), "get_greeting")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(greeting, "Howdy".to_string());
    println!("      Passed ✅ changes greeting");
    Ok(())
}

async fn test_default_encryption_pub_key(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    let greeting: String = user
        .call(contract.id(), "get_encryption_pub_key")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(greeting, "".to_string());
    println!("      Passed ✅ gets default encryption public key");
    Ok(())
}

async fn test_changes_encryption_pub_key(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    user.call(contract.id(), "set_encryption_pub_key")
        .args_json(json!({"key": "1224142141asdas"}))
        .transact()
        .await?
        .into_result()?;

    let greeting: String = user
        .view(contract.id(), "get_encryption_pub_key")
        .args_json(json!({}))
        .await?
        .json()?;

    assert_eq!(greeting, "1224142141asdas".to_string());
    println!("      Passed ✅ changes encryption public key");
    Ok(())
}

async fn test_increment(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..3 {
        user.call(contract.id(), "increment_counter")
            .args_json(json!({}))
            .transact()
            .await?
            .into_result()?;
    }

    let counter: usize = user
        .call(contract.id(), "get_counter")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(counter, 3);
    println!("      Passed ✅ increments");
    Ok(())
}

async fn test_multiply(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    user.call(contract.id(), "multiply_counter")
        .args_json(json!({"val": 3}))
        .transact()
        .await?
        .into_result()?;

    let counter: usize = user
        .call(contract.id(), "get_counter")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(counter, 9);
    println!("      Passed ✅ multiples");
    Ok(())
}
