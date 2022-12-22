
mod common;
mod account;

use account::AccountSummeryPage;
use solana_client::{
    rpc_client::RpcClient,
};
use std::{
    str::FromStr
};

use solana_sdk::{
    pubkey::Pubkey
};

use common::to_sol;

fn main() {

    let http = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(http);

    let pub_key_string = "CEzN7mqP9xoxn2HdyW6fjEJ73t7qaX9Rp2zyS6hb3iEu";
    let pub_key = <Pubkey as FromStr>::from_str(pub_key_string);
    
    if let Ok(page_data) = AccountSummeryPage::new(&client, &pub_key.unwrap_or_default()) {
        let balance = to_sol(page_data.account_data.lamports.clone());  
        
        println!("\nAccount: {:?}", page_data.account_data);
        
        println!("\n{} SOL", balance);

    }
     else {
        // handle transport err.
        println!("Could not connect to cluster")
    }
}
