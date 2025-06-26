use solana_indexer_core::account::AccountDecoder;

use super::OkxDexDecoder;

pub enum OkxDexAccount {}

impl AccountDecoder<'_> for OkxDexDecoder {
    type AccountType = OkxDexAccount;
    fn decode_account(
        &self,
        _account: &solana_account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
