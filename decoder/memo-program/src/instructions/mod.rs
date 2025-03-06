use crate::MemoProgramDecoder;
use solana_indexer_core::instruction::DecodedInstruction;

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
pub enum MemoProgramInstruction {
    Memo(Vec<u8>),
}

impl<'a> solana_indexer_core::instruction::InstructionDecoder<'a> for MemoProgramDecoder {
    type InstructionType = MemoProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        if instruction.program_id != spl_memo::ID {
            return None;
        }

        Some(DecodedInstruction {
            data: MemoProgramInstruction::Memo(instruction.data.clone()),
            program_id: instruction.program_id,
            accounts: instruction.accounts.clone(),
        })
    }
}
