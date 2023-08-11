use crate::hash::{DefaultHash, HashFunction};
use crate::tree::MerkleSearchTree;

use core::marker::PhantomData;

/// Builder type for [`MerkleSearchTree`].
pub struct MSTBuilder<H: HashFunction> {
    hashfn: PhantomData<H>,
}

impl<H: HashFunction> MSTBuilder<H> {
    /// Contsruct a new [`MSTBuilder`] instance.
    fn new() -> Self {
        MSTBuilder {
            hashfn: PhantomData,
        }
    }

    /// Finalize the builder and construct a new [`MerkleSearchTree`] from it.
    fn build(&self) -> MerkleSearchTree<H> {
        MerkleSearchTree::<H>::new()
    }
}

pub fn builder() -> MSTBuilder::<DefaultHash> {
    MSTBuilder::<DefaultHash>::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_construct_tree_with_builder() {
        let _tree = builder().build();
    }
}
