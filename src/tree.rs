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
    pub fn hash(&self, input: &[u8]) -> H::Hash {
        H::hash(input)
    }

    /// Construct a new [create::hash::Hasher](`Hasher`) instance corresponding to
    /// the Merkle Search Tree's underlying hash function.
    pub fn hasher(&self) -> H::Hasher {
        H::hasher()
    }
}

#[cfg(test)]
mod test {
    use crate::builder::*;

    #[test]
    pub fn test_hash() {
        let tree = builder().build();
        let hash1 = tree.hash(b"hello, world");
        let hash2 = tree.hasher().update(b"hello, world").finalize();

        assert_eq!(hash1, hash2);
    }
}
