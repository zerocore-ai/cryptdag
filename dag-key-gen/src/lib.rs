//! **`dag-key-gen`** is a structure-aware key generation tool for systems that are able to be represented as directed acyclic graphs (DAGs).
#![warn(missing_docs)]

mod dag;
mod key_gen;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use dag::*;
pub use key_gen::*;
