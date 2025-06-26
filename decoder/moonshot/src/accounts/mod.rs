use {
    super::MoonshotDecoder,
    crate::PROGRAM_ID,
    solana_indexer_core::{account::AccountDecoder, deserialize::IndexerDeserialize},
};
pub mod config_account;
pub mod curve_account;

pub enum MoonshotAccount {
    ConfigAccount(config_account::ConfigAccount),
    CurveAccount(curve_account::CurveAccount),
}

impl AccountDecoder<'_> for MoonshotDecoder {
    type AccountType = MoonshotAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            config_account::ConfigAccount::deserialize(account.data.as_slice())
        {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MoonshotAccount::ConfigAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            curve_account::CurveAccount::deserialize(account.data.as_slice())
        {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MoonshotAccount::CurveAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
