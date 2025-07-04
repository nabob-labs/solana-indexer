use crate::PROGRAM_ID;

use super::SplAssociatedTokenAccountDecoder;
pub mod create;
pub mod create_idempotent;
pub mod recover_nested;

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
pub enum SplAssociatedTokenAccountInstruction {
    Create(create::Create),
    CreateIdempotent(create_idempotent::CreateIdempotent),
    RecoverNested(recover_nested::RecoverNested),
}

impl solana_indexer_core::instruction::InstructionDecoder<'_> for SplAssociatedTokenAccountDecoder {
    type InstructionType = SplAssociatedTokenAccountInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<solana_indexer_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        solana_indexer_core::try_decode_instructions!(instruction,
            SplAssociatedTokenAccountInstruction::Create => create::Create,
            SplAssociatedTokenAccountInstruction::CreateIdempotent => create_idempotent::CreateIdempotent,
            SplAssociatedTokenAccountInstruction::RecoverNested => recover_nested::RecoverNested,
        )
    }
}
