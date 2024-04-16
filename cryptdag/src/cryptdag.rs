use skip_ratchet::Ratchet;

use crate::{Dag, DagPath, Result, SnapshotKey, TemporalKey};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// TODO: Document
pub struct Cryptdag<D>
where
    D: Dag,
{
    /// TODO: Document
    _primordial_ratchet: Ratchet,

    /// TODO: Document
    _dag: D,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<D> Cryptdag<D>
where
    D: Dag,
{
    /// TODO: Document
    pub fn new(_primordial_ratchet: Ratchet, _dag: D) -> Self {
        Self {
            _primordial_ratchet,
            _dag,
        }
    }

    /// TODO: Document
    pub fn get_temporal_key(&self, _path: DagPath<D::Identity>) -> Result<TemporalKey> {
        unimplemented!()
    }

    /// TODO: Document
    pub fn get_snapshot_key(&self, _path: DagPath<D::Identity>) -> Result<SnapshotKey> {
        unimplemented!()
    }
}
