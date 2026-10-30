use super::{reader::SceneReader, *};

#[derive(Clone, Copy)]
enum InfoScalar {
    Byte,
    Word,
    Wide,
}

impl InfoScalar {
    fn width(self) -> usize {
        match self {
            Self::Byte => 1,
            Self::Word => 4,
            Self::Wide => 8,
        }
    }

    fn read(self, reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
        match self {
            Self::Byte => {
                reader.byte(path)?;
            }
            Self::Word => {
                reader.word(path)?;
            }
            Self::Wide => reader.wide(path)?,
        }
        Ok(())
    }
}

pub(super) fn information(reader: &mut SceneReader<'_, '_>) -> Result<()> {
    reader.version()?;
    reader.text("information.source")?;
    reader.word("information.raw_word_20")?;
    let size_field = reader.ledger.fields.len();
    let size = reader.word("information.logical_size")? as usize;
    let start_field = reader.ledger.fields.len();
    information_body(reader, size)?;
    let end_field = reader.ledger.fields.len();
    reader.ledger.rules.push(SizeRule {
        size_field,
        start_field,
        end_field,
        omit_text_terminators: true,
    });
    Ok(())
}

fn information_body(reader: &mut SceneReader<'_, '_>, size: usize) -> Result<()> {
    let mut logical = 0;
    for name in ["3c", "34", "54", "44", "4c", "5c"] {
        info_scalar(reader, name, InfoScalar::Wide, size, &mut logical)?;
    }
    info_scalar(reader, "30", InfoScalar::Byte, size, &mut logical)?;
    info_scalar(reader, "32", InfoScalar::Byte, size, &mut logical)?;
    info_scalar(reader, "64", InfoScalar::Word, size, &mut logical)?;
    for name in ["68", "88"] {
        info_text(reader, name, size, &mut logical)?;
    }
    info_scalar(reader, "a8", InfoScalar::Word, size, &mut logical)?;
    info_scalar(reader, "ac", InfoScalar::Wide, size, &mut logical)?;
    info_scalar(reader, "31", InfoScalar::Byte, size, &mut logical)?;
    reader.extension("information.future_bytes", size - logical)
}

fn info_scalar(
    reader: &mut SceneReader<'_, '_>,
    name: &str,
    shape: InfoScalar,
    size: usize,
    logical: &mut usize,
) -> Result<()> {
    if *logical == size {
        return Ok(());
    }
    let width = shape.width();
    budget("information scalar width", width, size - *logical)?;
    shape.read(reader, &format!("information.raw_{name}"))?;
    *logical += width;
    Ok(())
}

fn info_text(
    reader: &mut SceneReader<'_, '_>,
    name: &str,
    size: usize,
    logical: &mut usize,
) -> Result<()> {
    if *logical == size {
        return Ok(());
    }
    let units = reader.text(&format!("information.string_{name}"))?;
    budget("information text content bytes", units * 2, size - *logical)?;
    *logical += units * 2;
    Ok(())
}

#[derive(Clone, Copy)]
enum UserCell {
    Text,
    Byte,
    Word,
    Patterns(usize),
}

pub(super) fn userdata(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<u32> {
    reader.sized(&format!("{path}.stored_size"), false, |body| {
        user_body(body, path)
    })
}

fn user_body(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<u32> {
    let mut lods = 1;
    for (name, shape) in USER_SEQUENCE {
        if reader.remaining() == 0 {
            continue;
        }
        let word = read_user_cell(reader, &format!("{path}.{name}"), shape)?;
        if name == "lod_count" {
            lods = word;
        }
    }
    reader.extension(&format!("{path}.future_bytes"), reader.remaining())?;
    Ok(lods)
}

const USER_SEQUENCE: [(&str, UserCell); 10] = [
    ("guid", UserCell::Text),
    ("raw_d0", UserCell::Word),
    ("assembly_name", UserCell::Text),
    ("raw_d4", UserCell::Word),
    ("raw_f8", UserCell::Word),
    ("center_of_gravity", UserCell::Patterns(3)),
    ("inertia_axes", UserCell::Patterns(9)),
    ("raw_12c", UserCell::Byte),
    ("issued_from", UserCell::Text),
    ("lod_count", UserCell::Word),
];

fn read_user_cell(reader: &mut SceneReader<'_, '_>, path: &str, shape: UserCell) -> Result<u32> {
    match shape {
        UserCell::Text => {
            reader.text(path)?;
        }
        UserCell::Byte => {
            reader.byte(path)?;
        }
        UserCell::Word => return reader.word(path),
        UserCell::Patterns(count) => reader.patterns(path, count)?,
    }
    Ok(0)
}

pub(super) fn attributes(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
    reader.text(&format!("{path}.first_name"))?;
    reader.text(&format!("{path}.second_name"))?;
    for name in ["diffuse", "specular", "ambient"] {
        reader.patterns(&format!("{path}.{name}"), 3)?;
    }
    reader.word(&format!("{path}.material_ec"))?;
    reader.word(&format!("{path}.alpha"))?;
    reader.sized(&format!("{path}.texture_body_size"), false, |body| {
        material_textures(body, path)
    })
}

fn material_textures(reader: &mut SceneReader<'_, '_>, path: &str) -> Result<()> {
    if reader.remaining() == 0 {
        return reader.extension(&format!("{path}.opaque_texture_bytes"), 0);
    }
    let count = reader.word(&format!("{path}.textures.count"))?;
    if count <= 4 {
        texture_columns(reader, path, count)?;
    }
    reader.extension(&format!("{path}.opaque_texture_bytes"), reader.remaining())
}

fn texture_columns(reader: &mut SceneReader<'_, '_>, path: &str, count: u32) -> Result<()> {
    let columns = [
        ("names", UserCell::Text),
        ("matrices", UserCell::Patterns(16)),
        ("mapping", UserCell::Byte),
        ("flags", UserCell::Byte),
        ("environment", UserCell::Word),
        ("wrap", UserCell::Byte),
        ("last_flags", UserCell::Byte),
    ];
    for (column, shape) in columns {
        for index in 0..count {
            if reader.remaining() == 0 {
                continue;
            }
            read_user_cell(reader, &format!("{path}.textures.{column}[{index}]"), shape)?;
        }
    }
    Ok(())
}
