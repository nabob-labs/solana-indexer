use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x03000000")]
pub struct DeactivateLookupTable {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeactivateLookupTableInstructionAccounts {
    pub look_up_table: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DeactivateLookupTable {
    type ArrangedAccounts = DeactivateLookupTableInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [look_up_table, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DeactivateLookupTableInstructionAccounts {
            look_up_table: look_up_table.pubkey,
            authority: authority.pubkey,
        })
    }
}
