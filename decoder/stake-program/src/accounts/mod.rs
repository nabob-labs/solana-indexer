use solana_indexer_core::account::AccountDecoder;

use super::StakeProgramDecoder;

pub enum StakeProgramAccount {}

impl<'a> AccountDecoder<'a> for StakeProgramDecoder {
    type AccountType = StakeProgramAccount;
    fn decode_account(
        &self,
        _account: &solana_sdk::account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
