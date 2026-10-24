/// A floating-point archive value whose NaN payload and signed zero are retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoredDouble(u64);

impl StoredDouble {
    /// Constructs a stored value without interpreting or canonicalizing its bits.
    ///
    /// ```
    /// assert_eq!(bessemer::header::StoredDouble::from_bits(1).bits(), 1);
    /// ```
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns the exact stored IEEE-754 representation.
    ///
    /// ```
    /// # use bessemer::header::StoredDouble;
    /// assert_eq!(StoredDouble::from_bits(0x8000_0000_0000_0000).bits(), (-0.0_f64).to_bits());
    /// ```
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Interprets the stored bits as a double, without validating geometric meaning.
    ///
    /// ```
    /// # use bessemer::header::StoredDouble;
    /// assert_eq!(StoredDouble::from_bits(1.5_f64.to_bits()).value(), 1.5);
    /// ```
    pub fn value(self) -> f64 {
        f64::from_bits(self.0)
    }
}
