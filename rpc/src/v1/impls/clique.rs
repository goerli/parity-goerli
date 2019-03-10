// Copyright 2015-2019 Parity Technologies (UK) Ltd.
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

//! clique rpc implementation.
use jsonrpc_core::Result;
use std::collections::BTreeMap;

use ethereum_types::{H256, Address};
use v1::traits::Clique;
use v1::types::{BlockNumber, Snapshot};

/// Clique rpc implementation.
pub struct CliqueClient;

impl CliqueClient {
	/// Creates new CliqueClient.
	pub fn new() -> Self {
		CliqueClient
	}
}

impl Clique for CliqueClient {
	fn get_snapshot(&self, block_number: BlockNumber) -> Result<Snapshot> {
		unimplemented!()
	}

	fn get_snapshot_at_hash(&self, hash: H256) -> Result<Snapshot> {
		unimplemented!()
	}

	fn get_signers(&self, block_number: BlockNumber) -> Result<Vec<Address>> {
		unimplemented!()
	}

	fn get_signers_at_hash(&self, hash: H256) -> Result<Vec<Address>> {
		unimplemented!()
	}

	fn proposals(&self) -> Result<BTreeMap<Address, bool>> {
		unimplemented!()
	}

	fn propose(&self, address: Address, auth: bool) -> Result<()> {
		unimplemented!()
	}

	fn discard(&self, address: Address) -> Result<()> {
		unimplemented!()
	}
}
