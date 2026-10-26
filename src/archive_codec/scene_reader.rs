use super::MaterialRead;
use crate::Result;

pub(crate) trait SceneRead: MaterialRead {
    type ObjectContext;
    fn scene_object(
        &mut self,
        context: &mut Self::ObjectContext,
        label: &str,
        classes: &[&str],
    ) -> Result<()>;
    fn scene_transform(&mut self, label: &str) -> Result<()>;
}
