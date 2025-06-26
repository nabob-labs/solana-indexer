use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x71e1a6b95ee7601c")]
pub struct UpdatePoolCreator {}

pub struct UpdatePoolCreatorInstructionAccounts {
    pub creator: solana_pubkey::Pubkey,
    pub new_creator: solana_pubkey::Pubkey,
    pub virtuals_mint: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub new_creator_virtuals_ata: solana_pubkey::Pubkey,
    pub new_creator_token_ata: solana_pubkey::Pubkey,
    pub vpool: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdatePoolCreator {
    type ArrangedAccounts = UpdatePoolCreatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [creator, new_creator, virtuals_mint, token_mint, new_creator_virtuals_ata, new_creator_token_ata, vpool, token_program, associated_token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UpdatePoolCreatorInstructionAccounts {
            creator: creator.pubkey,
            new_creator: new_creator.pubkey,
            virtuals_mint: virtuals_mint.pubkey,
            token_mint: token_mint.pubkey,
            new_creator_virtuals_ata: new_creator_virtuals_ata.pubkey,
            new_creator_token_ata: new_creator_token_ata.pubkey,
            vpool: vpool.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
