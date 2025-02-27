use crate::SystemProgramDecoder;
pub mod advance_nonce_account;
pub mod allocate;
pub mod allocate_with_seed;
pub mod assign;
pub mod assign_with_seed;
pub mod authorize_nonce_account;
pub mod create_account;
pub mod create_account_with_seed;
pub mod initialize_nonce_account;
pub mod transfer;
pub mod transfer_with_seed;
pub mod upgrade_nonce_account;
pub mod withdraw_nonce_account;

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
pub enum SystemProgramInstruction {
    CreateAccount(create_account::CreateAccount),
    Assign(assign::Assign),
    Transfer(transfer::Transfer),
    CreateAccountWithSeed(create_account_with_seed::CreateAccountWithSeed),
    AdvanceNonceAccount(advance_nonce_account::AdvanceNonceAccount),
    WithdrawNonceAccount(withdraw_nonce_account::WithdrawNonceAccount),
    InitializeNonceAccount(initialize_nonce_account::InitializeNonceAccount),
    AuthorizeNonceAccount(authorize_nonce_account::AuthorizeNonceAccount),
    Allocate(allocate::Allocate),
    AllocateWithSeed(allocate_with_seed::AllocateWithSeed),
    AssignWithSeed(assign_with_seed::AssignWithSeed),
    TransferWithSeed(transfer_with_seed::TransferWithSeed),
    UpgradeNonceAccount(upgrade_nonce_account::UpgradeNonceAccount),
}

impl solana_indexer_core::instruction::InstructionDecoder<'_> for SystemProgramDecoder {
    type InstructionType = SystemProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<solana_indexer_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&solana_sdk::system_program::id()) {
            return None;
        }

        solana_indexer_core::try_decode_instructions!(instruction,
            SystemProgramInstruction::CreateAccount => create_account::CreateAccount,
            SystemProgramInstruction::Assign => assign::Assign,
            SystemProgramInstruction::Transfer => transfer::Transfer,
            SystemProgramInstruction::CreateAccountWithSeed => create_account_with_seed::CreateAccountWithSeed,
            SystemProgramInstruction::AdvanceNonceAccount => advance_nonce_account::AdvanceNonceAccount,
            SystemProgramInstruction::WithdrawNonceAccount => withdraw_nonce_account::WithdrawNonceAccount,
            SystemProgramInstruction::InitializeNonceAccount => initialize_nonce_account::InitializeNonceAccount,
            SystemProgramInstruction::AuthorizeNonceAccount => authorize_nonce_account::AuthorizeNonceAccount,
            SystemProgramInstruction::Allocate => allocate::Allocate,
            SystemProgramInstruction::AllocateWithSeed => allocate_with_seed::AllocateWithSeed,
            SystemProgramInstruction::AssignWithSeed => assign_with_seed::AssignWithSeed,
            SystemProgramInstruction::TransferWithSeed => transfer_with_seed::TransferWithSeed,
            SystemProgramInstruction::UpgradeNonceAccount => upgrade_nonce_account::UpgradeNonceAccount,
        )
    }
}
