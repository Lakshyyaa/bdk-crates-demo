use anyhow::Context;
use bdk_electrum::BdkElectrumClient;
use bdk_wallet::Wallet;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::descriptor::template::Bip84;
use bdk_wallet::keys::{DerivableKey, ExtendedKey, bip39::Mnemonic};
use bdk_wallet::rusqlite;
use bdk_wallet::template::DescriptorTemplate;
fn main() -> anyhow::Result<()> {
    let mut db = rusqlite::Connection::open("wallet.db")?;
    let network = bdk_wallet::bitcoin::Network::Testnet;
    let words = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let mnemonic = Mnemonic::parse(words).unwrap();
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    let xprv = xkey.into_xprv(network).unwrap();
    let external_descriptor = Bip84(xprv.clone(), bdk_wallet::KeychainKind::External)
        .build(network)
        .unwrap();
    let internal_descriptor = Bip84(xprv.clone(), bdk_wallet::KeychainKind::Internal)
        .build(network)
        .unwrap();
    let external_descriptor_string: String = external_descriptor.0.to_string();
    let internal_descriptor_string: String = internal_descriptor.0.to_string();
    let new_network = Network::Testnet;
    let mut new_wallet = Wallet::create(
        external_descriptor_string.clone(),
        internal_descriptor_string.clone(),
    )
    .network(new_network)
    .create_wallet(&mut db)
    .context("failed to create wallet")?;
    Ok(())
}
