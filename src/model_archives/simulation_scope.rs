use super::ModelDecoder;
use crate::Result;

impl ModelDecoder<'_> {
    pub(in crate::model_archives) fn finish_embedded_prefix(
        self,
    ) -> Result<Vec<crate::model_archives::ModelField>> {
        self.active()?;
        if !self.is_simulation_embedded() {
            return Err(crate::model_archives::invalid(format!(
                "context {:?}; expected explicit Simulation embedded prefix context",
                self.context,
            )));
        }
        Ok(self.fields)
    }

    pub(in crate::model_archives) fn embedded_representation<T>(
        &mut self,
        read: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<T> {
        if self.simulation_representation_depth >= 128 {
            return self.checked(Err(crate::model_archives::invalid(format!(
                "embedded representation depth {}; expected < 128",
                self.simulation_representation_depth,
            ))));
        }
        self.simulation_representation_depth += 1;
        let result = read(self);
        self.simulation_representation_depth -= 1;
        self.checked(result)
    }
}
