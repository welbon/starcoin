// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

//! This crate defines [`trait StateView`](StateView).

use crate::access_path::AccessPath;
use anyhow::Result;
use crate::state_store::state_key::StateKey;

/// `StateView` is a trait that defines a read-only snapshot of the global state. It is passed to
/// the VM for transaction execution, during which the VM is guaranteed to read anything at the
/// given state.
pub trait StateView {
    /// Gets the state for a single access path.
    fn get(&self, access_path: &AccessPath) -> Result<Option<Vec<u8>>>;

    /// Gets the state value for given state key.
    fn get_state_value(&self, state_key: &StateKey) -> Result<Option<Vec<u8>>>;

    /// Gets states for a list of access paths.
    fn multi_get(&self, access_paths: &[AccessPath]) -> Result<Vec<Option<Vec<u8>>>>;

    /// VM needs this method to know whether the current state view is for genesis state creation.
    /// Currently TransactionPayload::WriteSet is only valid for genesis state creation.
    fn is_genesis(&self) -> bool;
}
