use solana_indexer_core::account::AccountDecoder;
use solana_indexer_core::deserialize::IndexerDeserialize;

use super::JupiterLimitOrderDecoder;
pub mod fee;
pub mod order;

pub enum JupiterLimitOrderAccount {
    Fee(fee::Fee),
    Order(order::Order),
}

impl<'a> AccountDecoder<'a> for JupiterLimitOrderDecoder {
    type AccountType = JupiterLimitOrderAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = fee::Fee::deserialize(account.data.as_slice()) {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JupiterLimitOrderAccount::Fee(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = order::Order::deserialize(account.data.as_slice()) {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JupiterLimitOrderAccount::Order(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
