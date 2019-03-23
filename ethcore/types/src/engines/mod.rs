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

//! Engine-specific types.

use ethereum_types::{H256, Address};
use std::collections::{HashSet, BTreeMap};

pub mod epoch;

/// Fork choice.
#[derive(Debug, PartialEq, Eq)]
pub enum ForkChoice {
	/// Choose the new block.
	New,
	/// Choose the current best block.
	Old,
}

// Vote (clique)
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

/// Vote Tally for a certain block
pub struct Tally {
	/// Indicates if the vote authorizes or bans
	pub authorize: bool,
	/// Number of votes
	pub votes: u64,
}

/// Clique Snapshot for a certain block
pub struct Snapshot {
	/// Block number
	pub number : u64,
	/// Block hash
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
