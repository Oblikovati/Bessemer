use super::{MapSlot, ObjectReader};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct ObjectMapSnapshot(Arc<[MapSlot]>);

impl ObjectReader {
    pub(crate) fn into_snapshot(self) -> ObjectMapSnapshot {
        ObjectMapSnapshot(self.slots.into())
    }

    pub(crate) fn from_snapshot(snapshot: &ObjectMapSnapshot) -> Self {
        Self {
            slots: snapshot.0.to_vec(),
        }
    }
}

impl ObjectMapSnapshot {
    pub(crate) fn slot_count(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{ObjectMapSnapshot, ObjectReader};
    use crate::archive_codec::{ArchiveClass, ArchiveCursor, ObjectRecord};

    fn loaded_map() -> ObjectMapSnapshot {
        let mut reader = ObjectReader::new();
        reader
            .object(&mut ArchiveCursor::new(b"\xff\xff\x01\x00\x08\x00moPart_c"))
            .unwrap();
        reader.append_loaded_object(None).unwrap();
        reader.into_snapshot()
    }

    #[test]
    fn inherited_class_object_and_null_slots_keep_their_physical_identities() {
        let snapshot = loaded_map();
        let mut inherited = ObjectReader::from_snapshot(&snapshot);
        assert_eq!(snapshot.slot_count(), 4);
        let alias = inherited.object(&mut ArchiveCursor::new(&[2, 0])).unwrap();
        assert!(matches!(alias, ObjectRecord::Reference { index, .. } if index.value() == 2));
        let loaded = inherited
            .object(&mut ArchiveCursor::new(&[1, 0x80]))
            .unwrap();
        assert!(matches!(loaded, ObjectRecord::New { index, .. } if index.value() == 4));
        let null = inherited.object(&mut ArchiveCursor::new(&[3, 0])).unwrap();
        assert_eq!(null, ObjectRecord::Null);
    }

    #[test]
    fn each_builder_and_snapshot_clone_remain_isolated_after_success_and_failure() {
        fn shared<T: Send + Sync>() {}
        shared::<ObjectMapSnapshot>();
        shared::<ObjectReader>();
        let source = loaded_map();
        let clone = source.clone();
        let mut changed = ObjectReader::from_snapshot(&source);
        let extra = ArchiveClass {
            name: "moExtObject_c".into(),
            schema: 1,
        };
        changed.append_loaded_object(Some(extra)).unwrap();
        assert!(changed.object(&mut ArchiveCursor::new(&[1, 0])).is_err());
        assert_eq!(changed.into_snapshot().slot_count(), 5);
        assert_eq!(source.slot_count(), 4);
        let mut independent = ObjectReader::from_snapshot(&clone);
        assert!(
            independent
                .object(&mut ArchiveCursor::new(&[4, 0]))
                .is_err()
        );
        assert_eq!(independent.into_snapshot().slot_count(), 4);
    }
}
