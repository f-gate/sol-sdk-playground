
mod common;
mod account;

use account::AccountSummeryPage;
use solana_client::{
    rpc_client::RpcClient,
};
use std::{
    str::FromStr,
    time::Duration, sync::Arc,
};

use solana_sdk::{
    pubkey::Pubkey
};
use common::to_sol;

fn main() {

    let http = "https://api.mainnet-beta.solana.com";
    let timeout = Duration::from_secs(10);
    let client: Arc<RpcClient> = Arc::new(RpcClient::new_with_timeout(http, timeout));

    // deffo would do some error handling here. Defaulting for simplicity.
    let pub_key_string = "CEzN7mqP9xoxn2HdyW6fjEJ73t7qaX9Rp2zyS6hb3iEu";
    let pub_key = <Pubkey as FromStr>::from_str(pub_key_string).expect("static key is valid; qed");

    
    let res = AccountSummeryPage::new(client.clone(), &pub_key);
    match res {
        Ok(page_data) => {

          let balance = to_sol(page_data.account_data.lamports);  
          println!("\nSol Balance: {:?}", balance);

          println!("\nSPL Tokens: {}", page_data.token_accounts.len());
        }
        Err(e) => {
            println!("{}", e);
        }

    }
}   
