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

//! rpc integration tests for clique

#[cfg(test)]
mod clique_tests {
	use ethjson::spec::ForkSpec;
	use v1::tests::eth_tester::EthTester;
	use ethcore::ethereum;

	#[test]
	fn clique_load_chain() {
		let chain = extract_chain!("/../tests_local/CliqueGenesis");
		let tester = EthTester::from_chain_spec(&chain, ethereum::new_goerli_test());

		assert_eq!("Clique", tester.client.engine().name());
	}

	#[test]
	fn clique_other_engine() {
		let chain = extract_chain!("BlockchainTests/bcGasPricerTest/RPC_API_Test");
		let tester = EthTester::from_chain_spec(&chain, ethereum::new_frontier_test());

		let req_block = r#"{
		"method":"clique_getSnapshot",
		"params":["0x4"],
		"id":1,
		"jsonrpc":"2.0"
		}"#;

		let have = tester.handler.handle_request_sync(req_block).unwrap_or_default();
		assert!(have.contains("The node is not running a clique engine"));
	}

	#[test]
	fn clique_get_snapshot() {
		let chain = extract_chain!("/../tests_local/CliqueGenesis");
		let tester = EthTester::from_chain_spec(&chain, ethereum::new_goerli_test());

		let req_block = r#"{
		"method":"clique_getSnapshot",
		"params":["0x4"],
		"id":1,
		"jsonrpc":"2.0"
		}"#;

		let have = tester.handler.handle_request_sync(req_block).unwrap();

		println!("{}", have);
	}

	#[test]
	fn clique_get_snapshot_at_hash() {
		// TODO: Fix/improve test

		let chain = extract_chain!("/../tests_local/CliqueGenesis");
		let tester = EthTester::from_chain_spec(&chain, ethereum::new_goerli_test());

		let req_block = r#"{
		"method":"clique_getSnapshotAtHash",
		"params":["0x4"],
		"id":1,
		"jsonrpc":"2.0"
		}"#;

		let have = tester.handler.handle_request_sync(req_block).unwrap();

		println!("{}", have);
	}


	#[test]
	fn clique_get_signers() {
		// TODO: Fix/improve test

		let chain = extract_chain!("/../tests_local/CliqueGenesis");
		let tester = EthTester::from_chain_spec(&chain, ethereum::new_goerli_test());

		let req_block = r#"{
		"method":"clique_getSigners",
		"params":["0x4"],
		"id":1,
		"jsonrpc":"2.0"
		}"#;

		let have = tester.handler.handle_request_sync(req_block).unwrap();

		println!("{}", have);

		assert!(false);
	}

	#[test]
	fn clique_get_signers_at_hash() {
		// TODO: Fix/improve test

		let chain = extract_chain!("/../tests_local/CliqueGenesis");
		let tester = EthTester::from_chain_spec(&chain, ethereum::new_goerli_test());

		let req_block = r#"{
		"method":"clique_getSignersAtHash",
		"params":["0x0000000000000000000000000000000000000000000000000000000000000234"],
		"id":1,
		"jsonrpc":"2.0"
		}"#;

		let have = tester.handler.handle_request_sync(req_block).unwrap();

		println!("{}", have);

		assert_eq!(have.contains("The node is not running a clique engine"), false);
	}

	#[test]
	fn clique_proposals() {
		// TODO: Fix/improve test

		let chain = extract_chain!("/../tests_local/CliqueGenesis");
		let tester = EthTester::from_chain_spec(&chain, ethereum::new_goerli_test());

		let req_block = r#"{
		"method":"clique_getSignersAtHash",
		"params":["0x4"],
		"id":1,
		"jsonrpc":"2.0"
		}"#;

		let have = tester.handler.handle_request_sync(req_block).unwrap();

		println!("{}", have);

		assert!(false);
	}

	#[test]
	fn clique_propose() {
		// TODO: Fix/improve test

		let chain = extract_chain!("/../tests_local/CliqueGenesis");
		let tester = EthTester::from_chain_spec(&chain, ethereum::new_goerli_test());

		let req_block = r#"{
		"method":"clique_getSignersAtHash",
		"params":["0x4"],
		"id":1,
		"jsonrpc":"2.0"
		}"#;

		let have = tester.handler.handle_request_sync(req_block).unwrap();

		println!("{}", have);

		assert!(false);
	}

	#[test]
	fn clique_discard() {
		// TODO: Fix/improve test

		let chain = extract_chain!("/../tests_local/CliqueGenesis");
		let tester = EthTester::from_chain_spec(&chain, ethereum::new_goerli_test());

		let req_block = r#"{
		"method":"clique_getSignersAtHash",
		"params":["0x4"],
		"id":1,
		"jsonrpc":"2.0"
		}"#;

		let have = tester.handler.handle_request_sync(req_block).unwrap();

		println!("{}", have);

		assert!(false);
	}
}