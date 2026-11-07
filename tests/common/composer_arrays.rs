use bessemer::composer::arrays::*;
use std::io::{Cursor, Read};

pub struct ArrayCase {
    pub dimensions: u8,
    pub count: u32,
    pub domain: u32,
    pub wire: Vec<u8>,
}

pub fn read_cases(bytes: &[u8]) -> Vec<ArrayCase> {
    let mut reader = Cursor::new(bytes);
    let mut magic = [0; 4];
    reader.read_exact(&mut magic).unwrap();
    assert_eq!(&magic, b"VARR");
    let cases = (0..word(&mut reader))
        .map(|_| read_case(&mut reader))
        .collect();
    assert_eq!(reader.position() as usize, bytes.len());
    cases
}

fn word(reader: &mut Cursor<&[u8]>) -> u32 {
    let mut bytes = [0; 4];
    reader.read_exact(&mut bytes).unwrap();
    u32::from_le_bytes(bytes)
}

fn read_case(reader: &mut Cursor<&[u8]>) -> ArrayCase {
    let mut dimensions = [0];
    reader.read_exact(&mut dimensions).unwrap();
    let (count, domain, size) = (word(reader), word(reader), word(reader));
    let mut wire = vec![0; size as usize];
    reader.read_exact(&mut wire).unwrap();
    ArrayCase {
        dimensions: dimensions[0],
        count,
        domain,
        wire,
    }
}

pub fn check_case(case: &ArrayCase) -> Vec<u8> {
    if case.dimensions == 0 {
        return check_indices(case);
    }
    let layout = vector_layout(case);
    let block = ComposerVectorBlock::from_bytes(&case.wire, layout, case.wire.len()).unwrap();
    assert_eq!(block.layout(), layout);
    assert_eq!(block.to_bytes(), case.wire);
    check_construction(&block, &case.wire);
    check_vector_edit(&block, &case.wire)
}

fn vector_layout(case: &ArrayCase) -> ComposerVectorLayout {
    ComposerVectorLayout {
        count: case.count,
        dimensions: match case.dimensions {
            2 => ComposerVectorDimensions::Two,
            3 => ComposerVectorDimensions::Three,
            value => panic!("dimension {value}; expected 2 or 3"),
        },
    }
}

fn check_construction(block: &ComposerVectorBlock, wire: &[u8]) {
    let constructed = ComposerVectorBlock::from_parts(
        block.layout(),
        block.packing(),
        block.axes(),
        block.packed_bytes(),
        wire.len(),
    )
    .unwrap();
    assert_eq!(constructed.to_bytes(), wire);
}

fn check_vector_edit(block: &ComposerVectorBlock, wire: &[u8]) -> Vec<u8> {
    let mut packed = block.packed_bytes().to_vec();
    if let Some(first) = packed.first_mut() {
        *first ^= 1;
    }
    let edit = block.edit_packed(&packed).unwrap();
    assert_eq!(edit.block.packed_bytes(), packed);
    assert_eq!(block.to_bytes(), wire);
    edit.block.to_bytes()
}

fn check_indices(case: &ArrayCase) -> Vec<u8> {
    let block =
        ComposerIndexBlock::from_bytes(&case.wire, case.count, case.domain, case.wire.len())
            .unwrap();
    assert_eq!((block.count(), block.domain()), (case.count, case.domain));
    assert_eq!(block.to_bytes(), case.wire);
    let mut values = block.values().collect::<Vec<_>>();
    let constructed =
        ComposerIndexBlock::from_values(&values, case.domain, case.wire.len()).unwrap();
    assert_eq!(constructed.to_bytes(), case.wire);
    if let Some(first) = values.first_mut() {
        *first ^= 1;
    }
    let edit = block.edit_values(&values).unwrap();
    assert_eq!(edit.block.values().collect::<Vec<_>>(), values);
    assert_eq!(block.to_bytes(), case.wire);
    edit.block.to_bytes()
}
