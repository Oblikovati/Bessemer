use super::*;

impl ProjectReadContext<'_> {
    pub(super) fn matching_project_members(&self, name: &str) -> Vec<usize> {
        self.members
            .iter()
            .enumerate()
            .filter_map(|(index, member)| (member.name.as_ref() == name).then_some(index))
            .collect()
    }
}

pub(super) fn read_project_databases(
    context: &mut ProjectReadContext<'_>,
    record: &ElectricalRecord,
) -> Result<Vec<ElectricalProjectDatabase>> {
    let Some(directory) = project_text_field(record, "pro_directory")? else {
        return Ok(Vec::new());
    };
    let prefix = format!("Projects/{directory}/");
    let name = format!("{prefix}Database/tew_project_data.sqlite");
    let mut databases = Vec::new();
    for index in context.matching_project_members(&name) {
        context.charge_project_count(budget::ProjectCountKind::Database)?;
        databases.push(read_project_database(context, index, &prefix)?);
    }
    Ok(databases)
}

fn read_project_database(
    context: &mut ProjectReadContext<'_>,
    index: usize,
    prefix: &str,
) -> Result<ElectricalProjectDatabase> {
    let file_rows = context.project_table(index, "tew_file")?;
    let mut files = Vec::new();
    context.charge_record_clones(&file_rows)?;
    for record in read::project_records_from_rows(&file_rows) {
        context.charge_project_count(budget::ProjectCountKind::File)?;
        files.push(read_file_link(context, record, prefix)?);
    }
    Ok(ElectricalProjectDatabase {
        member: context.members[index].id.clone(),
        files: files.into(),
        locations: context.project_table(index, "tew_location")?,
        functions: context.project_table(index, "tew_function")?,
        bundles: context.project_table(index, "tew_bundle")?,
        userdata: context.project_table(index, "tew_userdata")?,
        versions: context.project_table(index, "tew_version")?,
    })
}

fn read_file_link(
    context: &mut ProjectReadContext<'_>,
    record: ElectricalRecord,
    prefix: &str,
) -> Result<ElectricalFileLink> {
    let member_name = project_text_field(&record, "fil_filename")?
        .map(|filename| format!("{prefix}Drawings/{filename}"));
    let mut members = Vec::new();
    if let Some(name) = &member_name {
        context.charge_project_bytes(name.len())?;
        for index in context.matching_project_members(name) {
            context.charge_project_bytes(size_of::<ElectricalMemberId>())?;
            members.push(context.members[index].id.clone());
        }
    }
    Ok(ElectricalFileLink {
        record,
        member_name,
        members: members.into(),
    })
}

fn project_text_field<'a>(record: &'a ElectricalRecord, name: &str) -> Result<Option<&'a str>> {
    let Some(SqliteValue::Text(bytes)) = record.value(name) else {
        return Ok(None);
    };
    std::str::from_utf8(bytes).map(Some).map_err(|error| {
        project_failure(
            name,
            format!("{error}; expected UTF-8 TEXT for a literal archive name"),
        )
    })
}
