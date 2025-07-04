use crate::PROGRAM_ID;

use super::StakeProgramDecoder;
pub mod authorize;
pub mod authorize_checked;
pub mod authorize_checked_with_seed;
pub mod authorize_with_seed;
pub mod deactivate;
pub mod deactivate_delinquent;
pub mod delegate_stake;
pub mod get_minimum_delegation;
pub mod initialize;
pub mod initialize_checked;
pub mod merge;
pub mod set_lockup;
pub mod set_lockup_checked;
pub mod split;
pub mod withdraw;

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
pub enum StakeProgramInstruction {
    Initialize(initialize::Initialize),
    Authorize(authorize::Authorize),
    DelegateStake(delegate_stake::DelegateStake),
    Split(split::Split),
    Withdraw(withdraw::Withdraw),
    Deactivate(deactivate::Deactivate),
    SetLockup(set_lockup::SetLockup),
    Merge(merge::Merge),
    AuthorizeWithSeed(authorize_with_seed::AuthorizeWithSeed),
    InitializeChecked(initialize_checked::InitializeChecked),
    AuthorizeChecked(authorize_checked::AuthorizeChecked),
    AuthorizeCheckedWithSeed(authorize_checked_with_seed::AuthorizeCheckedWithSeed),
    SetLockupChecked(set_lockup_checked::SetLockupChecked),
    GetMinimumDelegation(get_minimum_delegation::GetMinimumDelegation),
    DeactivateDelinquent(deactivate_delinquent::DeactivateDelinquent),
}

impl solana_indexer_core::instruction::InstructionDecoder<'_> for StakeProgramDecoder {
    type InstructionType = StakeProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<solana_indexer_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        solana_indexer_core::try_decode_instructions!(instruction,
            StakeProgramInstruction::Initialize => initialize::Initialize,
            StakeProgramInstruction::Authorize => authorize::Authorize,
            StakeProgramInstruction::DelegateStake => delegate_stake::DelegateStake,
            StakeProgramInstruction::Split => split::Split,
            StakeProgramInstruction::Withdraw => withdraw::Withdraw,
            StakeProgramInstruction::Deactivate => deactivate::Deactivate,
            StakeProgramInstruction::SetLockup => set_lockup::SetLockup,
            StakeProgramInstruction::Merge => merge::Merge,
            StakeProgramInstruction::AuthorizeWithSeed => authorize_with_seed::AuthorizeWithSeed,
            StakeProgramInstruction::InitializeChecked => initialize_checked::InitializeChecked,
            StakeProgramInstruction::AuthorizeChecked => authorize_checked::AuthorizeChecked,
            StakeProgramInstruction::AuthorizeCheckedWithSeed => authorize_checked_with_seed::AuthorizeCheckedWithSeed,
            StakeProgramInstruction::SetLockupChecked => set_lockup_checked::SetLockupChecked,
            StakeProgramInstruction::GetMinimumDelegation => get_minimum_delegation::GetMinimumDelegation,
            StakeProgramInstruction::DeactivateDelinquent => deactivate_delinquent::DeactivateDelinquent,
        )
    }
}
