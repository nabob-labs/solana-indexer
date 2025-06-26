use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x25")]
pub struct DisableHarvestToMint {
    pub confidential_transfer_fee_discriminator: u8,
}

pub struct DisableHarvestToMintInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DisableHarvestToMint {
    type ArrangedAccounts = DisableHarvestToMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DisableHarvestToMintInstructionAccounts {
            mint: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
