use super::ConfigurationId;
use crate::BessemerError;

pub(super) fn selected_feature_error(
    mut error: BessemerError,
    configuration: ConfigurationId,
) -> BessemerError {
    let name = configuration.stream_name();
    let location = std::mem::replace(&mut error.entity, format!("{name}-ResolvedFeatures"));
    error.operation = "part resolved features";
    // #64: shared field lookups carry the model name rather than a distinct dependency.
    if location != name {
        error.cause = format!("{location}: {}", error.cause);
    }
    error
}

pub(super) fn manager_dependency(mut error: BessemerError) -> BessemerError {
    error.entity = format!("Contents/CMgr: {}", error.entity);
    error
}
