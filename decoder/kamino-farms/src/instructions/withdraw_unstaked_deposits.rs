use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x2466bb31dc248443")]
pub struct WithdrawUnstakedDeposits {}

pub struct WithdrawUnstakedDepositsInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub user_state: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub user_ata: solana_pubkey::Pubkey,
    pub farm_vault: solana_pubkey::Pubkey,
    pub farm_vaults_authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for WithdrawUnstakedDeposits {
    type ArrangedAccounts = WithdrawUnstakedDepositsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, user_state, farm_state, user_ata, farm_vault, farm_vaults_authority, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawUnstakedDepositsInstructionAccounts {
            owner: owner.pubkey,
            user_state: user_state.pubkey,
            farm_state: farm_state.pubkey,
            user_ata: user_ata.pubkey,
            farm_vault: farm_vault.pubkey,
            farm_vaults_authority: farm_vaults_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
