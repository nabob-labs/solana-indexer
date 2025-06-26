use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x04")]
pub struct UpdatePrimarySaleHappenedViaToken {}

pub struct UpdatePrimarySaleHappenedViaTokenInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePrimarySaleHappenedViaToken {
    type ArrangedAccounts = UpdatePrimarySaleHappenedViaTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, owner, token, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePrimarySaleHappenedViaTokenInstructionAccounts {
            metadata: metadata.pubkey,
            owner: owner.pubkey,
            token: token.pubkey,
        })
    }
}
