use {
    super::KaminoFarmsDecoder,
    crate::PROGRAM_ID,
    solana_indexer_core::{account::AccountDecoder, deserialize::IndexerDeserialize},
};
pub mod farm_state;
pub mod global_config;
pub mod oracle_prices;
pub mod user_state;

pub enum KaminoFarmsAccount {
    FarmState(Box<farm_state::FarmState>),
    GlobalConfig(Box<global_config::GlobalConfig>),
    UserState(Box<user_state::UserState>),
    OraclePrices(Box<oracle_prices::OraclePrices>),
}

impl AccountDecoder<'_> for KaminoFarmsDecoder {
    type AccountType = KaminoFarmsAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }
        if let Some(decoded_account) = farm_state::FarmState::deserialize(account.data.as_slice()) {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoFarmsAccount::FarmState(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            global_config::GlobalConfig::deserialize(account.data.as_slice())
        {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoFarmsAccount::GlobalConfig(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = user_state::UserState::deserialize(account.data.as_slice()) {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoFarmsAccount::UserState(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            oracle_prices::OraclePrices::deserialize(account.data.as_slice())
        {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: KaminoFarmsAccount::OraclePrices(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
