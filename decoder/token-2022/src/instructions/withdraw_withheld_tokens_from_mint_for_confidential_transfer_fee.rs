use {
    solana_indexer_core::{borsh, IndexerDeserialize},
    serde_big_array::BigArray,
};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x25")]
pub struct WithdrawWithheldTokensFromMintForConfidentialTransferFee {
    pub confidential_transfer_fee_discriminator: u8,
    pub proof_instruction_offset: i8,
    #[serde(with = "BigArray")]
    pub new_decryptable_available_balance: [u8; 36],
}

pub struct WithdrawWithheldTokensFromMintForConfidentialTransferFeeInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub instructions_sysvar_or_context_state: solana_sdk::pubkey::Pubkey,
    pub record: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts
    for WithdrawWithheldTokensFromMintForConfidentialTransferFee
{
    type ArrangedAccounts =
        WithdrawWithheldTokensFromMintForConfidentialTransferFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, destination, instructions_sysvar_or_context_state, record, authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(
            WithdrawWithheldTokensFromMintForConfidentialTransferFeeInstructionAccounts {
                mint: mint.pubkey,
                destination: destination.pubkey,
                instructions_sysvar_or_context_state: instructions_sysvar_or_context_state.pubkey,
                record: record.pubkey,
                authority: authority.pubkey,
            },
        )
    }
}
