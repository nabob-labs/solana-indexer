use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x27195f6b7411731c")]
pub struct ClosePresetParameter2 {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClosePresetParameter2InstructionAccounts {
    pub preset_parameter: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ClosePresetParameter2 {
    type ArrangedAccounts = ClosePresetParameter2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [preset_parameter, admin, rent_receiver, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ClosePresetParameter2InstructionAccounts {
            preset_parameter: preset_parameter.pubkey,
            admin: admin.pubkey,
            rent_receiver: rent_receiver.pubkey,
        })
    }
}
