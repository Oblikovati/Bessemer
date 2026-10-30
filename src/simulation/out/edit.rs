use super::{SimulationOutArchive, SimulationOutEdit, SimulationOutFieldId, decimal};
use crate::simulation::{Result, require};

impl SimulationOutArchive {
    /// Replace one exact numeric lexeme and reopen the complete result, e.g. `report.edit_numeric_lexeme(&id, "0.100D-03")?`.
    pub fn edit_numeric_lexeme(
        &self,
        id: &SimulationOutFieldId,
        lexeme: &str,
    ) -> Result<SimulationOutEdit> {
        let archive = self.replaced_lexeme(id, lexeme)?;
        Ok(super::outcome::edit_result(self, archive))
    }

    fn replaced_lexeme(&self, id: &SimulationOutFieldId, lexeme: &str) -> Result<Self> {
        let field = self.field(id)?;
        self.validate_replacement(field, lexeme)?;
        if field.numeric_lexeme.as_ref() == lexeme {
            return Ok(self.clone());
        }
        let lines = self.replacement_lines(field, lexeme);
        let archive = Self::from_lines(&lines, self.limits)?;
        require(
            archive.fields.len() == self.fields.len(),
            "OUT replacement occurrences",
            format!(
                "{} resulting, {} original; expected unchanged occurrence count",
                archive.fields.len(),
                self.fields.len()
            ),
        )?;
        Ok(archive)
    }
    fn replacement_lines(
        &self,
        field: &super::SimulationOutField,
        lexeme: &str,
    ) -> Vec<super::SimulationOutLineInput> {
        self.lines
            .iter()
            .enumerate()
            .map(|(index, line)| {
                if index != field.line_index {
                    return line.input.clone();
                }
                replaced_line(line, field, lexeme)
            })
            .collect()
    }

    fn validate_replacement(&self, field: &super::SimulationOutField, lexeme: &str) -> Result<()> {
        let size = self
            .bytes
            .len()
            .checked_sub(field.numeric_lexeme.len())
            .and_then(|size| size.checked_add(lexeme.len()));
        require(
            size.is_some_and(|size| size <= self.limits.maximum_bytes),
            "OUT replacement size",
            format!(
                "{} source bytes minus {} token bytes plus {}; expected at most {} wrapped bytes",
                self.bytes.len(),
                field.numeric_lexeme.len(),
                lexeme.len(),
                self.limits.maximum_bytes
            ),
        )?;
        validate_token(lexeme)
    }
}

fn replaced_line(
    line: &super::SimulationOutLine,
    field: &super::SimulationOutField,
    lexeme: &str,
) -> super::SimulationOutLineInput {
    let start = field.token_offset - line.stream_offset;
    let mut content = line.input.content[..start].to_vec();
    content.extend_from_slice(lexeme.as_bytes());
    content.extend_from_slice(&line.input.content[start + field.numeric_lexeme.len()..]);
    super::SimulationOutLineInput {
        content: content.into(),
        crlf: line.input.crlf,
    }
}

fn validate_token(lexeme: &str) -> Result<()> {
    require(
        decimal::parse(lexeme.as_bytes())?.is_some(),
        "OUT replacement token",
        format!(
            "{lexeme:?}; expected a signed decimal token with optional E/D exponent and no whitespace"
        ),
    )
}
