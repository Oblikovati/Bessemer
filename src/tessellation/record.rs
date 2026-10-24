macro_rules! slot_type {
    (required, $type:ty) => { $type };
    (optional, $type:ty) => { Option<$type> };
}

macro_rules! read_slot {
    (required, $type:ty, $cursor:expr, $context:expr, $enabled:expr) => {
        <$type as $crate::tessellation::codec::FacetCodec>::read($cursor, $context)
    };
    (optional, $type:ty, $cursor:expr, $context:expr, $enabled:expr) => {{
        let enabled = $enabled;
        $crate::tessellation::codec::read_optional::<$type>($cursor, $context, enabled)
    }};
}

macro_rules! write_slot {
    (required, $type:ty, $value:expr, $output:expr, $context:expr, $enabled:expr, $name:expr) => {
        <$type as $crate::tessellation::codec::FacetCodec>::write($value, $output, $context)
    };
    (optional, $type:ty, $value:expr, $output:expr, $context:expr, $enabled:expr, $name:expr) => {{
        let enabled = $enabled;
        $crate::tessellation::codec::write_optional::<$type>(
            $value, $output, $context, enabled, $name,
        )
    }};
}

macro_rules! archive_record {
    ($(#[$doc:meta])* $name:ident ($context:ident, $minimum:expr, $class:ident) {
        $($(#[$field_doc:meta])* $field:ident : $mode:ident $type:ty => $enabled:expr;)*
    }) => {
        $(#[$doc])* #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name { $($(#[$field_doc])* pub $field: $crate::tessellation::record::slot_type!($mode, $type),)* }
        impl $crate::tessellation::codec::FacetCodec for $name {
            const MINIMUM: usize = $minimum;
            const CLASS: $crate::tessellation::context::CollectionClass = $crate::tessellation::context::CollectionClass::$class;
            fn read(cursor: &mut $crate::archive_codec::ArchiveCursor<'_>, $context: &mut $crate::tessellation::context::FacetState) -> $crate::Result<Self> {
                $context.begin(Self::CLASS)?;
                $(let $field: $crate::tessellation::record::slot_type!($mode, $type) =
                    $crate::tessellation::record::read_slot!($mode, $type, cursor, $context, $enabled)?;)*
                Ok(Self { $($field,)* })
            }
            fn write(&self, output: &mut $crate::tessellation::output::FacetOutput, $context: &mut $crate::tessellation::context::FacetState) -> $crate::Result<()> {
                $context.begin(Self::CLASS)?;
                $($crate::tessellation::record::write_slot!($mode, $type, &self.$field, output, $context, $enabled, stringify!($field))?;)*
                Ok(())
            }
        }
    };
}

pub(super) use {archive_record, read_slot, slot_type, write_slot};
