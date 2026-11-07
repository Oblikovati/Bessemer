use super::{EntityVisualStoredEdit as Edit, EntityVisualStoredWords as Words, visual_role_error};
use crate::{Result, tessellation::StoredFloat};

impl Words {
    pub(crate) fn edited(mut self, edit: Edit, path: &str) -> Result<Self> {
        match edit {
            Edit::ByLayerRole(role, enabled) => {
                self.by_layer_flags =
                    (self.by_layer_flags & !role.mask()) | (u16::from(enabled) * role.mask());
            }
            Edit::CacheRole(role, enabled) => {
                self.layer_attribute_cache =
                    (self.layer_attribute_cache & !role.mask()) | (u8::from(enabled) * role.mask());
            }
            Edit::ColorRgb(rgb) => self.color_word = stored_rgb_word(self.color_word, rgb),
            Edit::LineWidth { standard, custom } => self.replace_width(standard, custom, path)?,
            Edit::CustomWidth(custom) => {
                self.replace_width(self.standard_width, Some(custom), path)?
            }
            other => self.replace_raw_word(other, path)?,
        }
        Ok(self)
    }

    fn replace_raw_word(&mut self, edit: Edit, path: &str) -> Result<()> {
        match edit {
            Edit::LayerNumber(value) => self.layer_number = value,
            Edit::ByLayerFlags(value) => self.by_layer_flags = value,
            Edit::LayerAttributeCache(value) => self.layer_attribute_cache = value,
            Edit::ColorWord(value) => self.color_word = value,
            Edit::LineStyle(value) => self.line_style = value,
            Edit::StandardWidth(value) => self.standard_width = value,
            other => return Err(visual_role_error(path, other)),
        }
        Ok(())
    }

    fn replace_width(
        &mut self,
        standard: i16,
        custom: Option<StoredFloat>,
        path: &str,
    ) -> Result<()> {
        if custom.is_some() != self.custom_width.is_some() {
            return Err(crate::BessemerError::new(
                "stored entity visual width",
                path,
                crate::FailureClass::Unsupported,
                format!(
                    "custom width {custom:?}; expected physical float presence {}",
                    self.custom_width.is_some()
                ),
            ));
        }
        self.standard_width = standard;
        self.custom_width = custom;
        Ok(())
    }
}

fn stored_rgb_word(original: u32, [red, green, blue]: [u8; 3]) -> u32 {
    (original & 0xff000000) | u32::from(red) | (u32::from(green) << 8) | (u32::from(blue) << 16)
}
