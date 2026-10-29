use super::header_imports::ImportedHeaderMap;
use super::{
    AssemblyConfigurationContext, AssemblyRootKind, ModelArchiveLimits, ModelDecoder, ModelField,
    ModelObjectRole,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectMapSnapshot, ObjectReader, ObjectRecord};

pub(super) struct ParsedAssemblyArchive {
    pub fields: Vec<ModelField>,
    pub imports: ImportedHeaderMap,
    pub object_map: ObjectMapSnapshot,
    pub wire_bytes: usize,
}

pub(super) fn read(
    bytes: &[u8],
    context: &AssemblyConfigurationContext,
    limits: ModelArchiveLimits,
) -> Result<ParsedAssemblyArchive> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        context.version,
        limits,
        super::reader::ModelArchiveContext::OrdinaryAssemblyExternalXml(context.kind),
    )?
    .with_kernel(context.kernel.clone());
    reader.require_ordinary_model()?;
    let mut map = ObjectReader::new();
    let imports = sections(&mut reader, &mut map, context, limits)?;
    Ok(ParsedAssemblyArchive {
        fields: reader.finish()?,
        imports,
        object_map: map.into_snapshot(),
        wire_bytes: bytes.len(),
    })
}

fn sections(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    context: &AssemblyConfigurationContext,
    limits: ModelArchiveLimits,
) -> Result<ImportedHeaderMap> {
    root(reader, map, context.kind)?;
    let size = reader.section_size("lightweight_size")?;
    let (imports, flags) =
        reader.bounded(size, |reader| lightweight(reader, map, context, limits))?;
    let size = reader.section_size("resolved_size")?;
    reader.bounded(size, |reader| resolved(reader, map, flags))?;
    if context.kind == AssemblyRootKind::RoutingAssembly {
        reader.scope("routing_trailer", super::assembly_tail::routing)?;
    }
    Ok(imports)
}

fn root(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    kind: AssemblyRootKind,
) -> Result<()> {
    let root = reader.object_tag("root", map, ModelObjectRole::Declared(kind.classes()))?;
    let result = match root {
        ObjectRecord::New { .. } => Ok(()),
        actual => Err(super::invalid(format!(
            "root {actual:?}; expected a fresh {:?} object",
            kind.classes()
        ))),
    };
    reader.checked(result)
}

fn lightweight(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    context: &AssemblyConfigurationContext,
    limits: ModelArchiveLimits,
) -> Result<(ImportedHeaderMap, u32)> {
    let plan = super::header_imports::ModelHeaderLoadPlan::new(
        context.header.clone(),
        context.version,
        limits.maximum_entries,
    )?;
    let imports = plan.append(reader, map)?;
    let node = super::lightweight::model_after_header(
        reader,
        map,
        super::lightweight::UnitComponentContext::External,
    )?;
    reader.scope("assembly_lightweight", super::assembly_tail::lightweight)?;
    let flags = node.unknown_2c.ok_or_else(|| {
        super::invalid(format!(
            "node flags {:?}; expected modern original u32",
            node.unknown_2c
        ))
    })?;
    Ok((imports, flags))
}

fn resolved(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, flags: u32) -> Result<()> {
    super::resolved_model::prefix(reader, map)?;
    reader.scope("reference_tail", |reader| {
        super::reference_tail::assembly(reader, map, flags)
    })?;
    reader.scope("shared_model_tail", |reader| {
        super::shared_tail::read(reader, map)
    })?;
    reader.scope("assembly_resolved_tail", |reader| {
        super::assembly_tail::resolved(reader, map)
    })
}

pub(super) fn unit_component(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.object(
        "owner",
        map,
        ModelObjectRole::Declared(&["moAssembly_c", "moRouteAssembly_c"]),
    )?;
    let present = reader.u8("has_external_model")?;
    super::configuration_io::absent(reader, "assembly unit external model", i32::from(present))
}

pub(super) fn construct(
    mut fields: Vec<ModelField>,
    context: &AssemblyConfigurationContext,
    limits: ModelArchiveLimits,
) -> Result<ParsedAssemblyArchive> {
    let end = fields
        .iter()
        .position(|field| field.path.starts_with("root.routing_trailer."))
        .unwrap_or(fields.len());
    super::construction::model_section_lengths(&mut fields, limits.maximum_bytes, end)?;
    let bytes = super::encode_fields(&fields, limits.maximum_bytes)?;
    let parsed = read(&bytes, context, limits)?;
    if fields != parsed.fields {
        return Err(super::field_mismatch::construction_mismatch(
            &fields,
            &parsed.fields,
        ));
    }
    Ok(parsed)
}
