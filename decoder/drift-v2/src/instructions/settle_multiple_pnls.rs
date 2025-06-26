use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x7f4275392832987f")]
pub struct SettleMultiplePnls {
    pub market_indexes: Vec<u16>,
    pub mode: SettlePnlMode,
}

pub struct SettleMultiplePnlsInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SettleMultiplePnls {
    type ArrangedAccounts = SettleMultiplePnlsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, authority, spot_market_vault, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SettleMultiplePnlsInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            authority: authority.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
        })
    }
}
