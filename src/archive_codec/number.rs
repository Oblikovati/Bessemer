macro_rules! stored_real {
    ($name:ident, $bits:ty, $float:ident, $path:literal) => {
        /// A stored IEEE-754 value whose NaN payload and signed zero are retained.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name($bits);

        impl $name {
            /// Constructs a stored value without interpreting or canonicalizing its bits.
            #[doc = concat!("\n```\nassert_eq!(", $path, "::from_bits(1).bits(), 1);\n```")]
            pub const fn from_bits(bits: $bits) -> Self { Self(bits) }

            /// Returns the exact stored IEEE-754 representation.
            #[doc = concat!("\n```\nassert_eq!(", $path, "::from_bits((-0.0_", stringify!($float), ").to_bits()).bits(), (-0.0_", stringify!($float), ").to_bits());\n```")]
            pub const fn bits(self) -> $bits { self.0 }

            /// Interprets the stored bits without validating geometric meaning.
            #[doc = concat!("\n```\nassert_eq!(", $path, "::from_bits((1.5_", stringify!($float), ").to_bits()).value(), 1.5);\n```")]
            pub fn value(self) -> $float { <$float>::from_bits(self.0) }
        }
    };
}

stored_real!(StoredDouble, u64, f64, "bessemer::header::StoredDouble");

stored_real!(StoredFloat, u32, f32, "bessemer::tessellation::StoredFloat");
