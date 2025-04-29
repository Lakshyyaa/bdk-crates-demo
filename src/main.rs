use anyhow::Context;
use bdk_electrum::BdkElectrumClient;
use bdk_wallet::Wallet;
use bdk_wallet::rusqlite;
fn main() -> anyhow::Result<()> {
    let mut db = rusqlite::Connection::open("wallet.db")?;
    let new_wallet = Wallet::load()
        .load_wallet(&mut db)
        .expect("failed to load wallet");

    let mut wallet = new_wallet.expect("Wallet should be initialized");
    
    const ELECTRUM_URL: &str = "ssl://electrum.blockstream.info:60002";
    let new_client =
        BdkElectrumClient::new(bdk_electrum::electrum_client::Client::new(ELECTRUM_URL).unwrap());
    let request = wallet.start_full_scan();
    eprintln!("Starting wallet synchronization...");
    let update = new_client.full_scan(request, 5, 90, false)?;
    eprintln!("Sync completed. Applying updates...");
    wallet.apply_update(update)?;
    wallet.persist(&mut db)?;
    Ok(())

}
