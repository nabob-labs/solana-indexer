use {
    super::super::types::*,
    solana_indexer_core::{borsh, IndexerDeserialize},
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x2c")]
pub struct Delegate {
    pub delegate_args: DelegateArgs,
}

pub struct DelegateInstructionAccounts {
    pub delegate_record: solana_pubkey::Pubkey,
    pub delegate: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub master_edition: solana_pubkey::Pubkey,
    pub token_record: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
    pub spl_token_program: solana_pubkey::Pubkey,
    pub authorization_rules_program: solana_pubkey::Pubkey,
    pub authorization_rules: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for Delegate {
    type ArrangedAccounts = DelegateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [delegate_record, delegate, metadata, master_edition, token_record, mint, token, authority, payer, system_program, sysvar_instructions, spl_token_program, authorization_rules_program, authorization_rules, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DelegateInstructionAccounts {
            delegate_record: delegate_record.pubkey,
            delegate: delegate.pubkey,
            metadata: metadata.pubkey,
            master_edition: master_edition.pubkey,
            token_record: token_record.pubkey,
            mint: mint.pubkey,
            token: token.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
            spl_token_program: spl_token_program.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules: authorization_rules.pubkey,
        })
    }
}
