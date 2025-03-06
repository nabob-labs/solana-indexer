use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x3d7334172e0d1f90")]
pub struct TogglePairStatus {}

pub struct TogglePairStatusInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for TogglePairStatus {
    type ArrangedAccounts = TogglePairStatusInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let admin = accounts.get(1)?;

        Some(TogglePairStatusInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            admin: admin.pubkey,
        })
    }
}
