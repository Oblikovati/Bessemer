use super::*;

impl ModelParameterSource {
    pub(super) fn fields(&self) -> &[ModelField] {
        match self {
            Self::Configuration(source) => source.fields(),
            Self::ResolvedFeatures(source) => source.fields(),
        }
    }

    pub(super) fn handle(&self, index: usize) -> Result<ModelFieldHandle> {
        match self {
            Self::Configuration(source) => source.field_handle(index),
            Self::ResolvedFeatures(source) => source.field_handle(index),
        }
    }

    pub(super) fn inherited(&self) -> Option<&PartConfigurationArchive> {
        match self {
            Self::Configuration(_) => None,
            Self::ResolvedFeatures(source) => Some(source.configuration_archive()),
        }
    }

    pub(super) fn coverage(&self) -> ModelArchiveCoverage {
        match self {
            Self::Configuration(source) => source.coverage(),
            Self::ResolvedFeatures(source) => source.coverage(),
        }
    }

    /// Borrow the complete owning configuration, including inherited name definitions; e.g. `state.source().configuration_archive()`.
    pub fn configuration_archive(&self) -> &PartConfigurationArchive {
        match self {
            Self::Configuration(source) => source,
            Self::ResolvedFeatures(source) => source.configuration_archive(),
        }
    }

    /// Resolve a copied field in its actual local or inherited revision; e.g. `state.source().resolve_field(&state.history()[0].result)?`.
    pub fn resolve_field(&self, handle: &ModelFieldHandle) -> Result<&ModelField> {
        match self {
            Self::Configuration(source) => source.resolve_field(handle),
            Self::ResolvedFeatures(source)
                if handle.revision == source.configuration_archive().revision =>
            {
                source.configuration_archive().resolve_field(handle)
            }
            Self::ResolvedFeatures(source) => source.resolve_field(handle),
        }
    }
}

impl PartConfigurationArchive {
    /// Enumerate physically defined parameters in wire order; e.g. `source.stored_parameters()?`.
    pub fn stored_parameters(&self) -> Result<Vec<ModelParameterStoredState>> {
        projection::enumerate(ModelParameterSource::Configuration(self.clone()))
    }

    /// Select a physical parameter object tag; e.g. `source.stored_parameter(&definition)?`.
    pub fn stored_parameter(
        &self,
        definition: &ModelFieldHandle,
    ) -> Result<ModelParameterStoredState> {
        self.resolve_field(definition)?;
        projection::project(
            ModelParameterSource::Configuration(self.clone()),
            definition,
        )
        .map_err(|error| super::super::context::selected_error(error, self.context.configuration))
    }
}

impl PartResolvedFeaturesArchive {
    /// Enumerate physical parameter definitions, keeping alias-only occurrences separate; e.g. `source.stored_parameters()?`.
    pub fn stored_parameters(&self) -> Result<Vec<ModelParameterStoredState>> {
        projection::enumerate(ModelParameterSource::ResolvedFeatures(self.clone()))
    }

    /// Select a physically defined parameter in this revision; e.g. `source.stored_parameter(&definition)?`.
    pub fn stored_parameter(
        &self,
        definition: &ModelFieldHandle,
    ) -> Result<ModelParameterStoredState> {
        self.resolve_field(definition)?;
        projection::project(
            ModelParameterSource::ResolvedFeatures(self.clone()),
            definition,
        )
        .map_err(|error| {
            super::super::resolved_failure::selected_feature_error(error, self.configuration())
        })
    }
}
