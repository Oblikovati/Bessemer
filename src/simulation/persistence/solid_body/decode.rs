use super::super::load_reader::BinaryLoadReader;
use crate::Result;

const EARLY_FIELDS: &[(i32, &str, usize)] = &[
    (800066, "word80", 1),
    (800068, "word84", 1),
    (810025, "word8c", 1),
    (830007, "words90_94", 2),
    (830011, "wordsa8_b0", 3),
    (830013, "wordb4", 1),
    (830025, "wordb8", 1),
];

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    super::super::load_persist_id::read(reader)?;
    reader.text("text58")?;
    reader.text("text60")?;
    reader.words("words70_7c", 4)?;
    reader.text("text68")?;
    early_words(reader)?;
    late_words(reader)?;
    list_and_suffix(reader)
}

fn gated_words(reader: &mut BinaryLoadReader<'_>, gates: &[(i32, &str, usize)]) -> Result<()> {
    for &(version, label, width) in gates {
        if reader.version < version {
            continue;
        }
        if width == 1 {
            reader.signed(label)?;
        } else {
            reader.words(label, width)?;
        }
    }
    Ok(())
}

fn early_words(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    gated_words(reader, EARLY_FIELDS)?;
    if reader.version >= 830029 {
        reader.words("wordsbc_c8", 4)?;
        reader.doubles("doublesd0_f0", 5)?;
    }
    // The earlier helper gate follows the later mixed block physically (C-731, #150).
    if reader.version >= 810029 {
        reader.words("helper20.two_words", 2)?;
    }
    Ok(())
}

fn late_words(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    gated_words(
        reader,
        &[
            (840006, "words160_168", 3),
            (840013, "word1c8", 1),
            (840025, "word10", 1),
            (850002, "word1e4", 1),
            (850013, "word1cc", 1),
            (850026, "word1d0", 1),
            (870005, "word1d4", 1),
        ],
    )
}

fn list_and_suffix(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 870008 {
        let count = reader.count("list128.count", 100000, 4)?;
        for index in 0..count {
            reader.text(&format!("list128.{index}"))?;
        }
    }
    if reader.version >= 870019 {
        reader.signed("word1e0")?;
    }
    if reader.version >= 880004 {
        reader.words("words1f0_1f4", 2)?;
        for label in ["text1f8", "text200", "text208"] {
            reader.text(label)?;
        }
    }
    Ok(())
}
