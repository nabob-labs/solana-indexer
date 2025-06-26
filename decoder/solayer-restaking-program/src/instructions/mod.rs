use {super::SolayerRestakingProgramDecoder, crate::PROGRAM_ID};
pub mod batch_thaw_lst_accounts;
pub mod initialize;
pub mod restake;
pub mod unrestake;

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
pub enum SolayerRestakingProgramInstruction {
    Initialize(initialize::Initialize),
    Restake(restake::Restake),
    Unrestake(unrestake::Unrestake),
    BatchThawLstAccounts(batch_thaw_lst_accounts::BatchThawLstAccounts),
}

impl solana_indexer_core::instruction::InstructionDecoder<'_> for SolayerRestakingProgramDecoder {
    type InstructionType = SolayerRestakingProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<solana_indexer_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }
        solana_indexer_core::try_decode_instructions!(instruction,
            SolayerRestakingProgramInstruction::Initialize => initialize::Initialize,
            SolayerRestakingProgramInstruction::Restake => restake::Restake,
            SolayerRestakingProgramInstruction::Unrestake => unrestake::Unrestake,
            SolayerRestakingProgramInstruction::BatchThawLstAccounts => batch_thaw_lst_accounts::BatchThawLstAccounts,
        )
    }
}
