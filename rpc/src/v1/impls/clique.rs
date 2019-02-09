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

//! clique-specific rpc implementation.

use std::collections::BTreeSet;
use std::sync::Arc;

use crypto::DEFAULT_MAC;
use ethkey::Secret;
use ethcore::account_provider::AccountProvider;

use jsonrpc_core::Result;
use v1::helpers::errors;
use v1::helpers::secretstore::{generate_document_key, encrypt_document,
	decrypt_document, decrypt_document_with_shadow, ordered_servers_keccak};

use v1::ParityClient;
use v1::traits::SecretStore;
use v1::traits::Clique;

use v1::types::{H160, H256, H512, Bytes, EncryptedDocumentKey};
use ethkey::Password;

use ethcore::snapshot::{SnapshotService, RestorationStatus};

impl<C, M, U, S> Clique for ParityClient<C, M, U> where
	S: StateInfo + 'static,
	C: miner::BlockChainClient + BlockChainClient + StateClient<State=S> + Call<State=S> + 'static,
	M: MinerService<State=S> + 'static,
	U: UpdateService + 'static,
{
	fn get_snapshot(&self, blockNumber: BlockNumber) -> Result<()> {
		let engine = self.client.engine()

		let snaphsot = self.snapshot.block_at
	}

	fn get_snapshot_at_hash(&self, H256) -> Result<()> {

	}

	fn get_signers(&self, BlockNumber) -> Result<()> {

	}

	fn get_signers_at_hash(&self, H256) -> Result<()> {

	}

	fn proposals(&self, BTreeSet<H512>) -> Result<()> {

	}

	fn propose(&self) -> Result<()> {

	}

	fn discard(&self, H160, Password, H256) -> Result<()> {
		
	}

}
