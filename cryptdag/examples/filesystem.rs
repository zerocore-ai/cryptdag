use cryptdag::{Cryptdag, Dag};
use skip_ratchet::Ratchet;

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() {
    let primordial_ratchet = Ratchet::from_rng(&mut rand::thread_rng());
    let cryptdag = Cryptdag::new(primordial_ratchet, FileSystemDag {});
    let _cryptdag_fs = CryptdagFileSystem::from(cryptdag);
}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

struct CryptdagFileSystem {
    _fs: FileSystem,
    _cryptdag: Cryptdag<FileSystemDag>,
}

struct FileSystemDag {}

struct FileSystem {}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl CryptdagFileSystem {
    fn from(_cryptdag: Cryptdag<FileSystemDag>) -> Self {
        Self {
            _fs: FileSystem::new(),
            _cryptdag,
        }
    }
}

impl FileSystem {
    fn new() -> Self {
        Self {}
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Dag for FileSystemDag {
    type Identity = String;
}
