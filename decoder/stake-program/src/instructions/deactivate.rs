use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x2c7021ac711c8e0d")]
pub struct Deactivate {}

pub struct DeactivateInstructionAccounts {
    pub stake: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Deactivate {
    type ArrangedAccounts = DeactivateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, clock, stake_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DeactivateInstructionAccounts {
            stake: stake.pubkey,
            clock: clock.pubkey,
            stake_authority: stake_authority.pubkey,
        })
    }
}
