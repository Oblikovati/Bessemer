use super::{EntityVisualStoredWords as Words, VisualScalar as Scalar, visual_role_error};
use crate::Result;

impl Words {
    pub(crate) fn from_scalars(values: &[Scalar], path: &str) -> Result<Self> {
        let (layer, flags, cache, color, style, width) = visual_integer_values(values, path)?;
        Ok(Self {
            layer_number: layer,
            by_layer_flags: flags,
            layer_attribute_cache: cache,
            color_word: color,
            line_style: style,
            standard_width: width,
            custom_width: visual_custom_value(values, path)?,
        })
    }

    pub(crate) fn scalars(self) -> Vec<Scalar> {
        let mut values = vec![
            Scalar::U16(self.layer_number),
            Scalar::U16(self.by_layer_flags),
            Scalar::U8(self.layer_attribute_cache),
            Scalar::U32(self.color_word),
            Scalar::U16(self.line_style),
            Scalar::I16(self.standard_width),
        ];
        if let Some(custom) = self.custom_width {
            values.push(Scalar::F32(custom));
        }
        values
    }
}

fn visual_integer_values(values: &[Scalar], path: &str) -> Result<(u16, u16, u8, u32, u16, i16)> {
    match values.get(..6) {
        Some(
            [
                Scalar::U16(layer),
                Scalar::U16(flags),
                Scalar::U8(cache),
                Scalar::U32(color),
                Scalar::U16(style),
                Scalar::I16(width),
            ],
        ) => Ok((*layer, *flags, *cache, *color, *style, *width)),
        _ => Err(visual_role_error(path, values)),
    }
}

fn visual_custom_value(
    values: &[Scalar],
    path: &str,
) -> Result<Option<crate::tessellation::StoredFloat>> {
    match values.get(6..) {
        Some([]) => Ok(None),
        Some([Scalar::F32(value)]) => Ok(Some(*value)),
        _ => Err(visual_role_error(path, values)),
    }
}
