use {
    super::JupiterDcaDecoder,
    crate::PROGRAM_ID,
    solana_indexer_core::{account::AccountDecoder, deserialize::IndexerDeserialize},
};
pub mod dca;

pub enum JupiterDcaAccount {
    Dca(dca::Dca),
}

impl AccountDecoder<'_> for JupiterDcaDecoder {
    type AccountType = JupiterDcaAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<solana_indexer_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = dca::Dca::deserialize(account.data.as_slice()) {
            return Some(solana_indexer_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JupiterDcaAccount::Dca(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
