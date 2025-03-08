use crate::PROGRAM_ID;

use super::JupiterDcaDecoder;
pub mod close_dca;
pub mod closed_event;
pub mod collected_fee_event;
pub mod deposit;
pub mod deposit_event;
pub mod end_and_close;
pub mod filled_event;
pub mod fulfill_dlmm_fill;
pub mod fulfill_flash_fill;
pub mod initiate_dlmm_fill;
pub mod initiate_flash_fill;
pub mod open_dca;
pub mod open_dca_v2;
pub mod opened_event;
pub mod transfer;
pub mod withdraw;
pub mod withdraw_event;
pub mod withdraw_fees;

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
pub enum JupiterDcaInstruction {
    OpenDca(open_dca::OpenDca),
    OpenDcaV2(open_dca_v2::OpenDcaV2),
    CloseDca(close_dca::CloseDca),
    Withdraw(withdraw::Withdraw),
    Deposit(deposit::Deposit),
    WithdrawFees(withdraw_fees::WithdrawFees),
    InitiateFlashFill(initiate_flash_fill::InitiateFlashFill),
    FulfillFlashFill(fulfill_flash_fill::FulfillFlashFill),
    InitiateDlmmFill(initiate_dlmm_fill::InitiateDlmmFill),
    FulfillDlmmFill(fulfill_dlmm_fill::FulfillDlmmFill),
    Transfer(transfer::Transfer),
    EndAndClose(end_and_close::EndAndClose),
    CollectedFeeEvent(collected_fee_event::CollectedFeeEvent),
    FilledEvent(filled_event::FilledEvent),
    OpenedEvent(opened_event::OpenedEvent),
    ClosedEvent(closed_event::ClosedEvent),
    WithdrawEvent(withdraw_event::WithdrawEvent),
    DepositEvent(deposit_event::DepositEvent),
}

impl solana_indexer_core::instruction::InstructionDecoder<'_> for JupiterDcaDecoder {
    type InstructionType = JupiterDcaInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<solana_indexer_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        solana_indexer_core::try_decode_instructions!(instruction,
            JupiterDcaInstruction::OpenDca => open_dca::OpenDca,
            JupiterDcaInstruction::OpenDcaV2 => open_dca_v2::OpenDcaV2,
            JupiterDcaInstruction::CloseDca => close_dca::CloseDca,
            JupiterDcaInstruction::Withdraw => withdraw::Withdraw,
            JupiterDcaInstruction::Deposit => deposit::Deposit,
            JupiterDcaInstruction::WithdrawFees => withdraw_fees::WithdrawFees,
            JupiterDcaInstruction::InitiateFlashFill => initiate_flash_fill::InitiateFlashFill,
            JupiterDcaInstruction::FulfillFlashFill => fulfill_flash_fill::FulfillFlashFill,
            JupiterDcaInstruction::InitiateDlmmFill => initiate_dlmm_fill::InitiateDlmmFill,
            JupiterDcaInstruction::FulfillDlmmFill => fulfill_dlmm_fill::FulfillDlmmFill,
            JupiterDcaInstruction::Transfer => transfer::Transfer,
            JupiterDcaInstruction::EndAndClose => end_and_close::EndAndClose,
            JupiterDcaInstruction::CollectedFeeEvent => collected_fee_event::CollectedFeeEvent,
            JupiterDcaInstruction::FilledEvent => filled_event::FilledEvent,
            JupiterDcaInstruction::OpenedEvent => opened_event::OpenedEvent,
            JupiterDcaInstruction::ClosedEvent => closed_event::ClosedEvent,
            JupiterDcaInstruction::WithdrawEvent => withdraw_event::WithdrawEvent,
            JupiterDcaInstruction::DepositEvent => deposit_event::DepositEvent,
        )
    }
}

#[cfg(test)]
mod tests {
    use solana_indexer_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use solana_sdk::{instruction::AccountMeta, pubkey};

    use super::*;

    #[test]
    fn test_decode_open_dca() {
        let expected_ix = JupiterDcaInstruction::OpenDca(open_dca::OpenDca {
            application_idx: 1739688565,
            in_amount: 5000000,
            in_amount_per_cycle: 100000,
            cycle_frequency: 60,
            min_out_amount: Some(0),
            max_out_amount: Some(0),
            start_at: Some(0),
            close_wsol_in_ata: Some(false),
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("CfBLHEJkCUqn5LrST6ptAG96UrkjB5pfZFc98LUQUY3g"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ByBxpqTdJUQt5NpnJJp9GzovBmnT3hmMx1CqhtRAKaK1"),
                true,
            ),
            AccountMeta::new(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(
                pubkey!("856kq6mGby27QaDDiL4yxWZ3cPugtkRsPKVjaJnBMvzs"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HDg5BvWKaBLZUGKjPnXk2thmRUWNQpdwoj1MAG37eheK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FREVjo5Fyasm1bqFNQRnqRPZ9z12HhDsrt3kFpkh51rc"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
                false,
            ),
        ];
        let expected_arranged_accounts = open_dca::OpenDcaInstructionAccounts {
            dca: pubkey!("CfBLHEJkCUqn5LrST6ptAG96UrkjB5pfZFc98LUQUY3g"),
            user: pubkey!("ByBxpqTdJUQt5NpnJJp9GzovBmnT3hmMx1CqhtRAKaK1"),
            input_mint: pubkey!("So11111111111111111111111111111111111111112"),
            output_mint: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            user_ata: pubkey!("856kq6mGby27QaDDiL4yxWZ3cPugtkRsPKVjaJnBMvzs"),
            in_ata: pubkey!("HDg5BvWKaBLZUGKjPnXk2thmRUWNQpdwoj1MAG37eheK"),
            out_ata: pubkey!("FREVjo5Fyasm1bqFNQRnqRPZ9z12HhDsrt3kFpkh51rc"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            event_authority: pubkey!("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj"),
            program: pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
        };

        let decoder = JupiterDcaDecoder;
        let instruction = solana_indexer_test_utils::read_instruction("tests/fixtures/open_dca_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            open_dca::OpenDca::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_open_dca_v2() {
        let expected_ix = JupiterDcaInstruction::OpenDcaV2(open_dca_v2::OpenDcaV2 {
            application_idx: 1741074974,
            in_amount: 2500000021,
            in_amount_per_cycle: 100000000,
            cycle_frequency: 86400,
            min_out_amount: Some(108695),
            max_out_amount: Some(131578),
            start_at: Some(0),
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("4nsDuDcB47yRPBxtuhSNDxUynTXEqGWHG3tmY7H2tGVW"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DdNbmJSE6EJsbXcHkZhfsyqhbLzGYaRpKa3nxufeURuu"),
                true,
            ),
            AccountMeta::new(
                pubkey!("DdNbmJSE6EJsbXcHkZhfsyqhbLzGYaRpKa3nxufeURuu"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FQtvkDXHNjsx2v9XYnTw9pASHJPqHycKd5cyds6h7NeM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7raHTbUE3SnxmeXUcTzMqPjsBunz7VEDcc4cCagZ8njz"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5NKVYpXyAgPDRfBZSWXPG657UZvaQXY8n2pjBTF9L4Gq"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
                false,
            ),
        ];
        let expected_arranged_accounts = open_dca_v2::OpenDcaV2InstructionAccounts {
            dca: pubkey!("4nsDuDcB47yRPBxtuhSNDxUynTXEqGWHG3tmY7H2tGVW"),
            user: pubkey!("DdNbmJSE6EJsbXcHkZhfsyqhbLzGYaRpKa3nxufeURuu"),
            payer: pubkey!("DdNbmJSE6EJsbXcHkZhfsyqhbLzGYaRpKa3nxufeURuu"),
            input_mint: pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"),
            output_mint: pubkey!("3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh"),
            user_ata: pubkey!("FQtvkDXHNjsx2v9XYnTw9pASHJPqHycKd5cyds6h7NeM"),
            in_ata: pubkey!("7raHTbUE3SnxmeXUcTzMqPjsBunz7VEDcc4cCagZ8njz"),
            out_ata: pubkey!("5NKVYpXyAgPDRfBZSWXPG657UZvaQXY8n2pjBTF9L4Gq"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            event_authority: pubkey!("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj"),
            program: pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
        };

        let decoder = JupiterDcaDecoder;
        let instruction = solana_indexer_test_utils::read_instruction("tests/fixtures/open_dca_v2_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            open_dca_v2::OpenDcaV2::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_close_dca() {
        let expected_ix = JupiterDcaInstruction::CloseDca(close_dca::CloseDca {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("AT8gbypqBtxnDFN8SQJSCiJQMQeygvrLCSbFCz7xn8p3"),
                true,
            ),
            AccountMeta::new(
                pubkey!("CswSqhPMvArgMWJgGiFLJXjYRUTxdcA8VSmSKEdbSx2f"),
                false,
            ),
            AccountMeta::new(
                pubkey!("bVc4J78grC9EwsTUcsTD2pT3XqveaoeKhhJ8YXi1Hgh"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Ai3RepbHcstdm9nessRvdvKrSVoetEcnSUGHjnaNXkQM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("1BUoGh66oJSrmcg8y2oSoSFvrQ8NLHAhwdY7P6D4TkK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("mMsTJbo8yN25VSMxnfGwZ8zDbh6pTFYf5LrGLsBY3R8"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AT8gbypqBtxnDFN8SQJSCiJQMQeygvrLCSbFCz7xn8p3"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CswSqhPMvArgMWJgGiFLJXjYRUTxdcA8VSmSKEdbSx2f"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
                false,
            ),
        ];
        let expected_arranged_accounts = close_dca::CloseDcaInstructionAccounts {
            user: pubkey!("AT8gbypqBtxnDFN8SQJSCiJQMQeygvrLCSbFCz7xn8p3"),
            dca: pubkey!("CswSqhPMvArgMWJgGiFLJXjYRUTxdcA8VSmSKEdbSx2f"),
            input_mint: pubkey!("bVc4J78grC9EwsTUcsTD2pT3XqveaoeKhhJ8YXi1Hgh"),
            output_mint: pubkey!("Ai3RepbHcstdm9nessRvdvKrSVoetEcnSUGHjnaNXkQM"),
            in_ata: pubkey!("1BUoGh66oJSrmcg8y2oSoSFvrQ8NLHAhwdY7P6D4TkK"),
            out_ata: pubkey!("mMsTJbo8yN25VSMxnfGwZ8zDbh6pTFYf5LrGLsBY3R8"),
            user_in_ata: pubkey!("AT8gbypqBtxnDFN8SQJSCiJQMQeygvrLCSbFCz7xn8p3"),
            user_out_ata: pubkey!("CswSqhPMvArgMWJgGiFLJXjYRUTxdcA8VSmSKEdbSx2f"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            event_authority: pubkey!("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj"),
            program: pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
        };

        let decoder = JupiterDcaDecoder;
        let instruction = solana_indexer_test_utils::read_instruction("tests/fixtures/close_dca_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            close_dca::CloseDca::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_withdraw_fees() {
        let expected_ix =
            JupiterDcaInstruction::WithdrawFees(withdraw_fees::WithdrawFees { amount: 1193884104 });
        let expected_accounts = vec![
            AccountMeta::new(pubkey!("JTFeFAf1EKzA74pe2v5pZ6FaJ8kfLuvtt6Frxfyx8QY"), true),
            AccountMeta::new_readonly(
                pubkey!("METAewgxyPbgwsseH8T16a39CQ5VyVxZi9zXiDPY18m"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("CpoD6tWAsMDeyvVG2q2rD1JbDY6d4AujnvAn2NdrhZV2"),
                false,
            ),
            AccountMeta::new(
                pubkey!("B8vjsiRcXYsq2yphD746u96ifQXJqhEBf42uTC3K9LFZ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HyFo8JDAqnCpj6P1Q6ATPXrppP4Ug91MqW1WFVrUfcY6"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
        ];
        let expected_arranged_accounts = withdraw_fees::WithdrawFeesInstructionAccounts {
            admin: pubkey!("JTFeFAf1EKzA74pe2v5pZ6FaJ8kfLuvtt6Frxfyx8QY"),
            mint: pubkey!("METAewgxyPbgwsseH8T16a39CQ5VyVxZi9zXiDPY18m"),
            fee_authority: pubkey!("CpoD6tWAsMDeyvVG2q2rD1JbDY6d4AujnvAn2NdrhZV2"),
            program_fee_ata: pubkey!("B8vjsiRcXYsq2yphD746u96ifQXJqhEBf42uTC3K9LFZ"),
            admin_fee_ata: pubkey!("HyFo8JDAqnCpj6P1Q6ATPXrppP4Ug91MqW1WFVrUfcY6"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
        };

        let decoder = JupiterDcaDecoder;
        let instruction =
            solana_indexer_test_utils::read_instruction("tests/fixtures/withdraw_fees_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            withdraw_fees::WithdrawFees::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initiate_flash_fill() {
        let expected_ix =
            JupiterDcaInstruction::InitiateFlashFill(initiate_flash_fill::InitiateFlashFill {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("JD25qVdtd65FoiXNmR89JjmoJdYk9sjYQeSTZAALFiMy"),
                true,
            ),
            AccountMeta::new(
                pubkey!("ArmGhb7enKa3kq11qUi7bPVpX8r2ZQfsPTiZWqZfGosH"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6dKCoWjpj5MFU5gWDEFdpUUeBasBLK3wLEwhUzQPAa1e"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BeEZ5YRyD7Wnoasygh7tRGZpchH4rMbR7qjKP6rEF14i"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7VKRDWs6BBei2W6qC5sLM2DpoVEu1WeijtuAqhFMcdkF"),
                false,
            ),
            AccountMeta::new(
                pubkey!("93KgX1YnywD3qx1YnLxznyWGLQAiiUJ9TgRpqp4c4F5Z"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Sysvar1nstructions1111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
        ];
        let expected_arranged_accounts =
            initiate_flash_fill::InitiateFlashFillInstructionAccounts {
                keeper: pubkey!("JD25qVdtd65FoiXNmR89JjmoJdYk9sjYQeSTZAALFiMy"),
                dca: pubkey!("ArmGhb7enKa3kq11qUi7bPVpX8r2ZQfsPTiZWqZfGosH"),
                input_mint: pubkey!("6dKCoWjpj5MFU5gWDEFdpUUeBasBLK3wLEwhUzQPAa1e"),
                keeper_in_ata: pubkey!("BeEZ5YRyD7Wnoasygh7tRGZpchH4rMbR7qjKP6rEF14i"),
                in_ata: pubkey!("7VKRDWs6BBei2W6qC5sLM2DpoVEu1WeijtuAqhFMcdkF"),
                out_ata: pubkey!("93KgX1YnywD3qx1YnLxznyWGLQAiiUJ9TgRpqp4c4F5Z"),
                instructions_sysvar: pubkey!("Sysvar1nstructions1111111111111111111111111"),
                system_program: pubkey!("11111111111111111111111111111111"),
                token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            };

        let decoder = JupiterDcaDecoder;
        let instruction =
            solana_indexer_test_utils::read_instruction("tests/fixtures/initiate_flash_fill_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            initiate_flash_fill::InitiateFlashFill::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_fulfill_flash_fill() {
        let expected_ix =
            JupiterDcaInstruction::FulfillFlashFill(fulfill_flash_fill::FulfillFlashFill {
                repay_amount: 459532541,
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("JD25qVdtd65FoiXNmR89JjmoJdYk9sjYQeSTZAALFiMy"),
                true,
            ),
            AccountMeta::new(
                pubkey!("ArmGhb7enKa3kq11qUi7bPVpX8r2ZQfsPTiZWqZfGosH"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6dKCoWjpj5MFU5gWDEFdpUUeBasBLK3wLEwhUzQPAa1e"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BeEZ5YRyD7Wnoasygh7tRGZpchH4rMbR7qjKP6rEF14i"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7VKRDWs6BBei2W6qC5sLM2DpoVEu1WeijtuAqhFMcdkF"),
                false,
            ),
            AccountMeta::new(
                pubkey!("93KgX1YnywD3qx1YnLxznyWGLQAiiUJ9TgRpqp4c4F5Z"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("CpoD6tWAsMDeyvVG2q2rD1JbDY6d4AujnvAn2NdrhZV2"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9hHgzur8K2kZAoNXLAxz8USBxzW4nDLRkutQ9FKJBFQh"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Sysvar1nstructions1111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
                false,
            ),
        ];
        let expected_arranged_accounts = fulfill_flash_fill::FulfillFlashFillInstructionAccounts {
            keeper: pubkey!("JD25qVdtd65FoiXNmR89JjmoJdYk9sjYQeSTZAALFiMy"),
            dca: pubkey!("ArmGhb7enKa3kq11qUi7bPVpX8r2ZQfsPTiZWqZfGosH"),
            input_mint: pubkey!("6dKCoWjpj5MFU5gWDEFdpUUeBasBLK3wLEwhUzQPAa1e"),
            output_mint: pubkey!("So11111111111111111111111111111111111111112"),
            keeper_in_ata: pubkey!("BeEZ5YRyD7Wnoasygh7tRGZpchH4rMbR7qjKP6rEF14i"),
            in_ata: pubkey!("7VKRDWs6BBei2W6qC5sLM2DpoVEu1WeijtuAqhFMcdkF"),
            out_ata: pubkey!("93KgX1YnywD3qx1YnLxznyWGLQAiiUJ9TgRpqp4c4F5Z"),
            fee_authority: pubkey!("CpoD6tWAsMDeyvVG2q2rD1JbDY6d4AujnvAn2NdrhZV2"),
            fee_ata: pubkey!("9hHgzur8K2kZAoNXLAxz8USBxzW4nDLRkutQ9FKJBFQh"),
            instructions_sysvar: pubkey!("Sysvar1nstructions1111111111111111111111111"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            event_authority: pubkey!("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj"),
            program: pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
        };

        let decoder = JupiterDcaDecoder;
        let instruction =
            solana_indexer_test_utils::read_instruction("tests/fixtures/fulfill_flash_fill_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            fulfill_flash_fill::FulfillFlashFill::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_end_and_close() {
        let expected_ix = JupiterDcaInstruction::EndAndClose(end_and_close::EndAndClose {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("JD38n7ynKYcgPpF7k1BhXEeREu1KqptU93fVGy3S624k"),
                true,
            ),
            AccountMeta::new(
                pubkey!("8NpPrybBTBZfK8MMQohm3Xc7kbT7drzgmWXvMecXW9Qz"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AGYYN4AZy2fKzhWUfNMybUpt3HY1aw3eFmGA5A9cBTyE"),
                false,
            ),
            AccountMeta::new(
                pubkey!("D6XGUjFaXLDkYsww1qkjjL5bgD4i8fmo8Wi1K9oz5u3H"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FDwKNxVk7SZRZAdzU6fwY9HRS9G4whHBMvD4WcEuB7fV"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AGPdt21fwr1uF9Qemx89GReFr9kcpAT6hgm4FefhzZtb"),
                false,
            ),
            AccountMeta::new(
                pubkey!("F5GkEY9zmFLaDtrJZAu8oojgczy2KW1siCtT1GDK8Yru"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
                false,
            ),
        ];
        let expected_arranged_accounts = end_and_close::EndAndCloseInstructionAccounts {
            keeper: pubkey!("JD38n7ynKYcgPpF7k1BhXEeREu1KqptU93fVGy3S624k"),
            dca: pubkey!("8NpPrybBTBZfK8MMQohm3Xc7kbT7drzgmWXvMecXW9Qz"),
            input_mint: pubkey!("JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN"),
            output_mint: pubkey!("So11111111111111111111111111111111111111112"),
            in_ata: pubkey!("AGYYN4AZy2fKzhWUfNMybUpt3HY1aw3eFmGA5A9cBTyE"),
            out_ata: pubkey!("D6XGUjFaXLDkYsww1qkjjL5bgD4i8fmo8Wi1K9oz5u3H"),
            user: pubkey!("FDwKNxVk7SZRZAdzU6fwY9HRS9G4whHBMvD4WcEuB7fV"),
            user_out_ata: pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
            init_user_out_ata: pubkey!("AGPdt21fwr1uF9Qemx89GReFr9kcpAT6hgm4FefhzZtb"),
            intermediate_account: pubkey!("F5GkEY9zmFLaDtrJZAu8oojgczy2KW1siCtT1GDK8Yru"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            event_authority: pubkey!("Cspp27eGUDMXxPEdhmEXFVRn6Lt1L7xJyALF3nmnWoBj"),
            program: pubkey!("DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"),
        };

        let decoder = JupiterDcaDecoder;
        let instruction =
            solana_indexer_test_utils::read_instruction("tests/fixtures/end_and_close_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            end_and_close::EndAndClose::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
