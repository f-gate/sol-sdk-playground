
use solana_sdk::{
    pubkey::Pubkey,
    account::Account, transport::TransportError    

};
use std::{str::FromStr, sync::Arc};
use solana_client::{
    rpc_client::RpcClient,
    rpc_response::RpcKeyedAccount
};

// Maybe this can hold some needed state for the page;
pub struct AccountSummeryPage {
    pub client: Arc<RpcClient>,
    pub account_data: Account,
    pub token_accounts: Vec<RpcKeyedAccount>,
}

/// The general idea is that a page and its associated data can be held in memory.
/// Each page that needs a client connection can it and it will be shared across.
/// Perhaps there needs to be a check to see if the connection is still valid?
impl AccountSummeryPage {
    /// Instantiate the page data for an account.
    pub fn new(client: Arc<RpcClient>, owner: &Pubkey) -> Result<Self, TransportError> {
        let spl_tokens = <Pubkey as FromStr>::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        let account_data = client.get_account(owner)?;
        let token_accounts = client.get_token_accounts_by_owner(owner,  solana_client::rpc_request::TokenAccountsFilter::ProgramId(spl_tokens))?;
        Ok(
            Self {
                client,
                account_data,
                token_accounts,
            }
        )
    }
}


