
use solana_sdk::{
    pubkey::Pubkey,
    account::Account, transport::TransportError    
};

use solana_client::rpc_client::RpcClient;

// Maybe this can hold some needed state for the page;
pub struct AccountSummeryPage<'a> {
    pub client: &'a RpcClient,
    pub account_data: Account,
}

/// The general idea is that a page and its associated data can be held in memory.
/// Each page that needs a client connection can it and it will be shared across.
/// Perhaps there needs to be a check to see if the connection is still valid?
impl <'a> AccountSummeryPage<'a> {
    /// Instantiate the page data for an account.
    pub fn new(client: &'a RpcClient, pubkey: &Pubkey) -> Result<Self, TransportError> {
        let maybe_account = client.get_account(pubkey)?;
        Ok(
            Self {
                client,
                account_data: maybe_account
            }
        )
    }
}


