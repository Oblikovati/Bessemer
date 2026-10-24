use super::codec::FacetCodec;
use super::context::{CollectionClass, FacetState};
use super::output::FacetOutput;
use super::{StoredFloat, invalid};
use crate::Result;
use crate::archive_codec::ArchiveCursor;

/// Nonzero element width in a dynamic array, distinct from the null-array marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayElementSize(u32);

impl ArrayElementSize {
    /// Validates an explicit positive element width.
    ///
    /// ```
    /// assert_eq!(bessemer::tessellation::ArrayElementSize::new(12)?.value(), 12);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn new(value: u32) -> Result<Self> {
        if value != 0 {
            return Ok(Self(value));
        }
        Err(invalid(
            "array element size 0; expected 1..=4294967295, with absence represented separately",
        ))
    }

    /// Returns the stored element width in bytes.
    ///
    /// ```
    /// assert_eq!(bessemer::tessellation::ArrayElementSize::new(4)?.value(), 4);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// Complete metadata and uninterpreted elements of a declared dynamic array.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicArrayValues {
    /// Element width; its declaration does not imply an element's semantic type.
    pub element_size: ArrayElementSize,
    /// Stored initial capacity, not necessarily the current allocation size.
    pub initial_capacity: u32,
    /// Stored growth step.
    pub growth_step: u32,
    /// Stored element count, validated against the complete payload length.
    pub count: u32,
    /// All element bytes, preserving unknown values and element kinds.
    pub payload: Vec<u8>,
}

/// Null-array marker or complete dynamic array, distinct from a present empty array.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynamicArray {
    /// Explicit u32 zero, without capacity/growth/count words.
    Absent,
    /// Positive width and all metadata/payload bytes.
    Present(DynamicArrayValues),
}

impl DynamicArray {
    /// Decodes i32 elements when the stored width is four; absence stays distinct from empty.
    ///
    /// ```
    /// assert!(bessemer::tessellation::DynamicArray::Absent.signed_integers()?.is_none());
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn signed_integers(&self) -> Result<Option<Vec<i32>>> {
        let Some(values) = self.typed_values(4)? else {
            return Ok(None);
        };
        Ok(Some(
            values
                .payload
                .as_chunks::<4>()
                .0
                .iter()
                .map(|span| i32::from_le_bytes(*span))
                .collect(),
        ))
    }

    /// Decodes three-float elements while preserving every stored floating-point bit.
    ///
    /// ```
    /// assert!(bessemer::tessellation::DynamicArray::Absent.float_triplets()?.is_none());
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn float_triplets(&self) -> Result<Option<Vec<[StoredFloat; 3]>>> {
        let Some(values) = self.typed_values(12)? else {
            return Ok(None);
        };
        Ok(Some(
            values
                .payload
                .as_chunks::<12>()
                .0
                .iter()
                .map(float_point)
                .collect(),
        ))
    }

    fn typed_values(&self, width: u32) -> Result<Option<&DynamicArrayValues>> {
        let Self::Present(values) = self else {
            return Ok(None);
        };
        validate_values(values)?;
        if values.element_size.value() != width {
            return Err(invalid(format!(
                "array element width {}; expected {width}",
                values.element_size.value()
            )));
        }
        Ok(Some(values))
    }

    pub(super) fn read_implied(
        cursor: &mut ArchiveCursor<'_>,
        context: &mut FacetState,
        width: ArrayElementSize,
    ) -> Result<Self> {
        read_values(cursor, context, width).map(Self::Present)
    }

    pub(super) fn write_implied(&self, output: &mut FacetOutput, width: u32) -> Result<()> {
        let values = self.typed_values(width)?.ok_or_else(|| {
            invalid(format!(
                "null compact array; expected present width-{width} array"
            ))
        })?;
        write_values(values, output)
    }
}

impl FacetCodec for DynamicArray {
    const MINIMUM: usize = 4;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let width = cursor.u32()?;
        if width == 0 {
            return Ok(Self::Absent);
        }
        Self::read_implied(cursor, context, ArrayElementSize::new(width)?)
    }
    fn write(&self, output: &mut FacetOutput, _context: &mut FacetState) -> Result<()> {
        let Self::Present(values) = self else {
            output.u32(0);
            return Ok(());
        };
        validate_values(values)?;
        output.u32(values.element_size.value());
        write_values(values, output)
    }
}

fn read_values(
    cursor: &mut ArchiveCursor<'_>,
    context: &mut FacetState,
    width: ArrayElementSize,
) -> Result<DynamicArrayValues> {
    let initial_capacity = cursor.u32()?;
    let growth_step = cursor.u32()?;
    let count = cursor.u32()?;
    context.require_count(count as usize, CollectionClass::Element)?;
    let size = payload_size(width, count)?;
    let payload = cursor.take(size)?.to_vec();
    Ok(DynamicArrayValues {
        element_size: width,
        initial_capacity,
        growth_step,
        count,
        payload,
    })
}

fn payload_size(width: ArrayElementSize, count: u32) -> Result<usize> {
    (count as usize)
        .checked_mul(width.value() as usize)
        .ok_or_else(|| {
            invalid(format!(
                "array count {count} times width {}; expected usize",
                width.value()
            ))
        })
}

fn validate_values(values: &DynamicArrayValues) -> Result<()> {
    let expected = payload_size(values.element_size, values.count)?;
    if values.payload.len() == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "array payload length {}; expected {expected} for count {} and width {}",
        values.payload.len(),
        values.count,
        values.element_size.value()
    )))
}

fn write_values(values: &DynamicArrayValues, output: &mut FacetOutput) -> Result<()> {
    validate_values(values)?;
    output.u32(values.initial_capacity);
    output.u32(values.growth_step);
    output.u32(values.count);
    output.raw(&values.payload);
    Ok(())
}

fn float_point(span: &[u8; 12]) -> [StoredFloat; 3] {
    std::array::from_fn(|index| {
        let offset = index * 4;
        StoredFloat::from_bits(u32::from_le_bytes([
            span[offset],
            span[offset + 1],
            span[offset + 2],
            span[offset + 3],
        ]))
    })
}
