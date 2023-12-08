use std::time::Duration;

use ethers::{
    prelude::{Address,LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256},
    utils::Ganache,
};
use eyre::{ContextCompact, Result};
use hex::ToHex;

#[tokio::main]
async fn main() -> Result<()>{
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("HTTP Endpoint: {}", ganache.endpoint());

    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let first_address = wallet.address();
    println!(
        "Wallet first address: {}",first_address.encode_hex::<String>()
    );

    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));

    let firrst_balance = provider.get_balance(first_address, None).await?;
    println!("first wallet balance: {}", firrst_balance);

    let other_address_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
    let other_address = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646".parse::<Address>()?;

}
