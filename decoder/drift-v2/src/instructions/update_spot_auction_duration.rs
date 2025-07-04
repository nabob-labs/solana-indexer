use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xb6b2cb48bb8f9d6b")]
pub struct UpdateSpotAuctionDuration {
    pub default_spot_auction_duration: u8,
}

pub struct UpdateSpotAuctionDurationInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateSpotAuctionDuration {
    type ArrangedAccounts = UpdateSpotAuctionDurationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSpotAuctionDurationInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
