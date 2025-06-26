use solana_indexer_core::account::AccountDecoder;
use solana_indexer_core::deserialize::IndexerDeserialize;

use crate::PROGRAM_ID;

use super::VirtualsDecoder;
pub mod virtuals_pool;

pub enum VirtualsAccount {
    VirtualsPool(virtuals_pool::VirtualsPool),
}

impl AccountDecoder<'_> for VirtualsDecoder {
    type AccountType = VirtualsAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            virtuals_pool::VirtualsPool::deserialize(account.data.as_slice())
        {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: VirtualsAccount::VirtualsPool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
