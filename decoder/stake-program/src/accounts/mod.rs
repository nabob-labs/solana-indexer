use solana_indexer_core::account::AccountDecoder;

use super::StakeProgramDecoder;

pub enum StakeProgramAccount {}

impl AccountDecoder<'_> for StakeProgramDecoder {
    type AccountType = StakeProgramAccount;
    fn decode_account(
        &self,
        _account: &solana_account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
