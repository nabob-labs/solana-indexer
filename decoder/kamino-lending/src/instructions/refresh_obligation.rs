use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x218493e497c04859")]
pub struct RefreshObligation {}

pub struct RefreshObligationInstructionAccounts {
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for RefreshObligation {
    type ArrangedAccounts = RefreshObligationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lending_market, obligation, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RefreshObligationInstructionAccounts {
            lending_market: lending_market.pubkey,
            obligation: obligation.pubkey,
        })
    }
}
