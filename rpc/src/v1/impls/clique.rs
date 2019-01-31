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


use v1::traits::SecretStore;
use v1::traits::Clique;

use v1::types::{H160, H256, H512, Bytes, EncryptedDocumentKey};
use ethkey::Password;

use ethcore::snapshot::{SnapshotService, RestorationStatus};



/// Parity implementation.
pub struct CliqueClient<C, M, U> {
	client: Arc<C>,
	miner: Arc<M>,
	updater: Arc<U>,
	sync: Arc<SyncProvider>,
	net: Arc<ManageNetwork>,
	accounts: Arc<AccountProvider>,
	logger: Arc<RotatingLogger>,
	settings: Arc<NetworkSettings>,
	signer: Option<Arc<SignerService>>,
	ws_address: Option<Host>,
	snapshot: Option<Arc<SnapshotService>>,
}

impl<C, M, U> CliqueClient<C, M, U> where
	C: BlockChainClient,
{
	/// Creates new `ParityClient`.
	pub fn new(
		client: Arc<C>,
		miner: Arc<M>,
		sync: Arc<SyncProvider>,
		updater: Arc<U>,
		net: Arc<ManageNetwork>,
		accounts: Arc<AccountProvider>,
		logger: Arc<RotatingLogger>,
		settings: Arc<NetworkSettings>,
		signer: Option<Arc<SignerService>>,
		ws_address: Option<Host>,
		snapshot: Option<Arc<SnapshotService>>,
	) -> Self {
		CliqueClient {
			client,
			miner,
			sync,
			updater,
			net,
			accounts,
			logger,
			settings,
			signer,
			ws_address,
			snapshot,
		}
	}
}

impl Clique for CliqueClient {
    pub fn new() -> Self {
        
    }

    fn get_snapshot(&self, BlockNumber: u32) -> Self {
        self.snapshot.
    }


}
