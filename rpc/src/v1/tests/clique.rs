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

use ethjson::spec::ForkSpec;
use v1::tests::eth_tester::EthTester;

#[test]
fn empty_test() {
	let a = 1;
	println!("{}", a);
}

#[test]
fn eth_get_signers() {
	let chain = extract_chain!("BlockchainTests/bcGasPricerTest/RPC_API_Test");
	let tester = EthTester::from_chain(&chain);

	let req_block = r#"{
		"method":"clique_getSigners",
		"params":["0x4"],
		"id":1,
		"jsonrpc":"2.0"
		}"#;

	let have = tester.handler.handle_request_sync(req_block).unwrap();

	println!("{}", have);

//	let want = "";
//	assert_eq!(want, have);
}
