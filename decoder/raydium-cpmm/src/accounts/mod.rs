use {
    super::RaydiumCpmmDecoder,
    crate::PROGRAM_ID,
    solana_indexer_core::{account::AccountDecoder, deserialize::IndexerDeserialize},
};
pub mod amm_config;
pub mod observation_state;
pub mod pool_state;

#[allow(clippy::large_enum_variant)]
pub enum RaydiumCpmmAccount {
    AmmConfig(amm_config::AmmConfig),
    ObservationState(observation_state::ObservationState),
    PoolState(pool_state::PoolState),
}

impl AccountDecoder<'_> for RaydiumCpmmDecoder {
    type AccountType = RaydiumCpmmAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = amm_config::AmmConfig::deserialize(account.data.as_slice()) {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumCpmmAccount::AmmConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            observation_state::ObservationState::deserialize(account.data.as_slice())
        {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumCpmmAccount::ObservationState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool_state::PoolState::deserialize(account.data.as_slice()) {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumCpmmAccount::PoolState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
