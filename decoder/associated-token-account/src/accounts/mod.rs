use super::SplAssociatedTokenAccountDecoder;
use solana_indexer_core::account::AccountDecoder;

pub enum SplAssociatedTokenAccountAccount {}

impl AccountDecoder<'_> for SplAssociatedTokenAccountDecoder {
    type AccountType = SplAssociatedTokenAccountAccount;
    fn decode_account(
        &self,
        _account: &solana_account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
