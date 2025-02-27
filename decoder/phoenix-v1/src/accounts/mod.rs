use solana_indexer_core::account::AccountDecoder;
use solana_indexer_core::deserialize::IndexerDeserialize;

use crate::PROGRAM_ID;

use super::PhoenixDecoder;
pub mod market_header;
pub mod seat;

pub enum PhoenixAccount {
    MarketHeader(market_header::MarketHeader),
    Seat(seat::Seat),
}

impl<'a> AccountDecoder<'a> for PhoenixDecoder {
    type AccountType = PhoenixAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            market_header::MarketHeader::deserialize(account.data.as_slice())
        {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PhoenixAccount::MarketHeader(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = seat::Seat::deserialize(account.data.as_slice()) {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PhoenixAccount::Seat(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
