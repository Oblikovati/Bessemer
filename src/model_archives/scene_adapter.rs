use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{ObjectReader, scene_reader::SceneRead};

impl SceneRead for ModelDecoder<'_> {
    type ObjectContext = ObjectReader;

    fn scene_object(
        &mut self,
        map: &mut ObjectReader,
        label: &str,
        classes: &[&str],
    ) -> Result<()> {
        self.object(label, map, ModelObjectRole::Declared(classes))
    }

    fn scene_transform(&mut self, label: &str) -> Result<()> {
        self.transform(label)
    }
}
