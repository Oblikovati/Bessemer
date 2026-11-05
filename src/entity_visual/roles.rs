use super::EntityVisualStoredWords;

/// Documented ByLayer bits; SP names retain their literal source bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityVisualByLayerRole {
    /// Stored color override bit.
    Color,
    /// Stored font override bit.
    Font,
    /// Stored width override bit.
    Width,
    /// Stored font-SP bit.
    FontSp,
    /// Stored width-SP bit.
    WidthSp,
}

impl EntityVisualByLayerRole {
    pub(crate) fn mask(self) -> u16 {
        match self {
            Self::Color => 1,
            Self::Font => 2,
            Self::Width => 4,
            Self::FontSp => 8,
            Self::WidthSp => 16,
        }
    }
}

/// Stored cache bits, before layer/default/native override resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityVisualCacheRole {
    /// Stored visible bit.
    Visible,
    /// Stored printable bit.
    Printable,
}

impl EntityVisualCacheRole {
    pub(crate) fn mask(self) -> u8 {
        match self {
            Self::Visible => 1,
            Self::Printable => 2,
        }
    }
}

impl EntityVisualStoredWords {
    /// Reads one stored ByLayer bit; e.g. `words.by_layer(EntityVisualByLayerRole::Color)`.
    pub fn by_layer(self, role: EntityVisualByLayerRole) -> bool {
        self.by_layer_flags & role.mask() != 0
    }

    /// Reads one stored cache bit; e.g. `words.cached(EntityVisualCacheRole::Visible)`.
    pub fn cached(self, role: EntityVisualCacheRole) -> bool {
        self.layer_attribute_cache & role.mask() != 0
    }

    /// Retains undocumented ByLayer bits; e.g. `let raw = words.by_layer_unresolved_bits();`.
    pub fn by_layer_unresolved_bits(self) -> u16 {
        self.by_layer_flags & 0xffe0
    }

    /// Retains undocumented cache bits; e.g. `let raw = words.cache_unresolved_bits();`.
    pub fn cache_unresolved_bits(self) -> u8 {
        self.layer_attribute_cache & 0xfc
    }

    /// Projects stored RGB byte order; e.g. `let [red, green, blue] = words.color_rgb();`.
    pub fn color_rgb(self) -> [u8; 3] {
        [
            self.color_word as u8,
            (self.color_word >> 8) as u8,
            (self.color_word >> 16) as u8,
        ]
    }

    /// Preserves the unresolved high byte; e.g. `let high = words.color_unresolved_high_byte();`.
    pub fn color_unresolved_high_byte(self) -> u8 {
        (self.color_word >> 24) as u8
    }

    /// Tests only the exact stored sentinel; e.g. `let missing = words.color_is_missing();`.
    pub fn color_is_missing(self) -> bool {
        self.color_word == u32::MAX
    }
}
