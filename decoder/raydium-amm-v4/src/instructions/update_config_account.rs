use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x0f")]
pub struct UpdateConfigAccount {
    pub param: u8,
    pub owner: solana_sdk::pubkey::Pubkey,
}

pub struct UpdateConfigAccountInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateConfigAccount {
    type ArrangedAccounts = UpdateConfigAccountInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let amm_config = accounts.get(1)?;

        Some(UpdateConfigAccountInstructionAccounts {
            admin: admin.pubkey,
            amm_config: amm_config.pubkey,
        })
    }
}
