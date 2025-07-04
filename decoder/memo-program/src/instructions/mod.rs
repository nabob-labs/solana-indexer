use {crate::MemoProgramDecoder, solana_indexer_core::instruction::DecodedInstruction};

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

impl solana_indexer_core::instruction::InstructionDecoder<'_> for MemoProgramDecoder {
    type InstructionType = MemoProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&spl_memo::ID) {
            return None;
        }

        Some(DecodedInstruction {
            data: MemoProgramInstruction::Memo(instruction.data.clone()),
            program_id: instruction.program_id,
            accounts: instruction.accounts.clone(),
        })
    }
}
