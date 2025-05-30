use std::io::Write;

use sha2::Digest;

pub struct Sha256Writer<W: Write> {
    hasher: sha2::Sha256,
    writer: W,
}

impl<W: Write> Sha256Writer<W> {
    pub fn new(writer: W) -> Sha256Writer<W> {
        Self {
            hasher: sha2::Sha256::new(),
            writer,
        }
    }

    pub fn finalize(self) -> [u8; 32] {
        self.hasher.finalize().into()
    }
}

impl<W: Write> std::io::Write for Sha256Writer<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hasher.update(buf);
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
