use core::convert::TryInto;

/// A trait for which can be used to store serialized output.
pub trait SerBacker {
    /// Return a new backer which len() == 4.
    fn new() -> Self;

    fn len(&self) -> usize;

    fn get_first_4byte_slice(&mut self) -> &mut [u8; 4];

    fn extend_from_slice(&mut self, other: &[u8]);
    fn push(&mut self, byte: u8);
    fn truncate(&mut self, len: usize);
}

impl SerBacker for Vec<u8> {
    fn new() -> Self {
        vec![0, 0, 0, 0]
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get_first_4byte_slice(&mut self) -> &mut [u8; 4] {
        (&mut self[..4]).try_into().unwrap()
    }

    fn extend_from_slice(&mut self, other: &[u8]) {
        self.extend_from_slice(other)
    }

    fn push(&mut self, byte: u8) {
        self.push(byte)
    }

    fn truncate(&mut self, len: usize) {
        self.truncate(len)
    }
}
