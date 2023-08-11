use crate::hash::HashFunction;

use core::marker::PhantomData;

pub struct MerkleSearchTree<H: HashFunction> {
    hashfn: PhantomData<H>,
}

impl<H: HashFunction> MerkleSearchTree<H> {
    pub fn new() -> Self {
        MerkleSearchTree {
            hashfn: PhantomData,
        }
    }
}
