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

mod client;

use snapshot::{self, io as snapshot_io, SnapshotClient};
use engines::clique::{ extract_signers };
use client::{
	BlockId,
    Client
};

pub trait Clique {
		fn get_snapshot(&self, BlockNumber) -> Result<()>;
		fn get_snapshot_at_hash(&self, H256) -> Result<()>;
		fn get_signers(&self, BlockNumber) -> Result<()>;
		fn get_signers_at_hash(&self, H256) -> Result<()>;
		fn proposals(&self, BTreeSet<H512>) -> Result<()>;
		fn propose(&self) -> Result<()>;
		fn discard(&self, H160, Password, H256) -> Result<()>;
}

impl Clique for Client {
    pub fn get_snapshot<W: snapshot_io::SnapshotWriter + Send>(&self, writer: W, at: BlockId, p: &snapshot::Progress) -> Result<(), EthcoreError> {
        let db = self.state_db.read().journal_db().boxed_clone();
		let best_block_number = self.chain_info().best_block_number;
		let block_number = self.block_number(at).ok_or(snapshot::Error::InvalidStartingBlock(at))?;

		let history = ::std::cmp::min(self.history, 1000);

		let start_hash = match at {
			BlockId::Latest => {
				let start_num = match db.earliest_era() {
					Some(era) => ::std::cmp::max(era, best_block_number.saturating_sub(history)),
					None => best_block_number.saturating_sub(history),
				};

				match self.block_hash(BlockId::Number(start_num)) {
					Some(h) => h,
					None => return Err(snapshot::Error::InvalidStartingBlock(at).into()),
				}
			}
			_ => match self.block_hash(at) {
				Some(hash) => hash,
				None => return Err(snapshot::Error::InvalidStartingBlock(at).into()),
			},
		};

		let processing_threads = self.config.snapshot.processing_threads;
		snapshot::take_snapshot(&*self.engine, &self.chain.read(), start_hash, db.as_hashdb(), writer, p, processing_threads)?;

		Ok(())
    }
    
    fn get_signers(&self, at: BlockId) -> Result<Vec<Address>, Error>> {
        let engine = self.engine();
        let hash = BlockId::hash(at)

        extract_signers(at)
    }

    fn proposals(&self, BTreeSet<H512>) -> Result<()> {
        OK(())
    }

    fn propose() -> Result<()> {
        Ok(())
    }

    fn discard() -> Result<()> {
        Ok(())
    }


}


	// /// Take a snapshot at the given block.
	// /// If the ID given is "latest", this will default to 1000 blocks behind.
	// pub fn take_snapshot<W: snapshot_io::SnapshotWriter + Send>(&self, writer: W, at: BlockId, p: &snapshot::Progress) -> Result<(), EthcoreError> {
	// 	let db = self.state_db.read().journal_db().boxed_clone();
	// 	let best_block_number = self.chain_info().best_block_number;
	// 	let block_number = self.block_number(at).ok_or(snapshot::Error::InvalidStartingBlock(at))?;

	// 	let history = ::std::cmp::min(self.history, 1000);

	// 	let start_hash = match at {
	// 		BlockId::Latest => {
	// 			let start_num = match db.earliest_era() {
	// 				Some(era) => ::std::cmp::max(era, best_block_number.saturating_sub(history)),
	// 				None => best_block_number.saturating_sub(history),
	// 			};

	// 			match self.block_hash(BlockId::Number(start_num)) {
	// 				Some(h) => h,
	// 				None => return Err(snapshot::Error::InvalidStartingBlock(at).into()),
	// 			}
	// 		}
	// 		_ => match self.block_hash(at) {
	// 			Some(hash) => hash,
	// 			None => return Err(snapshot::Error::InvalidStartingBlock(at).into()),
	// 		},
	// 	};

	// 	let processing_threads = self.config.snapshot.processing_threads;
	// 	snapshot::take_snapshot(&*self.engine, &self.chain.read(), start_hash, db.as_hashdb(), writer, p, processing_threads)?;

	// 	Ok(())
	// }
