use super::*;

pub(super) fn read_project_view(
    archive: &ElectricalArchive,
    limits: ElectricalProjectLimits,
) -> Result<ElectricalProjectView> {
    budget::validate_project_limits(limits)?;
    let mut context = project_context(archive, limits)?;
    let catalogs = context.matching_project_members("Database/tew_app_project.sqlite");
    let mut projects = Vec::new();
    for catalog in catalogs {
        read_catalog_projects(&mut context, catalog, &mut projects)?;
    }
    Ok(ElectricalProjectView {
        source: archive.clone(),
        projects: projects.into(),
        coverage: context.coverage,
    })
}

fn project_context(
    archive: &ElectricalArchive,
    limits: ElectricalProjectLimits,
) -> Result<ProjectReadContext<'_>> {
    let metadata_bytes = project_metadata_bytes(archive, limits)?;
    Ok(ProjectReadContext {
        archive,
        members: archive.members().collect(),
        limits,
        coverage: empty_project_coverage(metadata_bytes),
        images: BTreeMap::new(),
        tables: BTreeMap::new(),
    })
}

fn empty_project_coverage(metadata_bytes: usize) -> ElectricalProjectCoverage {
    ElectricalProjectCoverage {
        mode: ElectricalProjectCheckMode::LiteralProjectLinks,
        checked_images: 0,
        checked_image_bytes: 0,
        checked_tables: 0,
        checked_rows: 0,
        projects: 0,
        database_links: 0,
        file_links: 0,
        accounted_bytes: metadata_bytes,
    }
}

fn project_metadata_bytes(
    archive: &ElectricalArchive,
    limits: ElectricalProjectLimits,
) -> Result<usize> {
    let count = archive.members().len();
    let bytes = count.checked_mul(size_of::<ElectricalMember>());
    if let Some(bytes) = bytes.filter(|bytes| *bytes <= limits.maximum_accounted_bytes) {
        return Ok(bytes);
    }
    Err(project_failure(
        "member metadata",
        format!(
            "{count} member occurrences; expected metadata within {} accounted bytes",
            limits.maximum_accounted_bytes
        ),
    ))
}

fn read_catalog_projects(
    context: &mut ProjectReadContext<'_>,
    catalog: usize,
    projects: &mut Vec<ElectricalProject>,
) -> Result<()> {
    let rows = context.project_table(catalog, "tew_project")?;
    if rows.rows().is_empty() {
        return Ok(());
    }
    let versions = context.project_table(catalog, "tew_version")?;
    context.charge_record_clones(&rows)?;
    for record in project_records_from_rows(&rows) {
        context.charge_project_count(budget::ProjectCountKind::Project)?;
        let databases = links::read_project_databases(context, &record)?;
        projects.push(ElectricalProject {
            catalog: context.members[catalog].id.clone(),
            record,
            catalog_versions: versions.clone(),
            databases: databases.into(),
        });
    }
    Ok(())
}

pub(super) fn project_records_from_rows(rows: &SqliteRows) -> Vec<ElectricalRecord> {
    let names: Arc<[String]> = rows.columns().into();
    rows.rows()
        .iter()
        .enumerate()
        .map(|(ordinal, values)| ElectricalRecord {
            names: names.clone(),
            values: values.as_slice().into(),
            ordinal,
        })
        .collect()
}

impl ProjectReadContext<'_> {
    pub(super) fn project_table(&mut self, index: usize, name: &'static str) -> Result<SqliteRows> {
        if let Some(rows) = self.tables.get(&(index, name)) {
            return Ok(rows.clone());
        }
        let image = self.project_image(index)?;
        let rows = image
            .table_rows(name)
            .map_err(|error| self.project_member_error(index, error))?;
        self.charge_project_bytes(rows.accounted_bytes())?;
        self.coverage.checked_tables += 1;
        self.coverage.checked_rows += rows.rows().len();
        self.tables.insert((index, name), rows.clone());
        Ok(rows)
    }

    fn project_image(&mut self, index: usize) -> Result<SqliteImage> {
        if let Some(image) = self.images.get(&index) {
            return Ok(image.clone());
        }
        if self.images.len() >= self.limits.maximum_checked_images {
            return Err(project_failure(
                "SQLite images",
                format!(
                    "image {index}, already {}; expected at most {}",
                    self.images.len(),
                    self.limits.maximum_checked_images
                ),
            ));
        }
        let image = self
            .archive
            .sqlite_member(&self.members[index].id, self.limits.sqlite)
            .map_err(|error| self.project_member_error(index, error))?;
        self.charge_project_image(&image)?;
        self.images.insert(index, image.clone());
        Ok(image)
    }

    fn project_member_error(&self, index: usize, mut error: BessemerError) -> BessemerError {
        error.operation = "Electrical project records";
        error.entity = format!(
            "member {index} {:?}/{}",
            self.members[index].name, error.entity
        );
        error
    }
}
