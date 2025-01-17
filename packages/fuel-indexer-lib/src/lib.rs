//! # fuel_indexer_lib
//!
//! A collection of utilities used by the various `fuel-indexer-*` crates.

#![deny(unused_crate_dependencies)]
pub mod config;
pub mod constants;
pub mod defaults;
pub mod graphql;
pub mod manifest;
pub mod utils;

use proc_macro2::TokenStream;
use quote::quote;

/// Max size of Postgres array types.
pub const MAX_ARRAY_LENGTH: usize = 2500;

/// The source of execution for the indexer.
#[derive(Default, Clone, Debug)]
pub enum ExecutionSource {
    /// The indexer is being executed as a standalone binary.
    Native,

    /// The indexer is being executed in a WASM runtime.
    #[default]
    Wasm,
}

impl ExecutionSource {
    pub fn async_awaitness(&self) -> (TokenStream, TokenStream) {
        match self {
            Self::Native => (quote! {async}, quote! {.await}),
            Self::Wasm => (quote! {}, quote! {}),
        }
    }
}

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmIndexerError {
    DeserializationError = 0,
    SerializationError,
    PutObjectError,
    UnableToSaveListType,
    UninitializedMemory,
    KillSwitch,
    DatabaseError,
    MissingBlocksError,
    InvalidLogLevel,
    GetObjectIdFailed,
    GetStringFailed,
    AllocMissing,
    AllocFailed,
    Panic,
    GeneralError,
}

impl From<u32> for WasmIndexerError {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::DeserializationError,
            1 => Self::SerializationError,
            2 => Self::PutObjectError,
            3 => Self::UnableToSaveListType,
            4 => Self::UninitializedMemory,
            5 => Self::KillSwitch,
            6 => Self::DatabaseError,
            7 => Self::MissingBlocksError,
            8 => Self::InvalidLogLevel,
            9 => Self::GetObjectIdFailed,
            10 => Self::GetStringFailed,
            11 => Self::AllocMissing,
            12 => Self::AllocFailed,
            13 => Self::Panic,
            _ => Self::GeneralError,
        }
    }
}

impl std::fmt::Display for WasmIndexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerializationError => {
                write!(f, "Failed to serialize object")
            }
            Self::DeserializationError => {
                write!(f, "Failed to deserialize object")
            }
            Self::UnableToSaveListType => {
                write!(f, "Failed to save list")
            }
            Self::PutObjectError => {
                write!(f, "Failed to save object")
            }
            Self::UninitializedMemory => {
                write!(f, "Failed to create MemoryView for indexer")
            }
            Self::KillSwitch => {
                write!(
                    f,
                    "Indexer kill switch has been triggered. Indexer will halt"
                )
            }
            Self::DatabaseError => {
                write!(f, "Failed performing a database operation")
            }
            Self::MissingBlocksError => {
                write!(f, "Some blocks are missing")
            }
            Self::InvalidLogLevel => {
                write!(f, "Invalid log level")
            }
            Self::GetObjectIdFailed => {
                write!(f, "get_object_id failed")
            }
            Self::GetStringFailed => {
                write!(f, "get_string failed")
            }
            Self::AllocMissing => {
                write!(f, "alloc is missing")
            }
            Self::AllocFailed => {
                write!(f, "alloc failed")
            }
            Self::Panic => {
                write!(f, "Panic")
            }
            Self::GeneralError => write!(f, "Some unspecified WASM error occurred"),
        }
    }
}

/// Return a fully qualified indexer namespace.
pub fn fully_qualified_namespace(namespace: &str, identifier: &str) -> String {
    format!("{}_{}", namespace, identifier)
}

/// Return the name of the join table for the given entities.
pub fn join_table_name(a: &str, b: &str) -> String {
    format!("{}s_{}s", a, b)
}

/// Return the name of each TypeDefinition in the join table.
pub fn join_table_typedefs_name(join_table_name: &str) -> (String, String) {
    let mut parts = join_table_name.split('_');
    let a = parts.next().unwrap();
    let b = parts.next().unwrap();

    // Trim the plural 's' from the end of the TypeDefinition name.
    (a[0..a.len() - 1].to_string(), b[0..b.len() - 1].to_string())
}
