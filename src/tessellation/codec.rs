use super::context::{CollectionClass, FacetState};
use super::output::FacetOutput;
use super::{StoredDouble, StoredFloat, invalid};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ArchiveString, HeaderBox};

pub(super) trait FacetCodec: Sized {
    const MINIMUM: usize;
    const CLASS: CollectionClass = CollectionClass::Element;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self>;
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()>;
}

pub(super) trait FacetScalar: FacetCodec + Copy {
    const ZERO: Self;
}

macro_rules! scalar_codec {
    ($type:ty, $minimum:expr, $read:expr, $write:expr, $zero:expr) => {
        impl FacetCodec for $type {
            const MINIMUM: usize = $minimum;
            fn read(cursor: &mut ArchiveCursor<'_>, _context: &mut FacetState) -> Result<Self> {
                ($read)(cursor)
            }
            fn write(&self, output: &mut FacetOutput, _context: &mut FacetState) -> Result<()> {
                ($write)(output, *self);
                Ok(())
            }
        }
        impl FacetScalar for $type {
            const ZERO: Self = $zero;
        }
    };
}

scalar_codec!(
    i32,
    4,
    |cursor: &mut ArchiveCursor<'_>| cursor.i32(),
    |output: &mut FacetOutput, value: i32| output.i32(value),
    0
);
scalar_codec!(
    u32,
    4,
    |cursor: &mut ArchiveCursor<'_>| cursor.u32(),
    |output: &mut FacetOutput, value: u32| output.u32(value),
    0
);
scalar_codec!(
    u8,
    1,
    |cursor: &mut ArchiveCursor<'_>| cursor.u8(),
    |output: &mut FacetOutput, value: u8| output.u8(value),
    0
);
scalar_codec!(
    StoredFloat,
    4,
    |cursor: &mut ArchiveCursor<'_>| cursor.u32().map(StoredFloat::from_bits),
    |output: &mut FacetOutput, value: StoredFloat| output.u32(value.bits()),
    StoredFloat::from_bits(0)
);
scalar_codec!(
    StoredDouble,
    8,
    |cursor: &mut ArchiveCursor<'_>| cursor.u64().map(StoredDouble::from_bits),
    |output: &mut FacetOutput, value: StoredDouble| output.u64(value.bits()),
    StoredDouble::from_bits(0)
);

impl FacetCodec for ArchiveString {
    const MINIMUM: usize = 1;
    fn read(cursor: &mut ArchiveCursor<'_>, _context: &mut FacetState) -> Result<Self> {
        Self::decode(cursor)
    }
    fn write(&self, output: &mut FacetOutput, _context: &mut FacetState) -> Result<()> {
        output.text(self)
    }
}

impl FacetCodec for HeaderBox {
    const MINIMUM: usize = 80;
    fn read(cursor: &mut ArchiveCursor<'_>, _context: &mut FacetState) -> Result<Self> {
        crate::archive_codec::read_box(cursor)
    }
    fn write(&self, output: &mut FacetOutput, _context: &mut FacetState) -> Result<()> {
        output.bounds(self);
        Ok(())
    }
}

impl<T: FacetScalar, const N: usize> FacetCodec for [T; N] {
    const MINIMUM: usize = T::MINIMUM * N;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let mut values = [T::ZERO; N];
        for value in &mut values {
            *value = T::read(cursor, context)?;
        }
        Ok(values)
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        for value in self {
            value.write(output, context)?;
        }
        Ok(())
    }
}

impl<T: FacetScalar, const N: usize> FacetScalar for [T; N] {
    const ZERO: Self = [T::ZERO; N];
}

impl<T: FacetCodec> FacetCodec for Vec<T> {
    const MINIMUM: usize = 4;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let count = cursor.nonnegative_count(T::MINIMUM.max(1), "direct array")?;
        context.require_count(count, T::CLASS)?;
        let mut values = Vec::new();
        for _ in 0..count {
            values.push(T::read(cursor, context)?);
        }
        Ok(values)
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        output.count(self.len())?;
        for value in self {
            value.write(output, context)?;
        }
        Ok(())
    }
}

pub(super) fn read_optional<T: FacetCodec>(
    cursor: &mut ArchiveCursor<'_>,
    context: &mut FacetState,
    enabled: bool,
) -> Result<Option<T>> {
    if !enabled {
        return Ok(None);
    }
    T::read(cursor, context).map(Some)
}

pub(super) fn write_optional<T: FacetCodec>(
    value: &Option<T>,
    output: &mut FacetOutput,
    context: &mut FacetState,
    enabled: bool,
    name: &str,
) -> Result<()> {
    if enabled != value.is_some() {
        return Err(invalid(format!(
            "field {name} presence {}; expected {enabled} at version {:#x}",
            value.is_some(),
            context.version
        )));
    }
    if let Some(value) = value {
        value.write(output, context)?;
    }
    Ok(())
}
