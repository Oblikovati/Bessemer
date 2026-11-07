use std::io::Read;
pub fn inflate_exact(stored: &[u8]) -> Vec<u8> {
    let mut decoder = flate2::read::ZlibDecoder::new(stored);
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap();
    assert_eq!(decoder.total_in() as usize, stored.len());
    decoded
}
