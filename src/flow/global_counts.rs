use crate::{BessemerError, FailureClass, Result, archive_codec::ArchiveCursor};

#[derive(Clone, Copy, Debug)]
pub(super) struct GlobalCountPair {
    fluid: i32,
    solid: i32,
}

impl GlobalCountPair {
    pub(super) fn read(cursor: &mut ArchiveCursor<'_>, operation: &'static str) -> Result<Self> {
        let fluid = cursor.i32()?;
        let solid = cursor.i32()?;
        if fluid < 0 || solid < 0 {
            return Err(BessemerError::new(
                operation,
                "record 406",
                FailureClass::Invalid,
                format!(
                    "G.Fluid {fluid}, G.Solid {solid}; expected nonnegative signed-i32 count profile"
                ),
            ));
        }
        Ok(Self { fluid, solid })
    }

    pub(super) fn pair(self) -> [i32; 2] {
        [self.fluid, self.solid]
    }

    pub(super) fn mathematical_total(self) -> u64 {
        self.fluid as u64 + self.solid as u64
    }

    pub(super) fn native_total(self) -> i32 {
        self.mathematical_total() as u32 as i32
    }
}
