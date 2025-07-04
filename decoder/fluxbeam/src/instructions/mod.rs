use crate::PROGRAM_ID;

use super::FluxbeamDecoder;
pub mod deposit_all_token_types;
pub mod deposit_single_token_type_exact_amount_in;
pub mod initialize;
pub mod swap;
pub mod withdraw_all_token_types;
pub mod withdraw_single_token_type_exact_amount_out;

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
pub enum FluxbeamInstruction {
    Initialize(initialize::Initialize),
    Swap(swap::Swap),
    DepositAllTokenTypes(deposit_all_token_types::DepositAllTokenTypes),
    WithdrawAllTokenTypes(withdraw_all_token_types::WithdrawAllTokenTypes),
    DepositSingleTokenTypeExactAmountIn(
        deposit_single_token_type_exact_amount_in::DepositSingleTokenTypeExactAmountIn,
    ),
    WithdrawSingleTokenTypeExactAmountOut(
        withdraw_single_token_type_exact_amount_out::WithdrawSingleTokenTypeExactAmountOut,
    ),
}

impl solana_indexer_core::instruction::InstructionDecoder<'_> for FluxbeamDecoder {
    type InstructionType = FluxbeamInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<solana_indexer_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        solana_indexer_core::try_decode_instructions!(instruction,
            FluxbeamInstruction::Initialize => initialize::Initialize,
            FluxbeamInstruction::Swap => swap::Swap,
            FluxbeamInstruction::DepositAllTokenTypes => deposit_all_token_types::DepositAllTokenTypes,
            FluxbeamInstruction::WithdrawAllTokenTypes => withdraw_all_token_types::WithdrawAllTokenTypes,
            FluxbeamInstruction::DepositSingleTokenTypeExactAmountIn => deposit_single_token_type_exact_amount_in::DepositSingleTokenTypeExactAmountIn,
            FluxbeamInstruction::WithdrawSingleTokenTypeExactAmountOut => withdraw_single_token_type_exact_amount_out::WithdrawSingleTokenTypeExactAmountOut,
        )
    }
}
