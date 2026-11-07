use bessemer::simulation::*;
use std::io::Read;

pub struct Cursor<'a>(pub &'a [u8]);
impl<'a> Cursor<'a> {
    pub fn take(&mut self, count: usize) -> &'a [u8] {
        let (head, tail) = self.0.split_at(count);
        self.0 = tail;
        head
    }
    pub fn word(&mut self) -> u32 {
        u32::from_le_bytes(self.take(4).try_into().unwrap())
    }
    pub fn span(&mut self) -> &'a [u8] {
        let size = self.word() as usize;
        self.take(size)
    }
}

pub fn owned() -> Vec<u8> {
    let mut output = Vec::new();
    flate2::read::ZlibDecoder::new(
        include_bytes!("../fixtures/simulation-images/owned.bin").as_slice(),
    )
    .read_to_end(&mut output)
    .unwrap();
    output
}

pub fn master_oracle(master: &SimulationMasterArchive, expected: &[u8]) {
    let mut cursor = Cursor(expected);
    assert_eq!(master.version() as u32, cursor.word());
    for value in [384, 70, 81, 0, 70, 0, 0, 0, 70] {
        assert_eq!(value, cursor.word());
    }
    for tag in master.parts().field_tags {
        assert_eq!(tag as u32, cursor.word());
        assert_eq!(4, cursor.word());
    }
    for physical in master.parts().directory {
        assert_eq!(u32::from(physical), cursor.word());
    }
    for padding in [
        &master.parts().header_padding,
        &master.parts().field_table_padding,
        &master.parts().directory_padding,
    ] {
        assert_eq!(padding.as_ref(), cursor.span());
    }
    master_rows(master, &mut cursor);
    assert!(cursor.0.is_empty());
}

fn master_rows(master: &SimulationMasterArchive, cursor: &mut Cursor<'_>) {
    assert_eq!(70, cursor.word());
    for logical in 1..=70 {
        let physical = cursor.word() as usize;
        assert_eq!(physical, usize::from(master.parts().directory[logical - 1]));
        assert_eq!(1536 + (physical - 1) * 384, cursor.word() as usize);
        for field in 1..=81 {
            assert_eq!(
                master
                    .cell(&master.cell_id(logical, field).unwrap())
                    .unwrap(),
                cursor.word()
            );
        }
        assert_eq!(
            &master.parts().physical_rows[physical - 1].padding,
            cursor.span()
        );
    }
}

pub fn gen_oracle(image: &SimulationGenArchive, expected: &[u8]) {
    let mut cursor = Cursor(expected);
    let bytes = image.to_bytes();
    assert_eq!(
        u32::from_le_bytes(bytes[image.layout().blocks[9].0..][..4].try_into().unwrap()),
        cursor.word()
    );
    for count in image.layout().counts {
        assert_eq!(count, cursor.word());
    }
    assert_eq!(
        u32::from_le_bytes(bytes[600..604].try_into().unwrap()),
        cursor.word()
    );
    assert_eq!(image.layout().auxiliary_stride, cursor.word() as usize);
    assert_eq!(
        u32::from(image.layout().auxiliary_stride == 284),
        cursor.word()
    );
    assert_eq!(&bytes[4..404], cursor.span());
    assert_eq!(&bytes[460..600], cursor.span());
    for index in 0..10 {
        assert_eq!(image.layout().blocks[index].0, cursor.word() as usize);
        assert_eq!(image.block(index).unwrap(), cursor.span());
    }
    gen_vectors(image, &mut cursor);
    gen_mesh(image, &mut cursor);
    assert!(cursor.0.is_empty());
}

fn gen_vectors(image: &SimulationGenArchive, cursor: &mut Cursor<'_>) {
    assert_eq!(image.layout().vectors.len(), cursor.word() as usize);
    for (index, row) in image
        .block(7)
        .unwrap()
        .chunks_exact(image.layout().auxiliary_stride)
        .enumerate()
    {
        for offset in [0, 4] {
            assert_eq!(
                u32::from_le_bytes(row[offset..offset + 4].try_into().unwrap()),
                cursor.word()
            );
        }
        assert_eq!(row, cursor.span());
        let present = cursor.word() != 0;
        let actual = image.vector(index).unwrap();
        assert_eq!(actual.is_some(), present);
        if present {
            assert_eq!(
                image.layout().vectors[index].unwrap().0,
                cursor.word() as usize
            );
            assert_eq!(actual.unwrap(), cursor.span());
        }
    }
}

fn gen_mesh(image: &SimulationGenArchive, cursor: &mut Cursor<'_>) {
    assert_eq!(image.layout().counts[3], cursor.word());
    for index in 0..image.layout().counts[3] as usize {
        let node = image.node(&image.node_id(index).unwrap()).unwrap();
        for value in [
            u32::from(node.activity),
            node.group_tag as i32 as u32,
            node.group_tag as u16 as u32,
        ] {
            assert_eq!(value, cursor.word());
        }
        for value in node.coordinate_patterns {
            assert_eq!(value, cursor.word());
        }
    }
    assert_eq!(image.layout().counts[4], cursor.word());
    for index in 0..image.layout().counts[4] as usize {
        element_oracle(image, index, cursor);
    }
}

fn element_oracle(image: &SimulationGenArchive, index: usize, cursor: &mut Cursor<'_>) {
    let element = image.element(index).unwrap();
    assert_eq!(u32::from(element.activity), cursor.word());
    assert_eq!(u32::from(element.node_count), cursor.word());
    for value in element.elis_slots.into_iter().chain(element.node_slots) {
        assert_eq!(value as u32, cursor.word());
    }
    for ids in [
        element.node_ids().to_vec(),
        element.invalid_node_ids(image.layout().counts[3]),
    ] {
        assert_eq!(ids.len(), cursor.word() as usize);
        for id in ids {
            assert_eq!(id as u32, cursor.word());
        }
    }
}
