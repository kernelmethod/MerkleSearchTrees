use crate::hash::HashFunction;

use core::marker::PhantomData;

pub struct MerkleSearchTree<H: HashFunction> {
    hashfn: PhantomData<H>,
}

impl<H: HashFunction> MerkleSearchTree<H> {
    /// Construct a new [`MerkleSearchTree`] instance.
    pub fn new() -> Self {
        MerkleSearchTree {
            hashfn: PhantomData,
        }
    }

    /// Hash an input with the [`MerkleSearchTree`]'s hash function.
    pub fn hash(input: &[u8]) -> H::Hash {
        H::hash(input)
    }

    /// Construct a new [create::hash::Hasher](`Hasher`) instance corresponding to
    /// the Merkle Search Tree's underlying hash function.
    pub fn hasher() -> H::Hasher {
        H::hasher()
    }
}
