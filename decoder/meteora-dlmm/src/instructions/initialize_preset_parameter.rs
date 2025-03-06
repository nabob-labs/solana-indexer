use super::super::types::*;
use solana_indexer_core::{borsh, IndexerDeserialize};

#[derive(
    IndexerDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[indexer(discriminator = "0x42bc47d3626d0eba")]
pub struct InitializePresetParameter {
    pub ix: InitPresetParametersIx,
}

pub struct InitializePresetParameterInstructionAccounts {
    pub preset_parameter: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl solana_indexer_core::deserialize::ArrangeAccounts for InitializePresetParameter {
    type ArrangedAccounts = InitializePresetParameterInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let preset_parameter = accounts.get(0)?;
        let admin = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let rent = accounts.get(3)?;

        Some(InitializePresetParameterInstructionAccounts {
            preset_parameter: preset_parameter.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
