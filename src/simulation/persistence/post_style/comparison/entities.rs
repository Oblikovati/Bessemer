/// Typed raw-word dependencies in the native style comparator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStyleComparisonWord {
    /// Mapping control.
    Word1B8,
    /// Model-tessellation control.
    Word21C,
    /// Conditional color220 control.
    Word230,
    /// Always compared raw word28C.
    Word28C,
    /// Always compared raw word290.
    Word290,
    /// Always compared raw word294.
    Word294,
    /// Always compared raw word298.
    Word298,
    /// Hidden-parts control.
    Word1EC,
    /// Loaded runtime word1F0; no wire migration is inferred.
    Word1F0,
    /// Conditional color20C control.
    Word204,
    /// Conditional word208.
    Word208,
}

impl SimulationStyleComparisonWord {
    /// Enumerate every raw-word dependency in stable input order; e.g. `Self::ALL`.
    pub const ALL: [Self; 11] = [
        Self::Word1B8,
        Self::Word21C,
        Self::Word230,
        Self::Word28C,
        Self::Word290,
        Self::Word294,
        Self::Word298,
        Self::Word1EC,
        Self::Word1F0,
        Self::Word204,
        Self::Word208,
    ];

    /// Inspect the canonical field identity; e.g. `Self::Word1F0.field_path()`.
    pub fn field_path(self) -> &'static str {
        match self {
            Self::Word1B8 => "style.word_1b8",
            Self::Word21C => "style.word_21c",
            Self::Word230 => "style.word_230",
            Self::Word28C => "style.word_28c",
            Self::Word290 => "style.word_290",
            Self::Word294 => "style.word_294",
            Self::Word298 => "style.word_298",
            Self::Word1EC => "style.word_1ec",
            Self::Word1F0 => "style.word_1f0",
            Self::Word204 => "style.word_204",
            Self::Word208 => "style.word_208",
        }
    }
}

/// Color storage used by the comparator, including color2AC's final scalar alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStyleComparisonColor {
    /// First mapping color.
    Color1BC,
    /// Second mapping color.
    Color1CC,
    /// Model-tessellation color; its alpha is also scalar22C.
    Color220,
    /// Always compared color29C.
    Color29C,
    /// Conditional hidden-parts color.
    Color1F4,
    /// Conditional color20C.
    Color20C,
    /// Only its alpha is consumed by this comparator, as scalar2B8.
    Color2AC,
}

impl SimulationStyleComparisonColor {
    /// Enumerate all retained color storage in stable input order; e.g. `Self::ALL`.
    pub const ALL: [Self; 7] = [
        Self::Color1BC,
        Self::Color1CC,
        Self::Color220,
        Self::Color29C,
        Self::Color1F4,
        Self::Color20C,
        Self::Color2AC,
    ];

    /// Inspect the canonical color identity; e.g. `Self::Color220.field_path()`.
    pub fn field_path(self) -> &'static str {
        match self {
            Self::Color1BC => "style.color_1bc",
            Self::Color1CC => "style.color_1cc",
            Self::Color220 => "style.color_220",
            Self::Color29C => "style.color_29c",
            Self::Color1F4 => "style.color_1f4",
            Self::Color20C => "style.color_20c",
            Self::Color2AC => "style.color_2ac",
        }
    }
}

/// Scalar checks share the actual alpha storage of their enclosing colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStyleComparisonScalar {
    /// Model-tessellation alpha at220+12.
    Alpha22C,
    /// Final scalar at2AC+12.
    Alpha2B8,
}

impl SimulationStyleComparisonScalar {
    /// Inspect the actual color owning this scalar's storage; e.g. `Self::Alpha22C.color()`.
    pub fn color(self) -> SimulationStyleComparisonColor {
        match self {
            Self::Alpha22C => SimulationStyleComparisonColor::Color220,
            Self::Alpha2B8 => SimulationStyleComparisonColor::Color2AC,
        }
    }
}

/// One reached paired dependency check, in native execution order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStyleComparisonCheck {
    /// An exact paired raw-word check.
    Word(SimulationStyleComparisonWord),
    /// A paired scalar UCOMISS/JNE check under explicit SSE assumptions.
    Scalar(SimulationStyleComparisonScalar),
    /// A paired color check, possibly stopped at its first differing channel.
    Color(SimulationStyleComparisonColor),
    /// The existing complete conditional legend comparator.
    Legend,
}
