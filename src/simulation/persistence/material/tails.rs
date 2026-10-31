use super::super::load_reader::BinaryLoadReader;
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 810004 {
        reader.text("texta0")?;
    }
    for (gate, label) in [(820002, "word3994"), (830001, "worda8")] {
        if reader.version >= gate {
            reader.signed(label)?;
        }
    }
    if (830003..830028).contains(&reader.version) {
        reader.signed("temporary_legacy_word")?;
    }
    if reader.version >= 830014 {
        reader.signed("wordb8")?;
    }
    late_names(reader)?;
    final_scalars(reader)
}

fn final_scalars(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 890003 {
        reader.double("double78")?;
        reader.words("words80_84_88", 3)?;
    }
    if reader.version >= 890013 {
        reader.words("words8c_90", 2)?;
    }
    if reader.version >= 890022 {
        reader.signed("word94")?;
    }
    if reader.version >= 890015 {
        reader.double("double98")?;
    }
    Ok(())
}

fn late_names(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 830016 {
        for label in ["textb0", "textc0", "textc8"] {
            reader.text(label)?;
        }
    }
    if reader.version >= 850023 {
        reader.text("textd8")?;
    }
    if reader.version >= 880011 {
        reader.signed("worde0")?;
    }
    Ok(())
}
