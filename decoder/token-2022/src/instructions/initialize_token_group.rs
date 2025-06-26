use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x79716c2736330004")]
pub struct InitializeTokenGroup {
    pub update_authority: Option<solana_pubkey::Pubkey>,
    pub max_size: u64,
}

pub struct InitializeTokenGroupInstructionAccounts {
    pub group: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializeTokenGroup {
    type ArrangedAccounts = InitializeTokenGroupInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [group, mint, mint_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeTokenGroupInstructionAccounts {
            group: group.pubkey,
            mint: mint.pubkey,
            mint_authority: mint_authority.pubkey,
        })
    }
}
