use super::*;

pub(super) fn check_limits(length: usize, limits: DeferredValueLimits) -> Result<()> {
    if length > limits.maximum_bytes || limits.maximum_depth > 64 {
        return Err(invalid(format!(
            "length {length}, depth budget {}; expected bytes <= {} and host depth <= 64",
            limits.maximum_depth, limits.maximum_bytes
        )));
    }
    Ok(())
}

pub(super) struct ValueBudget {
    pub(super) limits: DeferredValueLimits,
    pub(super) used: usize,
    pub(super) deepest: usize,
}

impl ValueBudget {
    pub(super) fn charge(&mut self, depth: usize) -> Result<()> {
        if depth > self.limits.maximum_depth || self.used >= self.limits.maximum_values {
            return Err(invalid(format!(
                "depth {depth}, visited {}; expected depth <= {} and values < {} before visit",
                self.used, self.limits.maximum_depth, self.limits.maximum_values
            )));
        }
        self.used += 1;
        self.deepest = self.deepest.max(depth);
        Ok(())
    }
    pub(super) fn require_children(&self, count: usize) -> Result<()> {
        if count > self.limits.maximum_values - self.used {
            return Err(invalid(format!(
                "child count {count}; expected <= {} remaining values",
                self.limits.maximum_values - self.used
            )));
        }
        Ok(())
    }
}

pub(super) fn unpack(bytes: &[u8], profile: DeferredValueProfile) -> u64 {
    match profile {
        DeferredValueProfile::BigEndianControl0 => bytes
            .iter()
            .fold(0, |word, byte| (word << 8) | u64::from(*byte)),
        DeferredValueProfile::LittleEndianControl1 => bytes
            .iter()
            .rev()
            .fold(0, |word, byte| (word << 8) | u64::from(*byte)),
    }
}

pub(super) fn packed(word: u64, width: usize, profile: DeferredValueProfile) -> Vec<u8> {
    match profile {
        DeferredValueProfile::BigEndianControl0 => word.to_be_bytes()[8 - width..].to_vec(),
        DeferredValueProfile::LittleEndianControl1 => word.to_le_bytes()[..width].to_vec(),
    }
}
