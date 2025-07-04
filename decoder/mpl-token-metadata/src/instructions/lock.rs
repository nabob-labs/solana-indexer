use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x2e")]
pub struct Lock {
    pub lock_args: LockArgs,
}

pub struct LockInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub token_owner: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub token_record: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
    pub spl_token_program: solana_pubkey::Pubkey,
    pub authorization_rules_program: solana_pubkey::Pubkey,
    pub authorization_rules: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Lock {
    type ArrangedAccounts = LockInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, token_owner, token, mint, metadata, edition, token_record, payer, system_program, sysvar_instructions, spl_token_program, authorization_rules_program, authorization_rules, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LockInstructionAccounts {
            authority: authority.pubkey,
            token_owner: token_owner.pubkey,
            token: token.pubkey,
            mint: mint.pubkey,
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            token_record: token_record.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            spl_token_program: spl_token_program.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules: authorization_rules.pubkey,
        })
    }
}
