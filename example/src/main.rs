#[macro_use]
extern crate anyhow;
extern crate futures;
extern crate reqwest;
extern crate stellar_base;
extern crate stellar_horizon;
extern crate tokio;

use anyhow::Result;
use futures::stream::{Stream, StreamExt};
use stellar_base::crypto::KeyPair;
use stellar_horizon::api;
use stellar_horizon::client::{HorizonClient, HorizonHttpClient};
use stellar_horizon::request::PageRequest;

#[tokio::main]
async fn main() -> Result<()> {
    let horizon = HorizonHttpClient::new("https://horizon.stellar.org")?;

    let page = horizon.request(api::trades::all().with_limit(20)).await?;
    for record in page.records() {
        println!("{:?}", record);
    }

    let mut stream = horizon.stream(api::trades::all().with_cursor("now"))?;

    while let Some(event) = stream.next().await {
        let event = event?;
        println!("> {:?}", event);
    }

    /*
    let keypair =
        KeyPair::from_secret_seed("SAZF6FGQLYYB47DVFAB2XORTDDIDND3D7U65MI3L7KOOOFBT3SPQDQ7W")?;
    let keypair = KeyPair::random()?;
    let secret_seed = keypair.secret_key().secret_seed()?;
    let account_id = keypair.public_key().account_id()?;

    println!("Asking friendbot to fund account:");
    println!("  Secret Seed: {}", secret_seed);
    println!("   Account Id: {}", account_id);

    let friendbot_url = format!("https://friendbot.stellar.org/?addr={}", account_id);
    let friendbot_resp = reqwest::get(&friendbot_url).await?;
    if !friendbot_resp.status().is_success() {
        return Err(anyhow!("Received bad response from friendbot"));
    }

    println!("Account funded. Getting account details.");

    let account_req = api::account::single(keypair.public_key())?;
    let account_resp = horizon.request(&account_req).await?;

    println!("Got respn {:?}", account_resp);

    println!("Getting account operations.");
    println!("Building transaction.");
    println!("Submitting transaction.");
    println!("All done.");
     */

    Ok(())
}
