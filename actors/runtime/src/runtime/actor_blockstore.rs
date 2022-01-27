use anyhow::Result;
use cid::multihash::Code;
use cid::Cid;
use fvm_sdk::blockstore::Blockstore;
use fvm_shared::blockstore::Block;
use fvm_shared::error::ExitCode;

use crate::{Abortable, IntoAnyhow};

/// A blockstore suitable for use within actors.
pub struct ActorBlockstore;

/// Implements a blockstore delegating to IPLD syscalls.
impl fvm_shared::blockstore::Blockstore for ActorBlockstore {
    fn get(&self, cid: &Cid) -> Result<Option<Vec<u8>>> {
        Blockstore
            .get(cid)
            .or_abort(ExitCode::ErrIllegalState, "bs::get")
            .anyhow()
    }

    fn put_keyed(&self, k: &Cid, block: &[u8]) -> Result<()> {
        Blockstore
            .put_keyed(k, block)
            .or_abort(ExitCode::ErrSerialization, "bs::put_keyed")
            .anyhow()
    }

    fn put<D>(&self, code: Code, block: &Block<D>) -> Result<Cid>
    where
        D: AsRef<[u8]>,
    {
        Blockstore
            .put(code, block)
            .or_abort(ExitCode::ErrIllegalState, "bs::put")
            .anyhow()
    }
}
