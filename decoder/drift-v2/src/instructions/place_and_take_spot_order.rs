use super::super::types::*;

use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xbf038a4772c6ca64")]
pub struct PlaceAndTakeSpotOrder {
    pub params: OrderParams,
    pub fulfillment_type: Option<SpotFulfillmentType>,
    pub maker_order_id: Option<u32>,
}

pub struct PlaceAndTakeSpotOrderInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for PlaceAndTakeSpotOrder {
    type ArrangedAccounts = PlaceAndTakeSpotOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, user_stats, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PlaceAndTakeSpotOrderInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
        })
    }
}
