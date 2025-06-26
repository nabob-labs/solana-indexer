use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x963e7ddbabdc1aed")]
pub struct UpdateActivationPoint {
    pub new_activation_point: u64,
}

pub struct UpdateActivationPointInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for UpdateActivationPoint {
    type ArrangedAccounts = UpdateActivationPointInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateActivationPointInstructionAccounts {
            pool: pool.pubkey,
            admin: admin.pubkey,
        })
    }
}
