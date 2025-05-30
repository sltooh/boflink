/// JamCRC hasher.
pub struct Hasher {
    inner: crc32fast::Hasher,
}

impl Hasher {
    /// Creates a new [`Hasher`].
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: crc32fast::Hasher::new(),
        }
    }

    /// Creates a new [`Hasher`] with an initial state.
    #[inline]
    pub fn new_with_initial(init: u32) -> Self {
        Self {
            inner: crc32fast::Hasher::new_with_initial(init),
        }
    }

    /// Updates the hasher with the specified data.
    #[inline]
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Returns the JamCRC value.
    #[inline]
    pub fn finalize(self) -> u32 {
        !self.inner.finalize()
    }
}

impl std::default::Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}
