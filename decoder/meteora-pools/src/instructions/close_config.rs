use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x9109489d5f7d3d55")]
pub struct CloseConfig {}

pub struct CloseConfigInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for CloseConfig {
    type ArrangedAccounts = CloseConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, admin, rent_receiver, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CloseConfigInstructionAccounts {
            config: config.pubkey,
            admin: admin.pubkey,
            rent_receiver: rent_receiver.pubkey,
        })
    }
}
