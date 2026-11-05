use super::{ElectricalFileLink, SqliteValue};

// Specification 14.25, C-264: build-bound managed Constant rows plus native fil_filetype binding.
const DECLARED_FILE_CONSTANTS: [&str; 23] = [
    "kFileFolder",
    "kUnknowFileType",
    "kFileFolio",
    "kFileLineDiagram",
    "kFileBOM",
    "kFileTerminal",
    "kFileOther",
    "kFileCoverPage",
    "kFileSWCabinetLayout",
    "kFileAppendixDxfDwg",
    "kFile2DDrawingFrom3D",
    "kFile2DCabinetLayout",
    "kFileFlattenedRoute",
    "kFileDesignRule",
    "kFileMixed",
    "kFilePID",
    "kFileNotSupported",
    "kFileFluid",
    "kFileExportedReport",
    "kFileExportedDesignRule",
    "kFilePhysicalProduct",
    "kFileSWPhysicalProduct",
    "kFileMax",
];

impl ElectricalFileLink {
    /// Names an observed-build integer constant without validating persistence; e.g. `let name = link.file_type_constant()`.
    pub fn file_type_constant(&self) -> Option<&'static str> {
        let SqliteValue::Integer(code) = self.record.value("fil_filetype")? else {
            return None;
        };
        let index = usize::try_from(code.checked_add(2)?).ok()?;
        DECLARED_FILE_CONSTANTS.get(index).copied()
    }
}
