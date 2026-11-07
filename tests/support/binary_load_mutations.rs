use super::load_projection::read_word;

fn splice_mutant(cursor: &mut &[u8], original: &[u8]) -> Vec<u8> {
    let start = read_word(cursor) as usize;
    let removed = read_word(cursor) as usize;
    let size = read_word(cursor) as usize;
    let (replacement, remaining) = cursor.split_at(size);
    *cursor = remaining;
    let mut image = original[..start].to_vec();
    image.extend_from_slice(replacement);
    image.extend_from_slice(&original[start + removed..]);
    image
}

pub(super) fn check_mutations(
    cursor: &mut &[u8],
    original: &[u8],
    accepts: impl Fn(&[u8]) -> bool,
) -> (usize, usize) {
    let (mut accepted, mut rejected) = (0, 0);
    for _ in 0..read_word(cursor) {
        let expected = read_word(cursor) != 0;
        let image = splice_mutant(cursor, original);
        assert_eq!(
            accepts(&image),
            expected,
            "expected complete actual reference acceptance"
        );
        accepted += usize::from(expected);
        rejected += usize::from(!expected);
    }
    (accepted, rejected)
}
