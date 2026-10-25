use super::{DisplayFieldValue, archive_decode::DisplayDecoder};
use crate::Result;

impl DisplayDecoder<'_> {
    pub(super) fn view_entries(&mut self, value: &DisplayFieldValue) -> Result<()> {
        match value {
            DisplayFieldValue::NamedViews(Some(views)) => self.named_entries(views),
            DisplayFieldValue::SpatialViews(Some(views)) => {
                self.entries(views.len(), "3D views")?;
                for view in views {
                    self.entries(view.section_planes.len(), "3D-view planes")?;
                }
                Ok(())
            }
            DisplayFieldValue::AnnotationViews(Some(contents)) => {
                self.entries(contents.views.len(), "annotation views")?;
                for view in &contents.views {
                    self.entries(view.section_planes.len(), "annotation planes")?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn named_entries(&mut self, views: &[super::NamedView]) -> Result<()> {
        self.entries(views.len(), "named views")?;
        for view in views {
            if let Some(planes) = &view.section_planes {
                self.entries(planes.len(), "named-view planes")?;
            }
            let values = view
                .selections
                .as_ref()
                .and_then(|selection| selection.values.as_ref());
            if let Some(values) = values {
                self.entries(values.names.len(), "named-view selections")?;
            }
        }
        Ok(())
    }
}
