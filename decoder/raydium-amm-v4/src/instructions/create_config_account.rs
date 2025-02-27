use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0e")]
pub struct CreateConfigAccount {}

pub struct CreateConfigAccountInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CreateConfigAccount {
    type ArrangedAccounts = CreateConfigAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, amm_config, owner, system_program, rent, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateConfigAccountInstructionAccounts {
            admin: admin.pubkey,
            amm_config: amm_config.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
