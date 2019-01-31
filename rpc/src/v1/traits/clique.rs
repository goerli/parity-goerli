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

//! SecretStore-specific rpc interface.

use std::collections::BTreeSet;
use jsonrpc_core::Result;
use ethkey::Password;

use v1::types::{BlockNumber, H256};

build_rpc_trait! {

	pub trait Clique {
		
		#[rpc(name = "clique_getSnapshot")]
		fn get_snapshot(&self, BlockNumber) -> Result<EncryptedDocumentKey>;

	
		#[rpc(name = "clique_getSnapshotAtHash")]
		fn get_snapshot_at_hash(&self, H256) -> Result<Bytes>;

		#[rpc(name = "clique_getSigners")]
		fn get_signers(&self, BlockNumber) -> Result<Bytes>;

		#[rpc(name = "clique_getSignersAtHash")]
		fn get_signers_at_hash(&self, H256) -> Result<Bytes>;

		#[rpc(name = "clique_proposals")]
		fn proposals(&self, BTreeSet<H512>) -> Result<H256>;

		#[rpc(name = "clique_propose")]
		fn propose(&self) -> Result<Bytes>;

        #[rpc(name = "clique_discard")]
		fn discard(&self, H160, Password, H256) -> Result<Bytes>;
	}
}
