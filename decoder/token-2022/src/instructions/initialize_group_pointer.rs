use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x28")]
pub struct InitializeGroupPointer {
    pub group_pointer_discriminator: u8,
    pub authority: Option<solana_sdk::pubkey::Pubkey>,
    pub group_address: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct InitializeGroupPointerInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeGroupPointer {
    type ArrangedAccounts = InitializeGroupPointerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeGroupPointerInstructionAccounts { mint: mint.pubkey })
    }
}
