use {
    super::JupiterSwapDecoder,
    crate::PROGRAM_ID,
    solana_indexer_core::{account::AccountDecoder, deserialize::IndexerDeserialize},
};
pub mod token_ledger;

pub enum JupiterSwapAccount {
    TokenLedger(token_ledger::TokenLedger),
}

impl AccountDecoder<'_> for JupiterSwapDecoder {
    type AccountType = JupiterSwapAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

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
