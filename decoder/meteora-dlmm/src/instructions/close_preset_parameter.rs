use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x04949164861ab53d")]
pub struct ClosePresetParameter {}

pub struct ClosePresetParameterInstructionAccounts {
    pub preset_parameter: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub rent_receiver: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for ClosePresetParameter {
    type ArrangedAccounts = ClosePresetParameterInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [preset_parameter, admin, rent_receiver, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ClosePresetParameterInstructionAccounts {
            preset_parameter: preset_parameter.pubkey,
            admin: admin.pubkey,
            rent_receiver: rent_receiver.pubkey,
        })
    }
}
