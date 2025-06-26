use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x01000000")]
pub struct FreezeLookupTable {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct FreezeLookupTableInstructionAccounts {
    pub look_up_table: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for FreezeLookupTable {
    type ArrangedAccounts = FreezeLookupTableInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [look_up_table, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(FreezeLookupTableInstructionAccounts {
            look_up_table: look_up_table.pubkey,
            authority: authority.pubkey,
        })
    }
}
