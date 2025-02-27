use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x7b86510031446262")]
pub struct ClosePosition {}

pub struct ClosePositionInstructionAccounts {
    pub position_authority: solana_sdk::pubkey::Pubkey,
    pub receiver: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ClosePosition {
    type ArrangedAccounts = ClosePositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position_authority, receiver, position, position_mint, position_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClosePositionInstructionAccounts {
            position_authority: position_authority.pubkey,
            receiver: receiver.pubkey,
            position: position.pubkey,
            position_mint: position_mint.pubkey,
            position_token_account: position_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
