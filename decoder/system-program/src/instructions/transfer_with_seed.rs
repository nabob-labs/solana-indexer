use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0B")]
pub struct TransferWithSeed {
    pub lamports: u64,
    pub from_seed: String,
    pub from_owner: solana_sdk::pubkey::Pubkey,
}

pub struct TransferWithSeedAccounts {
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub base_for_funding_account: solana_sdk::pubkey::Pubkey,
    pub recipient_account: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for TransferWithSeed {
    type ArrangedAccounts = TransferWithSeedAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [funding_account, base_for_funding_account, recipient_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TransferWithSeedAccounts {
            funding_account: funding_account.pubkey,
            base_for_funding_account: base_for_funding_account.pubkey,
            recipient_account: recipient_account.pubkey,
        })
    }
}
