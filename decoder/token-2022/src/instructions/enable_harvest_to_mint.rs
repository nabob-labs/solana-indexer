use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x25")]
pub struct EnableHarvestToMint {
    pub confidential_transfer_fee_discriminator: u8,
}

pub struct EnableHarvestToMintInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for EnableHarvestToMint {
    type ArrangedAccounts = EnableHarvestToMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EnableHarvestToMintInstructionAccounts {
            mint: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
