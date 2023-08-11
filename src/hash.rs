/// Trait for hash functions that can be used by [`MerkleSearchTree`].
pub trait HashFunction {
    type Hash;
    type Hasher: Hasher<Hash = Self::Hash>;

    fn hash(input: &[u8]) -> Self::Hash;
    fn hasher() -> Self::Hasher;
}

/// Trait for hashing objects with incrementally updatable state.
pub trait Hasher {
    type Hash;

    fn update(&mut self, input: &[u8]) -> &mut Self;
    fn finalize(&self) -> Self::Hash;
}

/// Default hash function implementation for this crate. This type wraps [`blake3`]
/// under the hood.
pub struct DefaultHash;

impl HashFunction for DefaultHash {
    type Hash = blake3::Hash;
    type Hasher = blake3::Hasher;

    fn hash(input: &[u8]) -> Self::Hash {
        blake3::hash(input)
    }

    fn hasher() -> Self::Hasher {
        blake3::Hasher::new()
    }
}

impl Hasher for blake3::Hasher {
    type Hash = blake3::Hash;

    fn update(&mut self, input: &[u8]) -> &mut Self {
        self.update(input)
    }

    fn finalize(&self) -> Self::Hash {
        self.finalize()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_defaulthash() {
        let hash1 = DefaultHash::hash(b"hello, world");
        let hash2 = DefaultHash::hasher()
            .update(b"hello, world")
            .finalize();

        assert_eq!(hash1, hash2);
    }
}
