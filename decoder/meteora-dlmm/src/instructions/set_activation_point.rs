use solana_indexer_core::{borsh, IndexerDeserialize};
#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x5bf90fa51a81fe7d")]
pub struct SetActivationPoint {
    pub activation_point: u64,
}

pub struct SetActivationPointInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetActivationPoint {
    type ArrangedAccounts = SetActivationPointInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let admin = accounts.get(1)?;

        Some(SetActivationPointInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            admin: admin.pubkey,
        })
    }
}
