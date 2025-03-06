//! Provides utility functions to transform transaction data into various representations
//! within the `indexer-core` framework.
//!
//! This module includes functions for extracting transaction metadata, parsing instructions,
//! and nesting instructions based on stack depth. It also offers transformations for Solana
//! transaction components into suitable formats for the framework, enabling flexible processing
//! of transaction data.
//!
//! ## Key Components
//!
//! - **Metadata Extraction**: Extracts essential transaction metadata for processing.
//! - **Instruction Parsing**: Parses both top-level and nested instructions from transactions.
//! - **Account Metadata**: Converts account data into a standardized format for transactions.
//!
//! ## Notes
//!
//! - The module supports both legacy and v0 transactions, including handling of loaded
//!   addresses and inner instructions.

use crate::{
    collection::InstructionDecoderCollection,
    datasource::TransactionUpdate,
    error::{IndexerResult, Error},
    instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
    schema::ParsedInstruction,
    transaction::TransactionMetadata,
};
use solana_sdk::{
    instruction::{AccountMeta, CompiledInstruction},
    message::{
        v0::{LoadedAddresses, LoadedMessage},
        VersionedMessage,
    },
    pubkey::Pubkey,
    reserved_account_keys::ReservedAccountKeys,
    transaction_context::TransactionReturnData,
};
use solana_transaction_status::{
    option_serializer::OptionSerializer, InnerInstruction, InnerInstructions, Reward,
    TransactionStatusMeta, TransactionTokenBalance, UiInstruction, UiLoadedAddresses,
    UiTransactionStatusMeta,
};
use std::{collections::HashSet, str::FromStr};

/// Extracts the metadata from a transaction update.
///
/// This function retrieves core metadata such as the transaction's slot, signature, and
/// fee payer from the transaction's message. It ensures that these details are available
/// and ready for further processing.
///
/// # Parameters
///
/// - `transaction_update`: The `TransactionUpdate` containing the transaction details.
///
/// # Returns
///
/// A `IndexerResult<TransactionMetadata>` which includes the slot, signature, fee payer, transaction status metadata and the version transaction message.
///
/// # Errors
///
/// Returns an error if the fee payer cannot be extracted from the transaction's account keys.
pub fn extract_transaction_metadata(
    transaction_update: &TransactionUpdate,
) -> IndexerResult<TransactionMetadata> {
    log::trace!(
        "extract_transaction_metadata(transaction_update: {:?})",
        transaction_update
    );
    let accounts = transaction_update.transaction.message.static_account_keys();

    Ok(TransactionMetadata {
        slot: transaction_update.slot,
        signature: transaction_update.signature,
        fee_payer: *accounts.get(0).ok_or(Error::MissingFeePayer)?,
        meta: transaction_update.meta.clone(),
        message: transaction_update.transaction.message.clone(),
    })
}

/// Extracts instructions with metadata from a transaction update.
///
/// This function parses both top-level and inner instructions, associating them with
/// metadata such as stack height and account information. It provides a detailed
/// breakdown of each instruction, useful for further processing.
///
/// # Parameters
///
/// - `transaction_metadata`: Metadata about the transaction from which instructions are extracted.
/// - `transaction_update`: The `TransactionUpdate` containing the transaction's data and message.
///
/// # Returns
///
/// A `IndexerResult<Vec<(InstructionMetadata, solana_sdk::instruction::Instruction)>>` containing
/// instructions along with their associated metadata.
///
/// # Errors
///
/// Returns an error if any account metadata required for instruction processing is missing.
pub fn extract_instructions_with_metadata(
    transaction_metadata: &TransactionMetadata,
    transaction_update: &TransactionUpdate,
) -> IndexerResult<Vec<(InstructionMetadata, solana_sdk::instruction::Instruction)>> {
    log::trace!(
        "extract_instructions_with_metadata(transaction_metadata: {:?}, transaction_update: {:?})",
        transaction_metadata,
        transaction_update
    );
    let message = transaction_update.transaction.message.clone();
    let meta = transaction_update.meta.clone();

    let mut instructions_with_metadata =
        Vec::<(InstructionMetadata, solana_sdk::instruction::Instruction)>::new();

    match message {
        VersionedMessage::Legacy(legacy) => {
            for (i, compiled_instruction) in legacy.instructions.iter().enumerate() {
                let program_id = *legacy
                    .account_keys
                    .get(compiled_instruction.program_id_index as usize)
                    .unwrap_or(&Pubkey::default());

                let accounts: Vec<_> = compiled_instruction
                    .accounts
                    .iter()
                    .filter_map(|account_index| {
                        let account_pubkey = legacy.account_keys.get(*account_index as usize)?;
                        Some(AccountMeta {
                            pubkey: *account_pubkey,
                            is_writable: legacy.is_maybe_writable(*account_index as usize, None),
                            is_signer: legacy.is_signer(*account_index as usize),
                        })
                    })
                    .collect();

                instructions_with_metadata.push((
                    InstructionMetadata {
                        transaction_metadata: transaction_metadata.clone(),
                        stack_height: 1,
                    },
                    solana_sdk::instruction::Instruction {
                        program_id,
                        accounts,
                        data: compiled_instruction.data.clone(),
                    },
                ));

                if let Some(inner_instructions) = &meta.inner_instructions {
                    for inner_instructions_per_tx in inner_instructions {
                        if inner_instructions_per_tx.index == i as u8 {
                            for inner_instruction in inner_instructions_per_tx.instructions.iter() {
                                let program_id = *legacy
                                    .account_keys
                                    .get(inner_instruction.instruction.program_id_index as usize)
                                    .unwrap_or(&Pubkey::default());

                                let accounts: Vec<_> = inner_instruction
                                    .instruction
                                    .accounts
                                    .iter()
                                    .filter_map(|account_index| {
                                        let account_pubkey =
                                            legacy.account_keys.get(*account_index as usize)?;
                                        return Some(AccountMeta {
                                            pubkey: *account_pubkey,
                                            is_writable: legacy
                                                .is_maybe_writable(*account_index as usize, None),
                                            is_signer: legacy.is_signer(*account_index as usize),
                                        });
                                    })
                                    .collect();

                                instructions_with_metadata.push((
                                    InstructionMetadata {
                                        transaction_metadata: transaction_metadata.clone(),
                                        stack_height: inner_instruction.stack_height.unwrap_or(1),
                                    },
                                    solana_sdk::instruction::Instruction {
                                        program_id,
                                        accounts,
                                        data: inner_instruction.instruction.data.clone(),
                                    },
                                ));
                            }
                        }
                    }
                }
            }
        }
        VersionedMessage::V0(v0) => {
            let loaded_addresses = LoadedAddresses {
                writable: meta
                    .loaded_addresses
                    .writable
                    .iter()
                    .map(|key| key.clone())
                    .collect(),
                readonly: meta
                    .loaded_addresses
                    .readonly
                    .iter()
                    .map(|key| key.clone())
                    .collect(),
            };

            let loaded_message = LoadedMessage::new(
                v0.clone(),
                loaded_addresses,
                &ReservedAccountKeys::empty_key_set(),
            );

            for (i, compiled_instruction) in v0.instructions.iter().enumerate() {
                let program_id = *loaded_message
                    .account_keys()
                    .get(compiled_instruction.program_id_index as usize)
                    .unwrap_or(&Pubkey::default());

                let accounts: Vec<AccountMeta> = compiled_instruction
                    .accounts
                    .iter()
                    .filter_map(|account_index| {
                        let account_pubkey =
                            loaded_message.account_keys().get(*account_index as usize);

                        return Some(AccountMeta {
                            pubkey: account_pubkey.map(|acc| acc.clone()).unwrap_or_default(),
                            is_writable: loaded_message.is_writable(*account_index as usize),
                            is_signer: loaded_message.is_signer(*account_index as usize),
                        });
                    })
                    .collect();

                instructions_with_metadata.push((
                    InstructionMetadata {
                        transaction_metadata: transaction_metadata.clone(),
                        stack_height: 1,
                    },
                    solana_sdk::instruction::Instruction {
                        program_id,
                        accounts,
                        data: compiled_instruction.data.clone(),
                    },
                ));

                if let Some(inner_instructions) = &meta.inner_instructions {
                    for inner_instructions_per_tx in inner_instructions {
                        if inner_instructions_per_tx.index == i as u8 {
                            for inner_instruction in inner_instructions_per_tx.instructions.iter() {
                                let program_id = *loaded_message
                                    .account_keys()
                                    .get(inner_instruction.instruction.program_id_index as usize)
                                    .unwrap_or(&Pubkey::default());

                                let accounts: Vec<AccountMeta> = inner_instruction
                                    .instruction
                                    .accounts
                                    .iter()
                                    .filter_map(|account_index| {
                                        let account_pubkey = loaded_message
                                            .account_keys()
                                            .get(*account_index as usize)
                                            .map(|pubkey| pubkey.clone())
                                            .unwrap_or_default();
                                        return Some(AccountMeta {
                                            pubkey: account_pubkey.clone(),
                                            is_writable: loaded_message
                                                .is_writable(*account_index as usize),
                                            is_signer: loaded_message
                                                .is_signer(*account_index as usize),
                                        });
                                    })
                                    .collect();

                                instructions_with_metadata.push((
                                    InstructionMetadata {
                                        transaction_metadata: transaction_metadata.clone(),
                                        stack_height: inner_instruction.stack_height.unwrap_or(1),
                                    },
                                    solana_sdk::instruction::Instruction {
                                        program_id,
                                        accounts,
                                        data: inner_instruction.instruction.data.clone(),
                                    },
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(instructions_with_metadata)
}

/// Extracts account metadata from a compiled instruction and transaction message.
///
/// This function converts each account index within the instruction into an `AccountMeta`
/// struct, providing details on account keys, signer status, and write permissions.
///
/// # Parameters
///
/// - `compiled_instruction`: The compiled instruction to extract accounts from.
/// - `message`: The transaction message containing the account keys.
///
/// # Returns
///
/// A `IndexerResult<Vec<solana_sdk::instruction::AccountMeta>>` containing metadata
/// for each account involved in the instruction.
///
/// # Errors
///
/// Returns an error if any referenced account key is missing from the transaction.

pub fn extract_account_metas(
    compiled_instruction: &solana_sdk::instruction::CompiledInstruction,
    message: &solana_sdk::message::VersionedMessage,
) -> IndexerResult<Vec<solana_sdk::instruction::AccountMeta>> {
    log::trace!(
        "extract_account_metas(compiled_instruction: {:?}, message: {:?})",
        compiled_instruction,
        message
    );
    let mut accounts = Vec::<solana_sdk::instruction::AccountMeta>::new();

    for account_index in compiled_instruction.accounts.iter() {
        accounts.push(solana_sdk::instruction::AccountMeta {
            pubkey: *message
                .static_account_keys()
                .get(*account_index as usize)
                .ok_or(Error::MissingAccountInTransaction)?,
            is_signer: message.is_signer(*account_index as usize),
            is_writable: message.is_maybe_writable(
                *account_index as usize,
                Some(
                    &message
                        .static_account_keys()
                        .into_iter()
                        .map(|pubkey| pubkey.clone())
                        .collect::<HashSet<_>>(),
                ),
            ),
        });
    }

    Ok(accounts)
}

/// Nests instructions based on stack height, producing a hierarchy of `NestedInstruction`.
///
/// This function organizes instructions into a nested structure, enabling hierarchical
/// transaction analysis. Instructions are nested according to their stack height,
/// forming a tree-like structure.
///
/// # Parameters
///
/// - `instructions`: A list of tuples containing `InstructionMetadata` and instructions.
///
/// # Returns
///
/// A vector of `NestedInstruction`, representing the instructions organized by stack depth.
pub fn nest_instructions(
    instructions: Vec<(InstructionMetadata, solana_sdk::instruction::Instruction)>,
) -> Vec<NestedInstruction> {
    log::trace!("nest_instructions(instructions: {:?})", instructions);
    let mut result = Vec::<NestedInstruction>::new();
    let mut stack = Vec::<(Vec<usize>, usize)>::new();

    for (metadata, instruction) in instructions {
        let nested_instruction = NestedInstruction {
            metadata: metadata.clone(),
            instruction,
            inner_instructions: Vec::new(),
        };

        while let Some((_, parent_stack_height)) = stack.last() {
            if metadata.stack_height as usize > *parent_stack_height {
                break;
            }
            stack.pop();
        }

        if let Some((path_to_parent, _)) = stack.last() {
            let mut current_instructions = &mut result;
            for &index in path_to_parent {
                current_instructions = &mut current_instructions[index].inner_instructions;
            }
            current_instructions.push(nested_instruction);
            let mut new_path = path_to_parent.clone();
            new_path.push(current_instructions.len() - 1);
            stack.push((new_path, metadata.stack_height as usize));
        } else {
            result.push(nested_instruction);
            let new_path = vec![result.len() - 1];
            stack.push((new_path, metadata.stack_height as usize));
        }
    }

    result
}

/// Unnests parsed instructions, producing an array of `(InstructionMetadata, DecodedInstruction<T>)` tuple
///
/// This function takes a vector of `ParsedInstruction` and unnests them into a vector of `(InstructionMetadata, DecodedInstruction<T>)` tuples.
/// It recursively processes nested instructions, increasing the stack height for each level of nesting.
///
/// # Parameters
///
/// - `transaction_metadata`: The metadata of the transaction containing the instructions.
/// - `instructions`: The vector of `ParsedInstruction` to be unnested.
/// - `stack_height`: The current stack height.
///
/// # Returns
///
/// A vector of `(InstructionMetadata, DecodedInstruction<T>)` tuples representing the unnested instructions.
pub fn unnest_parsed_instructions<T: InstructionDecoderCollection>(
    transaction_metadata: TransactionMetadata,
    instructions: Vec<ParsedInstruction<T>>,
    stack_height: u32,
) -> Vec<(InstructionMetadata, DecodedInstruction<T>)> {
    log::trace!(
        "unnest_parsed_instructions(instructions: {:?})",
        instructions
    );

    let mut result = Vec::new();

    for parsed_instruction in instructions.into_iter() {
        result.push((
            InstructionMetadata {
                transaction_metadata: transaction_metadata.clone(),
                stack_height,
            },
            parsed_instruction.instruction,
        ));
        result.extend(unnest_parsed_instructions(
            transaction_metadata.clone(),
            *parsed_instruction.inner_instructions,
            stack_height + 1,
        ));
    }

    result
}

/// Converts UI transaction metadata into `TransactionStatusMeta`.
///
/// This function transforms the user interface format of transaction metadata into
/// a more comprehensive `TransactionStatusMeta` structure suitable for backend processing.
///
/// # Parameters
///
/// - `meta_original`: The original UI format of transaction status metadata.
///
/// # Returns
///
/// A `IndexerResult<TransactionStatusMeta>` representing the full transaction status
/// with nested instructions, token balances, and rewards.
///
/// # Notes
///
/// This function handles various metadata fields, including inner instructions, token
/// balances, and rewards, providing a complete view of the transaction's effects.

pub fn transaction_metadata_from_original_meta(
    meta_original: UiTransactionStatusMeta,
) -> IndexerResult<TransactionStatusMeta> {
    log::trace!(
        "transaction_metadata_from_original_meta(meta_original: {:?})",
        meta_original
    );
    Ok(TransactionStatusMeta {
        status: meta_original.status,
        fee: meta_original.fee,
        pre_balances: meta_original.pre_balances,
        post_balances: meta_original.post_balances,
        inner_instructions: Some(
            meta_original
                .inner_instructions
                .unwrap_or_else(|| vec![])
                .iter()
                .map(|inner_instruction_group| InnerInstructions {
                    index: inner_instruction_group.index,
                    instructions: inner_instruction_group
                        .instructions
                        .iter()
                        .map(|ui_instruction| match ui_instruction {
                            UiInstruction::Compiled(compiled_ui_instruction) => {
                                let decoded_data =
                                    bs58::decode(compiled_ui_instruction.data.clone())
                                        .into_vec()
                                        .unwrap_or_else(|_| vec![]);
                                InnerInstruction {
                                    instruction: CompiledInstruction {
                                        program_id_index: compiled_ui_instruction.program_id_index,
                                        accounts: compiled_ui_instruction.accounts.clone(),
                                        data: decoded_data,
                                    },
                                    stack_height: compiled_ui_instruction.stack_height,
                                }
                            }
                            _ => {
                                log::error!("Unsupported instruction type encountered");
                                InnerInstruction {
                                    instruction: CompiledInstruction {
                                        program_id_index: 0,
                                        accounts: vec![],
                                        data: vec![],
                                    },
                                    stack_height: None,
                                }
                            }
                        })
                        .collect::<Vec<InnerInstruction>>(),
                })
                .collect::<Vec<InnerInstructions>>(),
        ),
        log_messages: Some(meta_original.log_messages.unwrap_or_else(|| vec![])),
        pre_token_balances: Some(
            meta_original
                .pre_token_balances
                .unwrap_or_else(|| vec![])
                .iter()
                .filter_map(|transaction_token_balance| {
                    if let (OptionSerializer::Some(owner), OptionSerializer::Some(program_id)) = (
                        transaction_token_balance.owner.as_ref(),
                        transaction_token_balance.program_id.as_ref(),
                    ) {
                        Some(TransactionTokenBalance {
                            account_index: transaction_token_balance.account_index,
                            mint: transaction_token_balance.mint.clone(),
                            ui_token_amount: transaction_token_balance.ui_token_amount.clone(),
                            owner: owner.to_string(),
                            program_id: program_id.to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<TransactionTokenBalance>>(),
        ),
        post_token_balances: Some(
            meta_original
                .post_token_balances
                .unwrap_or_else(|| vec![])
                .iter()
                .filter_map(|transaction_token_balance| {
                    if let (OptionSerializer::Some(owner), OptionSerializer::Some(program_id)) = (
                        transaction_token_balance.owner.as_ref(),
                        transaction_token_balance.program_id.as_ref(),
                    ) {
                        Some(TransactionTokenBalance {
                            account_index: transaction_token_balance.account_index,
                            mint: transaction_token_balance.mint.clone(),
                            ui_token_amount: transaction_token_balance.ui_token_amount.clone(),
                            owner: owner.to_string(),
                            program_id: program_id.to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<TransactionTokenBalance>>(),
        ),
        rewards: Some(
            meta_original
                .rewards
                .unwrap_or_else(|| vec![])
                .iter()
                .map(|rewards| Reward {
                    pubkey: rewards.pubkey.clone(),
                    lamports: rewards.lamports,
                    post_balance: rewards.post_balance,
                    reward_type: rewards.reward_type,
                    commission: rewards.commission,
                })
                .collect::<Vec<Reward>>(),
        ),
        loaded_addresses: {
            let loaded = meta_original
                .loaded_addresses
                .unwrap_or_else(|| UiLoadedAddresses {
                    writable: vec![],
                    readonly: vec![],
                });
            LoadedAddresses {
                writable: loaded
                    .writable
                    .iter()
                    .map(|w| Pubkey::from_str(&w).unwrap_or_default())
                    .collect::<Vec<Pubkey>>(),
                readonly: loaded
                    .readonly
                    .iter()
                    .map(|r| Pubkey::from_str(&r).unwrap_or_default())
                    .collect::<Vec<Pubkey>>(),
            }
        },
        return_data: meta_original
            .return_data
            .map(|return_data| TransactionReturnData {
                program_id: return_data.program_id.parse().unwrap_or_default(),
                data: return_data.data.0.as_bytes().to_vec(),
            }),
        compute_units_consumed: meta_original
            .compute_units_consumed
            .map(|compute_unit_consumed| compute_unit_consumed)
            .or(None),
    })
}
