use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0xdb52dbec3c73c540")]
pub struct SetMarketExpired {}

pub struct SetMarketExpiredInstructionAccounts {
    pub close_market_admin: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetMarketExpired {
    type ArrangedAccounts = SetMarketExpiredInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [close_market_admin, market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetMarketExpiredInstructionAccounts {
            close_market_admin: close_market_admin.pubkey,
            market: market.pubkey,
        })
    }
}
