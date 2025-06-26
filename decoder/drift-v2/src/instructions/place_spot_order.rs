use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x2d4f51a0f85a5bdc")]
pub struct PlaceSpotOrder {
    pub params: OrderParams,
}

pub struct PlaceSpotOrderInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for PlaceSpotOrder {
    type ArrangedAccounts = PlaceSpotOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PlaceSpotOrderInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
