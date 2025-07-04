use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xacb80c0a346940d5")]
pub struct TransferExcessSpreadBalance {}

pub struct TransferExcessSpreadBalanceInstructionAccounts {
    pub zeta_group: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub spread_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for TransferExcessSpreadBalance {
    type ArrangedAccounts = TransferExcessSpreadBalanceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [zeta_group, margin_account, spread_account, authority, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(TransferExcessSpreadBalanceInstructionAccounts {
            zeta_group: zeta_group.pubkey,
            margin_account: margin_account.pubkey,
            spread_account: spread_account.pubkey,
            authority: authority.pubkey,
        })
    }
}
