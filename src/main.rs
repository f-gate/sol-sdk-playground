
use solana_client::{client_error::reqwest::Client, connection_cache::ConnectionCache};

mod common;
mod account;

use account::AccountSummeryPage;
use solana_client::{
    thin_client::ThinClient,
};
use std::{
    time::Duration,
    net::{SocketAddr, Ipv4Addr, IpAddr},
    sync::Arc,
};

use solana_sdk::{
    pubkey::Pubkey
};

fn main() {
    // Socket will be localhost
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    // I need a thin client as it impls SyncClient.
    let client = ThinClient::new(socket, socket, Arc::new(ConnectionCache::new(512)));

    // check if client has connected properly?

    // Instantiate the Accountdata page with the specified key.
    let key = Pubkey::new("CEzN7mqP9xoxn2HdyW6fjEJ73t7qaX9Rp2zyS6hb3iEu".as_bytes());
    if let Ok(page_data) = AccountSummeryPage::new(&client, &key) {

        // A connection has been made, the legitimacy of the key has not been checked.
        // If for whatever reason the key is invalid, a balance of 0 will show.
        let mut balance: u64 = Default::default();

        if let Some(account) = page_data.account_data {
            balance = account.lamports;  
        }

        println!("{}", balance);
    }

}
