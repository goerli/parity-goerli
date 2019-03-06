// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// Copyright 2019 ZondaX GmbH (CH)
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

//! Clique rpc interface (Modeled after geth interface)
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use std::collections::BTreeMap;
use v1::types::{BlockNumber, H160, H256};

type Snapshot = u8;      // TODO: Matching type for Snapshots seems to be missing
type Address = H160;

/// Clique rpc interface.
#[rpc]
pub trait Clique {
    // TODO: Doc
    #[rpc(name = "clique_getSnapshot")]
    fn get_snapshot(&self, block_number: BlockNumber) -> Result<Snapshot>;

    // TODO: Doc
    #[rpc(name = "clique_getSnapshotAtHash")]
    fn get_snapshot_at_hash(&self, hash: H256) -> Result<Snapshot>;

    // TODO: Doc
    #[rpc(name = "clique_getSigners")]
    fn get_signers(&self, block_number: BlockNumber) -> Result<Vec<Address>>;

    // TODO: Doc
    #[rpc(name = "clique_getSignersAtHash")]
    fn get_signers_at_hash(&self, hash: H256) -> Result<Vec<Address>>;

    // TODO: Doc
    #[rpc(name = "clique_proposals")]
    fn proposals(&self) -> Result<BTreeMap<Address, bool>>;

    // TODO: Doc
    #[rpc(name = "clique_propose")]
    fn propose(&self, address: Address, auth: bool)-> Result<()>;

    // TODO: Doc
    #[rpc(name = "clique_discard")]
    fn discard(&self, address: Address)-> Result<()>;
}
