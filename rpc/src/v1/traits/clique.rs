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

use ethereum_types::{H256, Address};

use v1::types::{BlockNumber, Snapshot};

/// Clique rpc interface.
#[rpc]
pub trait Clique {
    // retrieves the state snapshot at a given block.
    #[rpc(name = "clique_getSnapshot")]
    fn get_snapshot(&self, block_number: BlockNumber) -> Result<Snapshot>;

    // retrieves the state snapshot at a given hash.
    #[rpc(name = "clique_getSnapshotAtHash")]
    fn get_snapshot_at_hash(&self, hash: H256) -> Result<Snapshot>;

    // retrieves the list of authorized signers at the specified block.
    #[rpc(name = "clique_getSigners")]
    fn get_signers(&self, block_number: BlockNumber) -> Result<Vec<Address>>;

    // retrieves the list of authorized signers at the specified hash.
    #[rpc(name = "clique_getSignersAtHash")]
    fn get_signers_at_hash(&self, hash: H256) -> Result<Vec<Address>>;

    // returns the current proposals the node tries to uphold and vote on.
    #[rpc(name = "clique_proposals")]
    fn proposals(&self) -> Result<BTreeMap<Address, bool>>;

    // injects a new authorization proposal that the signer will attempt to push through.
    #[rpc(name = "clique_propose")]
    fn propose(&self, address: Address, auth: bool)-> Result<()>;

    // drops a currently running proposal, stopping the signer from casting
	// further votes (either for or against).
    #[rpc(name = "clique_discard")]
    fn discard(&self, address: Address)-> Result<()>;
}
