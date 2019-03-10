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

use ethereum_types::{Address, H256};
use std::collections::{BTreeMap, HashSet};

/// TODO:
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vote {
	/// The address that is signing this vote
	pub signer: Address,
	/// Block number where the vote was casted
	pub block: u64,
	/// Address being voted to change its authorization
	pub address: Address,
	/// Indicates if the vote will authorize or not
	pub authorize: bool,
}

/// TODO:
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tally {
	/// Indicates if the vote authorizes or bans
	pub authorize: bool,
	/// Number of votes
	pub votes: u64,
}

/// TODO:
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
	// Block number
	pub number: u64,
	// Block hash
	pub hash: H256,
	/// Authorized Signers
	pub signers: HashSet<Address>,
	/// Recent Signers (spam protection)
	//FIXME: send a PR to Geth to correct the API typo (recents vs recent)
	pub recents: BTreeMap<u64, Address>,
	/// List of votes (chronologically sorted)
	pub votes: Vec<Vote>,
	/// Vote tally
	pub tally: BTreeMap<Address, Tally>,
}

#[cfg(test)]
mod tests {
	use serde_json;
	use v1::types::clique::{Tally, Vote};
	use ethereum_types::{Address, H160, H256};
	use v1::types::Snapshot;
	use std::collections::{BTreeMap, HashSet};

	fn mock_address(i: u64) -> Address {
		H160::from(H256::from(i))
	}

	#[test]
	fn test_tally_serialize() {
		let t1 = Tally { authorize: false, votes: 12 };
		let t2 = Tally { authorize: true, votes: 8 };

		let out1 = serde_json::to_string(&t1).unwrap();
		let out2 = serde_json::to_string(&t2).unwrap();

		assert_eq!(out1, r#"{"authorize":false,"votes":12}"#);
		assert_eq!(out2, r#"{"authorize":true,"votes":8}"#);
	}

	#[test]
	fn test_vote_serialize() {
		let v1 = Vote {
			signer: mock_address(0x77),
			block: 123,
			address: mock_address(0x88),
			authorize: true,
		};

		let out1 = serde_json::to_string(&v1).unwrap();

		assert_eq!(out1, r#"{"signer":"0x0000000000000000000000000000000000000077","block":123,"address":"0x0000000000000000000000000000000000000088","authorize":true}"#);
	}

	#[test]
	fn test_snapshot_serialize() {
		let mut signers = HashSet::new();
		signers.insert(mock_address(0x5001));

		let mut recent = BTreeMap::new();
		recent.insert(1u64, mock_address(0x001));
		recent.insert(1u64, mock_address(0x002));

		let v1 = Vote {
			signer: mock_address(0x274),
			block: 123,
			address: mock_address(0x632),
			authorize: true,
		};

		let mut tally_map = BTreeMap::new();
		tally_map.insert(mock_address(0x123), Tally { authorize: false, votes: 33 });
		tally_map.insert(mock_address(0x456), Tally { authorize: false, votes: 44 });

		let s1 = Snapshot {
			number: 9876,
			hash: H256::from(0x234),
			signers: signers,
			recents: recent,
			votes: vec![v1],
			tally: tally_map,
		};

		let out1 = serde_json::to_string(&s1).unwrap();

		let expected = r#"{"number":9876,"hash":"0x0000000000000000000000000000000000000000000000000000000000000234","signers":["0x0000000000000000000000000000000000005001"],"recents":{"1":"0x0000000000000000000000000000000000000002"},"votes":[{"signer":"0x0000000000000000000000000000000000000274","block":123,"address":"0x0000000000000000000000000000000000000632","authorize":true}],"tally":{"0x0000000000000000000000000000000000000123":{"authorize":false,"votes":33},"0x0000000000000000000000000000000000000456":{"authorize":false,"votes":44}}}"#;

		assert_eq!(expected, out1);
	}
}
