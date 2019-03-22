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
use std::sync::Arc;
use ethcore::client::{EngineInfo, BlockId};

use v1::helpers::errors::{clique_not_running, unimplemented};

/// Clique rpc implementation.
pub struct CliqueClient<EI>
{
	engine_info: Arc<EI>,
}

impl<EI> CliqueClient<EI> where EI: EngineInfo + Sync + Send {
	/// Creates new CliqueClient.
	pub fn new(engine_info: &Arc<EI>) -> Self {
		CliqueClient {
			engine_info: engine_info.clone(),
		}
	}

	pub fn clique_engine(&self) -> Result<&ethcore::engines::Clique> {
		let generic_engine = self.engine_info.engine();
		return generic_engine.downcast_ref::<ethcore::engines::Clique>().ok_or_else(|| clique_not_running());
	}
}

fn snapshot_from(header: types::header::Header, state: CliqueBlockState) -> Result<Snapshot> {
	// FIXME(jleni): Use correct errors
	// let number = header.number.ok_or(unimplemented(None))? as u64;
	let number = 0;
	let hash = header.hash();
	let signers = state.signers.iter().map(|x| *x).collect();
	let recents = state.recent_signers.iter().map(|x| *x).collect();

	// FIXME(jleni):
	let votes = state.votes_history;
	let tally = BTreeMap::new();

	Ok(Snapshot { number, hash, signers, recents, votes, tally })
}

fn block_number_to_query(bn: BlockNumber) -> Result<BlockId> {
	match bn {
		BlockNumber::Num(v) => Ok(BlockId::Number(v)),
		BlockNumber::Latest => Ok(BlockId::Latest),
		BlockNumber::Earliest => Ok(BlockId::Earliest),
		BlockNumber::Pending => Err(unimplemented(None))
	}
}

fn hash_to_query(h: H256) -> Result<BlockId> {
	Ok(BlockId::Hash(h))
}

impl<EI: 'static> Clique for CliqueClient<EI> where EI: EngineInfo + Sync + Send {
	fn get_snapshot(&self, block_number: BlockNumber) -> Result<Snapshot> {
		let clique_engine = self.clique_engine()?;
		let query = block_number_to_query(block_number)?;

		// FIXME(jleni): Improve this
//		let state = clique_engine.get_state(query).or_else(unimplemented(None))?;
		let state = clique_engine.get_state(query)?;
		let header = clique_engine.find_header(query)?;

		Ok(snapshot_from(header, state)?)
	}

	fn get_snapshot_at_hash(&self, hash: H256) -> Result<Snapshot> {
		let clique_engine = self.clique_engine()?;
		let query = hash_to_query(hash)?;

		// FIXME(jleni): Improve this
		let state = clique_engine.get_state(query).or_else(unimplemented(None))?;
		let header = clique_engine.find_header(query)?;

		Ok(snapshot_from(header, state)?)
	}

	fn get_signers(&self, block_number: BlockNumber) -> Result<Vec<Address>> {
//		let query = BlockId::Number(block_number);
//		let query = BlockId::Hash(*hash);
		let clique_engine = self.clique_engine()?;
		let query = block_number_to_query(block_number)?;
		let answer = clique_engine.get_signers(query).or_else(unimplemented(None))?;
		Ok(answer)
	}

	fn get_signers_at_hash(&self, hash: H256) -> Result<Vec<Address>> {
		let clique_engine = self.clique_engine()?;
		let query = hash_to_query(hash)?;
		let answer = clique_engine.get_signers(query).or_else(unimplemented(None))?;
		Ok(answer)
	}

	fn proposals(&self) -> Result<BTreeMap<Address, bool>> {
		let clique_engine = self.clique_engine()?;

		unimplemented!()
	}

	fn propose(&self, address: Address, auth: bool) -> Result<()> {
		let clique_engine = self.clique_engine()?;

		unimplemented!()
	}

	fn discard(&self, address: Address) -> Result<()> {
		let clique_engine = self.clique_engine()?;

		unimplemented!()
	}
}
