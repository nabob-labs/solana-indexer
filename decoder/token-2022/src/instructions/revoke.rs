use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x05")]
pub struct Revoke {}

pub struct RevokeInstructionAccounts {
    pub source: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Revoke {
    type ArrangedAccounts = RevokeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RevokeInstructionAccounts {
            source: source.pubkey,
            owner: owner.pubkey,
        })
    }
}
