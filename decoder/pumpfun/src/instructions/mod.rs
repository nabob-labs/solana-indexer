use crate::PROGRAM_ID;

use super::PumpfunDecoder;

pub mod buy;
pub mod collect_creator_fee;
pub mod collect_creator_fee_event;
pub mod complete_event;
pub mod complete_pump_amm_migration_event;
pub mod create;
pub mod create_event;
pub mod extend_account;
pub mod extend_account_event;
pub mod initialize;
pub mod migrate;
pub mod sell;
pub mod set_creator;
pub mod set_creator_event;
pub mod set_metaplex_creator;
pub mod set_metaplex_creator_event;
pub mod set_params;
pub mod set_params_event;
pub mod trade_event;
pub mod update_global_authority;
pub mod update_global_authority_event;

#[derive(
    solana_indexer_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum PumpfunInstruction {
    Buy(buy::Buy),
    CollectCreatorFee(collect_creator_fee::CollectCreatorFee),
    Create(create::Create),
    ExtendAccount(extend_account::ExtendAccount),
    Initialize(initialize::Initialize),
    Migrate(migrate::Migrate),
    Sell(sell::Sell),
    SetCreator(set_creator::SetCreator),
    SetMetaplexCreator(set_metaplex_creator::SetMetaplexCreator),
    SetParams(set_params::SetParams),
    UpdateGlobalAuthority(update_global_authority::UpdateGlobalAuthority),
    CollectCreatorFeeEvent(collect_creator_fee_event::CollectCreatorFeeEvent),
    CompleteEvent(complete_event::CompleteEvent),
    CompletePumpAmmMigrationEvent(complete_pump_amm_migration_event::CompletePumpAmmMigrationEvent),
    CreateEvent(create_event::CreateEvent),
    ExtendAccountEvent(extend_account_event::ExtendAccountEvent),
    SetCreatorEvent(set_creator_event::SetCreatorEvent),
    SetMetaplexCreatorEvent(set_metaplex_creator_event::SetMetaplexCreatorEvent),
    SetParamsEvent(set_params_event::SetParamsEvent),
    TradeEvent(trade_event::TradeEvent),
    UpdateGlobalAuthorityEvent(update_global_authority_event::UpdateGlobalAuthorityEvent),
}

impl solana_indexer_core::instruction::InstructionDecoder<'_> for PumpfunDecoder {
    type InstructionType = PumpfunInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<solana_indexer_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }
        solana_indexer_core::try_decode_instructions!(instruction,
            PumpfunInstruction::Buy => buy::Buy,
            PumpfunInstruction::CollectCreatorFee => collect_creator_fee::CollectCreatorFee,
            PumpfunInstruction::Create => create::Create,
            PumpfunInstruction::ExtendAccount => extend_account::ExtendAccount,
            PumpfunInstruction::Initialize => initialize::Initialize,
            PumpfunInstruction::Migrate => migrate::Migrate,
            PumpfunInstruction::Sell => sell::Sell,
            PumpfunInstruction::SetCreator => set_creator::SetCreator,
            PumpfunInstruction::SetMetaplexCreator => set_metaplex_creator::SetMetaplexCreator,
            PumpfunInstruction::SetParams => set_params::SetParams,
            PumpfunInstruction::UpdateGlobalAuthority => update_global_authority::UpdateGlobalAuthority,
            PumpfunInstruction::CollectCreatorFeeEvent => collect_creator_fee_event::CollectCreatorFeeEvent,
            PumpfunInstruction::CompleteEvent => complete_event::CompleteEvent,
            PumpfunInstruction::CompletePumpAmmMigrationEvent => complete_pump_amm_migration_event::CompletePumpAmmMigrationEvent,
            PumpfunInstruction::CreateEvent => create_event::CreateEvent,
            PumpfunInstruction::ExtendAccountEvent => extend_account_event::ExtendAccountEvent,
            PumpfunInstruction::SetCreatorEvent => set_creator_event::SetCreatorEvent,
            PumpfunInstruction::SetMetaplexCreatorEvent => set_metaplex_creator_event::SetMetaplexCreatorEvent,
            PumpfunInstruction::SetParamsEvent => set_params_event::SetParamsEvent,
            PumpfunInstruction::TradeEvent => trade_event::TradeEvent,
            PumpfunInstruction::UpdateGlobalAuthorityEvent => update_global_authority_event::UpdateGlobalAuthorityEvent,
        )
    }
}
