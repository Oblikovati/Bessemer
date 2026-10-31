use super::load_projection::read_word;
use bessemer::simulation::SimulationPersistVersion;

pub(super) struct LoadEditCase<'a, K> {
    pub kind: K,
    pub version: SimulationPersistVersion,
    pub replacement: i32,
    pub changed: bool,
    pub original: &'a [u8],
    pub expected: &'a [u8],
}

pub(super) fn read_edit_case<'a, K: Copy>(
    cursor: &mut &'a [u8],
    kinds: &[K],
) -> LoadEditCase<'a, K> {
    let kind = kinds[read_word(cursor) as usize];
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let replacement = read_word(cursor) as i32;
    let changed = read_word(cursor) != 0;
    let size = read_word(cursor) as usize;
    let (original, remaining) = cursor.split_at(size);
    let (expected, remaining) = remaining.split_at(size);
    *cursor = remaining;
    LoadEditCase {
        kind,
        version,
        replacement,
        changed,
        original,
        expected,
    }
}
