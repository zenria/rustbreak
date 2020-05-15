/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use thiserror::Error;
use std::io;

/// The different kinds of errors that can be returned
#[derive(Debug,  Error)]
pub enum RustbreakError {
    /// A context error when a serialization failed
    #[error( "Could not serialize the value")]
    Serialization,
    /// A context error when a deserialization failed
    #[error( "Could not deserialize the value: {0}")]
    Deserialization(#[from] anyhow::Error),
    /// This error is returned if the `Database` is poisoned. See `Database::write` for details
    #[error( "The database has been poisoned")]
    Poison,
    /// An error in the backend happened
    #[error( "The backend has encountered an error")]
    Backend,
    /// An IOError occured
    #[error("The backend has encountered an error: {0}")]
    BackendIO(#[from] io::Error),

    /// If `Database::write_safe` is used and the closure panics, this error is returned
    #[error( "The write operation paniced but got caught")]
    WritePanic,
    /// This variant should never be used. It is meant to keep this enum forward compatible.
    #[doc(hidden)]
    #[error( "You have found a secret message, please report it to the Rustbreak maintainer")]
    __Nonexhaustive,
}


/// A simple type alias for errors
pub type Result<T> = ::std::result::Result<T, RustbreakError>;
