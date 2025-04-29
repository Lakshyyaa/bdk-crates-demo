use anyhow::Context;
use bdk_electrum::BdkElectrumClient;
use bdk_wallet::Wallet;
use bdk_wallet::{
    TxBuilder, TxOrdering,
    bitcoin::{Address, Amount, Network, PublicKey, CompressedPublicKey},
    rusqlite,
};
use std::str::FromStr;
fn main() -> anyhow::Result<()> {
    let mut db = rusqlite::Connection::open("wallet.db")?;
    let new_wallet = Wallet::load()
        .load_wallet(&mut db)
        .expect("failed to load wallet");

    let mut wallet = new_wallet.expect("Wallet should be initialized");
    println!("balance is {}", wallet.balance());
    let pubkey = CompressedPublicKey::from_str(
        "0202020202020202020202020202020202020202020202020202020202020202",
    )?;

    // Or if you need to convert from bitcoin::PublicKey to CompressedPublicKey:
    // let bitcoin_pubkey = PublicKey::from_str("0202020202020202020202020202020202020202020202020202020202020202")?;
    // let pubkey = CompressedPublicKey::from(bitcoin_pubkey);

    // Fix 2: Pass Network::Testnet by value, not by reference
    let addr2 = Address::p2wpkh(
        &pubkey,
        Network::Testnet, // Remove the & reference
    );
    let psbt1 = {
        let mut builder = wallet.build_tx();
        builder.add_recipient(addr2.script_pubkey(), Amount::from_sat(50_000));
        builder.finish()?
    };
    wallet.persist(&mut db)?;

    Ok(())
    // const ELECTRUM_URL: &str = "ssl://electrum.blockstream.info:60002";
    // let new_client =
    //     BdkElectrumClient::new(bdk_electrum::electrum_client::Client::new(ELECTRUM_URL).unwrap());
    // let request = wallet.start_full_scan();
    // eprintln!("Starting wallet synchronization...");
    // let update = new_client.full_scan(request, 5, 90, false)?;
    // eprintln!("Sync completed. Applying updates...");

    // build tx
}
