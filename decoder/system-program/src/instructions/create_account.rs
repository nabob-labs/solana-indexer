use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x00000000")]
pub struct CreateAccount {
    pub lamports: u64,
    pub space: u64,
    pub program_address: solana_pubkey::Pubkey,
}

pub struct CreateAccountInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub new_account: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateAccount {
    type ArrangedAccounts = CreateAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, new_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateAccountInstructionAccounts {
            payer: payer.pubkey,
            new_account: new_account.pubkey,
        })
    }
}
