use crate::PROGRAM_ID;

use super::NameDecoder;
pub mod create;
pub mod delete;
pub mod realloc;
pub mod transfer;
pub mod update;

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
pub enum NameInstruction {
    Create(create::Create),
    Update(update::Update),
    Transfer(transfer::Transfer),
    Delete(delete::Delete),
    Realloc(realloc::Realloc),
}

impl solana_indexer_core::instruction::InstructionDecoder<'_> for NameDecoder {
    type InstructionType = NameInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<solana_indexer_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        solana_indexer_core::try_decode_instructions!(instruction,
            NameInstruction::Create => create::Create,
            NameInstruction::Update => update::Update,
            NameInstruction::Transfer => transfer::Transfer,
            NameInstruction::Delete => delete::Delete,
            NameInstruction::Realloc => realloc::Realloc,
        )
    }
}
