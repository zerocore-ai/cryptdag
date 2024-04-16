use std::collections::BTreeMap;

use crate::SnapshotKey;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// TODO: Document
pub struct Content<I> {
    /// TODO: Document
    _encrypted_ratchet: Vec<u8>, // -> Ratchet

    /// TODO: Document
    _children_keys: BTreeMap<I, SnapshotKey>,

    /// TODO: Document
    _serialized_data: Vec<u8>,
}

/// TODO: Document
pub struct ParentContentChallenge {
    /// TODO: Document
    _temporal_key_challenge: Vec<Vec<u8>>,

    /// TODO: Document
    _snapshot_key_challenge: Vec<Vec<u8>>,

    /// TODO: Document
    _encrypted_content: Vec<u8>, // -> Content
}
