use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x6e827329a466023b")]
pub struct DepositStakeAccount {
    pub validator_index: u32,
}

pub struct DepositStakeAccountInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub validator_list: solana_pubkey::Pubkey,
    pub stake_list: solana_pubkey::Pubkey,
    pub stake_account: solana_pubkey::Pubkey,
    pub stake_authority: solana_pubkey::Pubkey,
    pub duplication_flag: solana_pubkey::Pubkey,
    pub rent_payer: solana_pubkey::Pubkey,
    pub msol_mint: solana_pubkey::Pubkey,
    pub mint_to: solana_pubkey::Pubkey,
    pub msol_mint_authority: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub stake_program: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for DepositStakeAccount {
    type ArrangedAccounts = DepositStakeAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, validator_list, stake_list, stake_account, stake_authority, duplication_flag, rent_payer, msol_mint, mint_to, msol_mint_authority, clock, rent, system_program, token_program, stake_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositStakeAccountInstructionAccounts {
            state: state.pubkey,
            validator_list: validator_list.pubkey,
            stake_list: stake_list.pubkey,
            stake_account: stake_account.pubkey,
            stake_authority: stake_authority.pubkey,
            duplication_flag: duplication_flag.pubkey,
            rent_payer: rent_payer.pubkey,
            msol_mint: msol_mint.pubkey,
            mint_to: mint_to.pubkey,
            msol_mint_authority: msol_mint_authority.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}
