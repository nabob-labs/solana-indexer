use solana_indexer_core::account::AccountDecoder;
use solana_indexer_core::deserialize::IndexerDeserialize;

use crate::PROGRAM_ID;

use super::PumpSwapDecoder;
pub mod bonding_curve;
pub mod global_config;
pub mod pool;

pub enum PumpSwapAccount {
    BondingCurve(bonding_curve::BondingCurve),
    GlobalConfig(global_config::GlobalConfig),
    Pool(pool::Pool),
}

impl AccountDecoder<'_> for PumpSwapDecoder {
    type AccountType = PumpSwapAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            bonding_curve::BondingCurve::deserialize(account.data.as_slice())
        {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpSwapAccount::BondingCurve(decoded_account),
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
                data: PumpSwapAccount::GlobalConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool::Pool::deserialize(account.data.as_slice()) {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpSwapAccount::Pool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
