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

//! rpc integration tests.
use std::sync::Arc;

use accounts::AccountProvider;
use ethcore::client::{Client, ClientConfig, ChainInfo, ImportBlock};
use ethcore::ethereum;
use ethcore::miner::Miner;
use ethcore::spec::{Genesis, Spec};
use ethcore::test_helpers;
use ethcore::verification::VerifierType;
use ethcore::verification::queue::kind::blocks::Unverified;
use ethjson::blockchain::BlockChain;
use ethjson::spec::ForkSpec;
use io::IoChannel;
use miner::external::ExternalMiner;
use parity_runtime::Runtime;
use parking_lot::Mutex;

use jsonrpc_core::IoHandler;
use v1::helpers::dispatch::{self, FullDispatcher};
use v1::helpers::nonce;
use v1::impls::{EthClient, EthClientOptions, SigningUnsafeClient};
use v1::metadata::Metadata;
use v1::tests::helpers::{TestSnapshotService, TestSyncProvider, Config};
use v1::traits::{Eth, EthSigning};

fn account_provider() -> Arc<AccountProvider> {
	Arc::new(AccountProvider::transient_provider())
}

fn sync_provider() -> Arc<TestSyncProvider> {
	Arc::new(TestSyncProvider::new(Config {
		network_id: 3,
		num_peers: 120,
	}))
}

fn miner_service(spec: &Spec) -> Arc<Miner> {
	Arc::new(Miner::new_for_tests(spec, None))
}

fn snapshot_service() -> Arc<TestSnapshotService> {
	Arc::new(TestSnapshotService::new())
}

fn make_spec(chain: &BlockChain) -> Spec {
	let genesis = Genesis::from(chain.genesis());
	let mut spec = ethereum::new_frontier_test();
	let state = chain.pre_state.clone().into();
	spec.set_genesis_state(state).expect("unable to set genesis state");
	spec.overwrite_genesis_params(genesis);
	assert!(spec.is_state_root_valid());
	spec
}

pub struct EthTester {
	_miner: Arc<Miner>,
	_runtime: Runtime,
	_snapshot: Arc<TestSnapshotService>,
	pub accounts: Arc<AccountProvider>,
	pub client: Arc<Client>,
	pub handler: IoHandler<Metadata>,
}

impl EthTester {
	pub fn from_chain(chain: &BlockChain) -> Self {
		let tester = if ::ethjson::blockchain::Engine::NoProof == chain.engine {
			let mut config = ClientConfig::default();
			config.verifier_type = VerifierType::CanonNoSeal;
			config.check_seal = false;
			Self::from_spec_conf(make_spec(chain), config)
		} else {
			Self::from_spec(make_spec(chain))
		};

		for b in chain.blocks_rlp() {
			if let Ok(block) = Unverified::from_rlp(b) {
				let _ = tester.client.import_block(block);
				tester.client.flush_queue();
				tester.client.import_verified_blocks();
			}
		}

		tester.client.flush_queue();

		assert!(tester.client.chain_info().best_block_hash == chain.best_block.clone().into());
		tester
	}

	pub fn from_spec(spec: Spec) -> Self {
		let config = ClientConfig::default();
		Self::from_spec_conf(spec, config)
	}

	pub fn from_spec_conf(spec: Spec, config: ClientConfig) -> Self {
		let runtime = Runtime::with_thread_count(1);
		let account_provider = account_provider();
		let ap = account_provider.clone();
		let accounts = Arc::new(move || ap.accounts().unwrap_or_default()) as _;
		let miner_service = miner_service(&spec);
		let snapshot_service = snapshot_service();

		let client = Client::new(
			config,
			&spec,
			test_helpers::new_db(),
			miner_service.clone(),
			IoChannel::disconnected(),
		).unwrap();
		let sync_provider = sync_provider();
		let external_miner = Arc::new(ExternalMiner::default());

		let eth_client = EthClient::new(
			&client,
			&snapshot_service,
			&sync_provider,
			&accounts,
			&miner_service,
			&external_miner,
			EthClientOptions {
				pending_nonce_from_queue: false,
				allow_pending_receipt_query: true,
				send_block_number_in_get_work: true,
				gas_price_percentile: 50,
				allow_experimental_rpcs: true,
				allow_missing_blocks: false,
			},
		);

		let reservations = Arc::new(Mutex::new(nonce::Reservations::new(runtime.executor())));

		let dispatcher = FullDispatcher::new(client.clone(), miner_service.clone(), reservations, 50);
		let signer = Arc::new(dispatch::Signer::new(account_provider.clone())) as _;
		let eth_sign = SigningUnsafeClient::new(
			&signer,
			dispatcher,
		);

		let mut handler = IoHandler::default();
		handler.extend_with(eth_client.to_delegate());
		handler.extend_with(eth_sign.to_delegate());

		EthTester {
			_miner: miner_service,
			_runtime: runtime,
			_snapshot: snapshot_service,
			accounts: account_provider,
			client: client,
			handler: handler,
		}
	}
}

#[test]
fn harness_works() {
	let chain: BlockChain = extract_chain!("BlockchainTests/bcWalletTest/wallet2outOf3txs");
	let _ = EthTester::from_chain(&chain);
}
