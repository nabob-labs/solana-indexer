use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x1416567bc61cdb84")]
pub struct CollectCreatorFee {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectCreatorFeeInstructionAccounts {
    pub creator: solana_pubkey::Pubkey,
    pub creator_vault: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CollectCreatorFee {
    type ArrangedAccounts = CollectCreatorFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [creator, creator_vault, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectCreatorFeeInstructionAccounts {
            creator: creator.pubkey,
            creator_vault: creator_vault.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
