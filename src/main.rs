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
    for i in 0..100 {
        let address = wallet.reveal_next_address(bdk_wallet::KeychainKind::External);
        println!("Address: {}, {:?}", address.to_string(), address.index);
    }
    wallet.persist(&mut db)?;
    Ok(())
}
