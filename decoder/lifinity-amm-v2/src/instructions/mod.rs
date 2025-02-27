use crate::PROGRAM_ID;

use super::LifinityAmmV2Decoder;
pub mod deposit_all_token_types;
pub mod swap;
pub mod withdraw_all_token_types;

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
pub enum LifinityAmmV2Instruction {
    Swap(swap::Swap),
    DepositAllTokenTypes(deposit_all_token_types::DepositAllTokenTypes),
    WithdrawAllTokenTypes(withdraw_all_token_types::WithdrawAllTokenTypes),
}

impl<'a> solana_indexer_core::instruction::InstructionDecoder<'a> for LifinityAmmV2Decoder {
    type InstructionType = LifinityAmmV2Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<solana_indexer_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        solana_indexer_core::try_decode_instructions!(instruction,
            LifinityAmmV2Instruction::Swap => swap::Swap,
            LifinityAmmV2Instruction::DepositAllTokenTypes => deposit_all_token_types::DepositAllTokenTypes,
            LifinityAmmV2Instruction::WithdrawAllTokenTypes => withdraw_all_token_types::WithdrawAllTokenTypes,
        )
    }
}
