use super::projections::span;
use std::io::Read;

pub(super) struct StoredParameterCase {
    pub input: Vec<u8>,
    pub expected: Vec<u8>,
}

pub(super) struct OwnedParameterFixture {
    reader: flate2::read::ZlibDecoder<&'static [u8]>,
    bases: Vec<Vec<u8>>,
    pub expanded_size: u32,
    pub count: u32,
}

impl OwnedParameterFixture {
    pub fn open() -> Self {
        let mut reader = flate2::read::ZlibDecoder::new(
            include_bytes!("../fixtures/simulation-parameter-sets/owned.bin").as_slice(),
        );
        assert_eq!(take(&mut reader, 4), b"PADX");
        assert_eq!(word(&mut reader), 2);
        let expanded_size = word(&mut reader);
        let base_count = word(&mut reader);
        let bases = (0..base_count).map(|_| read_span(&mut reader)).collect();
        let count = word(&mut reader);
        Self {
            reader,
            bases,
            expanded_size,
            count,
        }
    }

    pub fn next_case(&mut self) -> StoredParameterCase {
        let mut input = take(&mut self.reader, 28);
        span(&mut input, &read_span(&mut self.reader));
        span(&mut input, &read_span(&mut self.reader));
        let index = word(&mut self.reader) as usize;
        let delta = read_span(&mut self.reader);
        let base = &self.bases[index];
        assert_eq!(base.len(), delta.len());
        let expected = base
            .iter()
            .zip(delta)
            .map(|(first, second)| first ^ second)
            .collect();
        StoredParameterCase { input, expected }
    }

    pub fn finish(&mut self) {
        let mut extra = [0; 1];
        assert_eq!(self.reader.read(&mut extra).unwrap(), 0);
    }
}

fn word(reader: &mut impl Read) -> u32 {
    let mut bytes = [0; 4];
    reader.read_exact(&mut bytes).unwrap();
    u32::from_le_bytes(bytes)
}

fn read_span(reader: &mut impl Read) -> Vec<u8> {
    let size = word(reader) as usize;
    take(reader, size)
}

fn take(reader: &mut impl Read, size: usize) -> Vec<u8> {
    let mut bytes = vec![0; size];
    reader.read_exact(&mut bytes).unwrap();
    bytes
}
