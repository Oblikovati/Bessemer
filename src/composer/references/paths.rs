use super::{ComposerReferenceText, Result};

impl ComposerReferenceText {
    /// Native lexical filename, e.g. `path.file_name()`; backslash takes priority.
    pub fn file_name(&self) -> Self {
        let start = self
            .last(92)
            .or_else(|| self.last(47))
            .map_or(0, |at| at + 1);
        self.slice(start, self.0.len())
    }

    /// Backslash-only directory, e.g. `path.file_folder()`; absent directory yields `.\`.
    pub fn file_folder(&self) -> Self {
        self.last(92)
            .map_or_else(|| Self(vec![46, 92].into()), |at| self.slice(0, at))
    }

    /// Remove the last dot and following units, e.g. `path.file_base_path()`.
    pub fn file_base_path(&self) -> Self {
        self.slice(0, self.last(46).unwrap_or(self.0.len()))
    }

    /// Native false-mode basename, e.g. `path.file_base()`; forward slashes remain.
    pub fn file_base(&self) -> Self {
        let end = self.last(46).unwrap_or(self.0.len());
        let start = self.last(92).filter(|at| *at < end).map_or(0, |at| at + 1);
        self.slice(start, end)
    }

    /// Plain-reference initial key with explicit ASCII profile, e.g. `project.initial_file_key()?`.
    pub fn initial_file_key(&self) -> Result<Self> {
        self.ascii()?;
        if self
            .file_name()
            .ascii()?
            .eq_ignore_ascii_case("scenario.xml")
        {
            return Ok(self.file_folder().appended("\\geometry.geom"));
        }
        Ok(self.file_base_path().appended(".smggeom"))
    }

    /// Ordered lexical candidates without deduplication, e.g. `file.fallback_file_keys(&project)`.
    pub fn fallback_file_keys(&self, project: &Self) -> [Self; 4] {
        let replaced = Self(
            self.0
                .iter()
                .map(|unit| if *unit == 47 { 92 } else { *unit })
                .collect(),
        );
        [
            self.clone(),
            replaced.file_name(),
            project.file_base().appended(".geom"),
            Self(
                vec![
                    103, 101, 111, 109, 101, 116, 114, 121, 46, 103, 101, 111, 109,
                ]
                .into(),
            ),
        ]
    }
}
