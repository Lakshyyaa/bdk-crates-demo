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
    {
        let mut txs = wallet.transactions();
        let first_tx = txs.next().unwrap();
        let send_and_received = wallet.sent_and_received(&first_tx.tx_node);
        println!("send_and_received: {:?}", send_and_received);
    }
    println!("balance is {}", wallet.balance());
    wallet.persist(&mut db)?;
    Ok(())
}
