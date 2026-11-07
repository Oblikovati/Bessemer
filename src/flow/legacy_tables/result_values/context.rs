use super::*;
impl FlowImsValueContext {
    /// Validates and retains exact definitions under an explicit declaration; e.g. `let context = FlowImsValueContext::new(&definitions, FlowImsFreshObjects::FreshDeclared)?`.
    pub fn new(
        definitions: &FlowResultDefinitions,
        declaration: FlowImsFreshObjects,
    ) -> Result<Self> {
        for ordinal in 0..definitions.stored_count() {
            let object = definitions.definition(ordinal)?;
            if object.version() != FlowImsDefinitionVersion::V2
                || object.allocation_byte() != Some(0)
            {
                return Err(invalid(format!(
                    "definition {ordinal} version {:?}, allocation {:?}; expected IMS version two with allocation byte zero",
                    object.version(),
                    object.allocation_byte()
                )));
            }
        }
        Ok(Self {
            definitions: definitions.clone(),
            declaration,
        })
    }
    /// Borrows the exact validated definition checkpoint; e.g. `let record = context.definitions().record()`.
    pub fn definitions(&self) -> &FlowResultDefinitions {
        &self.definitions
    }
    /// Returns the explicit fresh-object declaration; e.g. `let declaration = context.declaration()`.
    pub fn declaration(&self) -> FlowImsFreshObjects {
        self.declaration
    }
    /// Returns the inherited physical object count, retaining duplicates; e.g. `let count = context.stored_count()`.
    pub fn stored_count(&self) -> usize {
        self.definitions.stored_count()
    }
}
impl PartialEq for FlowImsValueContext {
    fn eq(&self, other: &Self) -> bool {
        self.definitions.record() == other.definitions.record()
            && self.declaration == other.declaration
    }
}
impl Eq for FlowImsValueContext {}
