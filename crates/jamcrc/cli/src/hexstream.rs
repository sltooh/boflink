use std::io::Read;

const DEFAULT_BUFFER_SIZE: usize = (8 * 1024) / 2;

pub struct HexDecodeStream<R: Read> {
    buffer: Vec<u8>,
    reader: R,
}

impl<R: Read> HexDecodeStream<R> {
    pub fn new(reader: R) -> HexDecodeStream<R> {
        Self {
            buffer: Vec::with_capacity(DEFAULT_BUFFER_SIZE),
            reader,
        }
    }
}

impl<R: Read> From<R> for HexDecodeStream<R> {
    fn from(value: R) -> Self {
        Self::new(value)
    }
}

impl<R: Read> std::io::Read for HexDecodeStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buffer.resize(buf.len() * 2, 0);
        let read_in = self.reader.read(&mut self.buffer)?;
        hex::decode_to_slice(&self.buffer[..read_in], &mut buf[..read_in / 2])
            .map_err(std::io::Error::other)?;
        Ok(read_in / 2)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::HexDecodeStream;

    #[test]
    fn decode_stream_round_trip() {
        let input = "hello world";

        let encoded = hex::encode(input);
        let mut stream = HexDecodeStream::new(encoded.as_bytes());

        let mut decoded = Vec::new();
        stream
            .read_to_end(&mut decoded)
            .expect("Could not read stream");

        assert_eq!(decoded, input.as_bytes());
    }
}
