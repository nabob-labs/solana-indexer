use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x3ec6d6c1d59f6cd2")]
pub struct Claim {
    pub id: u8,
}

pub struct ClaimInstructionAccounts {
    pub wallet: solana_pubkey::Pubkey,
    pub program_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Claim {
    type ArrangedAccounts = ClaimInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [wallet, program_authority, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ClaimInstructionAccounts {
            wallet: wallet.pubkey,
            program_authority: program_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
