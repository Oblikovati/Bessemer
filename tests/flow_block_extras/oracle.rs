use super::fixture::{blob, boolean, byte, number, signed, word};
use bessemer::flow::*;
use std::io::{Cursor, Read};

pub struct ExpectedExtraFrame {
    pub name: String,
    pub scopes: Vec<ExpectedExtraScope>,
    pub fields: Vec<ExpectedExtraField>,
}

pub struct ExpectedExtraScope {
    pub entity: FlowBlockExtraEntity,
    pub interval: (usize, usize),
    pub native_index: Option<i32>,
    pub stored: Option<bool>,
}

pub struct ExpectedExtraField {
    pub entity: FlowBlockExtraEntity,
    pub kind: FlowBlockExtraFieldKind,
    pub encoding: FlowBlockExtraEncoding,
    pub editable: bool,
    pub layout: FlowBlockExtraFieldLayout,
    pub packed: Vec<u8>,
}

pub fn frames() -> Vec<ExpectedExtraFrame> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/flow-block-extras/fields.bin.gz")[..],
    )
    .take(50_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FEO1");
    let result = (0..word(&mut cursor)).map(|_| frame(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn frame(cursor: &mut Cursor<&[u8]>) -> ExpectedExtraFrame {
    ExpectedExtraFrame {
        name: String::from_utf8(blob(cursor)).unwrap(),
        scopes: (0..word(cursor)).map(|_| scope(cursor)).collect(),
        fields: (0..word(cursor)).map(|_| field(cursor)).collect(),
    }
}

fn scope(cursor: &mut Cursor<&[u8]>) -> ExpectedExtraScope {
    let entity = entity(cursor);
    let interval = (word(cursor) as usize, word(cursor) as usize);
    let index_tag = byte(cursor);
    assert!(index_tag <= 2);
    ExpectedExtraScope {
        entity,
        interval,
        native_index: (index_tag != 0).then(|| signed(cursor)),
        stored: (index_tag != 0).then_some(index_tag == 2),
    }
}

fn entity(cursor: &mut Cursor<&[u8]>) -> FlowBlockExtraEntity {
    let tag = byte(cursor);
    let [block, row, index] = std::array::from_fn(|_| word(cursor) as usize);
    use FlowBlockExtraEntity::*;
    match tag {
        0 => Record,
        1 => Block(block),
        2 => Body(block),
        3 => First { block, index },
        4 => FirstMixed { block, index },
        5 => Second { block, index },
        6 => Third { block, index },
        7 => Fourth(block),
        8 => NestedRow { block, row },
        9 => NestedEntry { block, row, index },
        10 => Final(block),
        11 => Extra(block),
        12 => ExtraMixed(block),
        _ => panic!("entity tag {tag}; expected 0..12"),
    }
}

fn field(cursor: &mut Cursor<&[u8]>) -> ExpectedExtraField {
    let entity = entity(cursor);
    let kind = kind(byte(cursor));
    let encoding = encoding(byte(cursor));
    let editable = boolean(cursor);
    let layout = FlowBlockExtraFieldLayout {
        first_byte: word(cursor) as usize,
        element_count: word(cursor) as usize,
        element_bytes: word(cursor) as usize,
        stride_bytes: word(cursor) as usize,
    };
    ExpectedExtraField {
        entity,
        kind,
        encoding,
        editable,
        layout,
        packed: blob(cursor),
    }
}

fn encoding(tag: u8) -> FlowBlockExtraEncoding {
    match tag {
        0 => FlowBlockExtraEncoding::Byte,
        1 => FlowBlockExtraEncoding::Signed32,
        2 => FlowBlockExtraEncoding::WideBits,
        _ => panic!("encoding tag {tag}; expected 0..2"),
    }
}

fn kind(tag: u8) -> FlowBlockExtraFieldKind {
    *EXTRA_FIELD_KINDS.get(tag as usize).unwrap()
}

use FlowBlockExtraFieldKind::*;
const EXTRA_FIELD_KINDS: &[FlowBlockExtraFieldKind] = &[
    Version,
    Token,
    BodyFlag,
    FirstCount,
    SecondCount,
    ThirdCount,
    PairBytes,
    FirstWords,
    MixedCount,
    MixedWords,
    MixedWide,
    ReferenceWords,
    ByteArrayCount,
    ByteArray,
    AdditionalFlag,
    AdditionalWords,
    TagByte,
    ThirdWord,
    ThirdWide,
    FourthCount,
    FourthWide,
    FourthBytes,
    OuterFlag,
    NestedFlag,
    NestedCount,
    NestedElementCount,
    NestedIndex,
    NestedWords,
    NestedWide,
    FinalFlag,
    FinalWord,
    FinalCount,
    FinalWords,
    ExtraFlag,
    ExtraWord,
    LastByte,
];
