//! Defines the `Error` enum and `IndexerResult` type used for error handling in
//! the `solana-indexer-core` framework.
//!
//! The `Error` enum captures various error types that can occur within the
//! framework, providing detailed error messages and support for custom error
//! handling. The `IndexerResult` type alias simplifies function signatures by
//! unifying the return type for functions that may return an `Error`.
//!
//! # Overview
//!
//! - **`Error`**: An enum representing specific error cases, from missing data
//!   in transactions to issues with data sources. Each variant provides a
//!   descriptive error message.
//! - **`IndexerResult`**: A type alias for `Result<T, Error>`, where `T` is the
//!   successful return type.
//!
//! These errors are essential for handling various scenarios that may arise
//! during data processing in the `solana-indexer-core` pipeline, including missing
//! update types, missing transaction components, and custom errors for more
//! flexible error management.
//!
//! # Notes
//!
//! - Implementing `thiserror::Error` provides automatic derivation of error
//!   display messages.
//! - Each error variant corresponds to a unique error scenario within the
//!   `solana-indexer-core` framework.

use {crate::datasource::UpdateType, thiserror::Error};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing update type in datasource")]
    MissingUpdateTypeInDatasource(UpdateType),
    #[error("Failed to receive updates({0})")]
    FailedToReceiveUpdates(String),
    #[error("Transaction missing fee payer")]
    MissingFeePayer,
    #[error("Missing inner instructions")]
    MissingInnerInstructions,
    #[error("Missing account in transaction")]
    MissingAccountInTransaction,
    #[error("Missing instruction data")]
    MissingInstructionData,
    #[error("Failed to consume datasource ({0})")]
    FailedToConsumeDatasource(String),
    #[error("Custom error: {0}")]
    Custom(String),
}

/// A type alias for `Result` with the `Error` type as the error variant.
///
/// This alias simplifies function signatures in the `solana-indexer-core` framework by
/// unifying error handling under a common type. Any function that may result in
/// an `Error` can return a `IndexerResult`, providing clear and consistent error
/// reporting.
///
/// # Example
///
/// ```ignore
/// use core::error::Error;
/// use solana_indexer_core::error::IndexerResult;
///
/// fn example_function(success: bool) -> IndexerResult<()> {
///     if success {
///         Ok(())
///     } else {
///        Err(<dyn Error>::MissingInstructionData)
///     }
/// }
///
/// match example_function(false) {
///     Ok(_) => println!("Operation succeeded."),
///     Err(e) => eprintln!("Error occurred: {}", e),
/// }
/// ```
pub type IndexerResult<T> = Result<T, Error>;
