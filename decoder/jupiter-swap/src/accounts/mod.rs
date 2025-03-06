use solana_indexer_core::account::AccountDecoder;
use solana_indexer_core::deserialize::IndexerDeserialize;

use super::JupiterSwapDecoder;
pub mod token_ledger;

pub enum JupiterSwapAccount {
    TokenLedger(token_ledger::TokenLedger),
}

impl<'a> AccountDecoder<'a> for JupiterSwapDecoder {
    type AccountType = JupiterSwapAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            token_ledger::TokenLedger::deserialize(account.data.as_slice())
        {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JupiterSwapAccount::TokenLedger(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
