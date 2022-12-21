
use solana_sdk::{
    client::{SyncClient},
    pubkey::Pubkey,
    account::Account, transport::TransportError    
};

// Maybe this can hold some needed state for the page;
pub struct AccountSummeryPage<'a ,T: SyncClient> {
    pub client: &'a T,
    pub account_data: Option<Account>,
}

/// The general idea is that a page and its associated data can be held in memory.
/// Each page that needs a client connection can it and it will be shared across.
/// Perhaps there needs to be a check to see if the connection is still valid?
impl <'a, T: SyncClient> AccountSummeryPage<'a, T> {
    /// Instantiate the page data for an account.
    pub fn new(client: &'a T, pubkey: &Pubkey) -> Result<Self, TransportError> {
        let maybe_account = client.get_account(pubkey)?;
        Ok(
            Self {
                client,
                account_data: maybe_account
            }
        )
    }
}


