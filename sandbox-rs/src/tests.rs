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
    test_default_ipfs_cid(&alice, &contract).await?;
    test_changes_ipfs_cid(&alice, &contract).await?;
    test_default_encryption_pub_key(&alice, &contract).await?;
    test_changes_encryption_pub_key(&alice, &contract).await?;
    Ok(())
}

async fn test_default_ipfs_cid(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    let greeting: String = user
        .call(contract.id(), "get_ipfs_cid")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(greeting, "".to_string());
    println!("      Passed ✅ gets default ipfs_cid");
    Ok(())
}

async fn test_changes_ipfs_cid(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    user.call(contract.id(), "set_ipfs_cid")
        .args_json(json!({"ipfs_cid": "Howdy"}))
        .transact()
        .await?
        .into_result()?;

    let ipfs_cid: String = user
        .view(contract.id(), "get_ipfs_cid")
        .args_json(json!({}))
        .await?
        .json()?;

    assert_eq!(ipfs_cid, "Howdy".to_string());
    println!("      Passed ✅ changes ipfs_cid");
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
