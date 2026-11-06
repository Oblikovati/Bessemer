use super::*;

pub(super) enum ProjectCountKind {
    Project,
    Database,
    File,
}

pub(super) fn validate_project_limits(limits: ElectricalProjectLimits) -> Result<()> {
    let bounds = [
        limits.maximum_projects,
        limits.maximum_database_links,
        limits.maximum_file_links,
        limits.maximum_checked_images,
        limits.maximum_accounted_bytes,
    ];
    if bounds.contains(&0) {
        return Err(project_failure(
            "limits",
            format!("{limits:?}; expected every project bound >= 1"),
        ));
    }
    // Even an empty catalog projection must reject invalid nested SQLite limits.
    crate::sqlite_image::validate_limits(limits.sqlite)
}

impl ProjectReadContext<'_> {
    pub(super) fn charge_project_bytes(&mut self, bytes: usize) -> Result<()> {
        let used = self.coverage.accounted_bytes;
        let maximum = self.limits.maximum_accounted_bytes;
        if used.checked_add(bytes).is_none_or(|total| total > maximum) {
            return Err(project_failure(
                "aggregate bytes",
                format!("{used} + {bytes}; expected at most {maximum} accounted bytes"),
            ));
        }
        self.coverage.accounted_bytes += bytes;
        Ok(())
    }

    fn project_counter(&mut self, kind: ProjectCountKind) -> (&mut usize, usize, usize) {
        let (count, maximum, bytes) = match kind {
            ProjectCountKind::Project => (
                &mut self.coverage.projects,
                self.limits.maximum_projects,
                size_of::<ElectricalProject>(),
            ),
            ProjectCountKind::Database => (
                &mut self.coverage.database_links,
                self.limits.maximum_database_links,
                size_of::<ElectricalProjectDatabase>(),
            ),
            ProjectCountKind::File => (
                &mut self.coverage.file_links,
                self.limits.maximum_file_links,
                size_of::<ElectricalFileLink>(),
            ),
        };
        (count, maximum, bytes)
    }

    pub(super) fn charge_project_count(&mut self, kind: ProjectCountKind) -> Result<()> {
        let (count, maximum, bytes) = self.project_counter(kind);
        if *count >= maximum {
            return Err(project_failure(
                "project links",
                format!("more than {count}; expected at most {maximum} links"),
            ));
        }
        *count += 1;
        self.charge_project_bytes(bytes)
    }

    pub(super) fn charge_record_clones(&mut self, rows: &SqliteRows) -> Result<()> {
        for name in rows.columns() {
            self.charge_project_bytes(name.len() + size_of::<String>())?;
        }
        for row in rows.rows() {
            self.charge_project_bytes(
                size_of::<ElectricalRecord>() + row.len() * size_of::<SqliteValue>(),
            )?;
        }
        Ok(())
    }

    pub(super) fn charge_project_image(&mut self, image: &SqliteImage) -> Result<()> {
        self.charge_project_bytes(image.as_bytes().len())?;
        for object in image.objects() {
            self.charge_project_bytes(size_of::<crate::sqlite_image::SqliteSchemaObject>())?;
            self.charge_project_bytes(object.kind.len() + object.name.len() + object.table.len())?;
            self.charge_project_bytes(object.sql.as_ref().map_or(0, String::len))?;
            for rows in [&object.columns, &object.foreign_keys]
                .into_iter()
                .flatten()
            {
                self.charge_project_bytes(rows.accounted_bytes())?;
            }
        }
        self.coverage.checked_images += 1;
        self.coverage.checked_image_bytes += image.as_bytes().len();
        Ok(())
    }
}
