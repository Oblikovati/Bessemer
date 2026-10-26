use super::archive_decode::DisplayDecoder;
use crate::Result;
use crate::archive_codec::scene_reader::SceneRead;

impl SceneRead for DisplayDecoder<'_> {
    type ObjectContext = ();

    fn scene_object(&mut self, _: &mut (), label: &str, classes: &[&str]) -> Result<()> {
        self.object(label, classes)
    }

    fn scene_transform(&mut self, label: &str) -> Result<()> {
        self.transform(label)
    }
}
