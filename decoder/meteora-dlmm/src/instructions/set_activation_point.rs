use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x5bf90fa51a81fe7d")]
pub struct SetActivationPoint {
    pub activation_point: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetActivationPointInstructionAccounts {
    pub lb_pair: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for SetActivationPoint {
    type ArrangedAccounts = SetActivationPointInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetActivationPointInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            admin: admin.pubkey,
        })
    }
}
