use super::super::load_reader::BinaryLoadReader;
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<(usize, usize)> {
    if reader.version > 800016 {
        reader.words("words3900_3904", 2)?;
    }
    if reader.version > 800028 {
        reader.signed("word6c")?;
    }
    let (mut delegates, mut pairs) = (0, 0);
    if reader.version >= 800039 {
        reader.words("words70_74", 2)?;
        let result = group(reader, "curve_group0")?;
        delegates += result.0;
        pairs += result.1;
    }
    if reader.version >= 800049 {
        reader.words("words3958_3988_398c_3990", 4)?;
        let result = group(reader, "curve_group1")?;
        delegates += result.0;
        pairs += result.1;
    }
    Ok((delegates, pairs))
}

fn group(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<(usize, usize)> {
    let count = reader.declared_count(&format!("{label}.count"), 10000)?;
    let mut pairs = 0;
    for index in 0..count {
        pairs += curve(reader, &format!("{label}.item{index}"))?;
    }
    Ok((count, pairs))
}

fn curve(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<usize> {
    reader.text(&format!("{label}.name"))?;
    reader.words(&format!("{label}.seven_words"), 7)?;
    if reader.version > 800032 {
        reader.signed(&format!("{label}.word100"))?;
    }
    reader.doubles(&format!("{label}.nine_doubles"), 9)?;
    reader.signed(&format!("{label}.word90"))?;
    reader.double(&format!("{label}.double88"))?;
    reader.signed(&format!("{label}.word94"))?;
    reader.doubles(&format!("{label}.doublesb0_b8"), 2)?;
    let count = reader.declared_count(&format!("{label}.pair_count"), 10000)?;
    if reader.version > 820010 {
        reader.signed(&format!("{label}.wordf8"))?;
    }
    reader.doubles(&format!("{label}.pairs"), count * 2)?;
    if reader.version > 830015 {
        reader.text(&format!("{label}.text10"))?;
    }
    Ok(count)
}
