use super::{
    SimulationOutField, SimulationOutFieldId, SimulationOutLimits, SimulationOutLine,
    SimulationOutLineInput, SimulationOutRevision, syntax,
};
use crate::simulation::{Result, require};

struct FieldBuilder {
    occurrence: SimulationOutField,
    legends: Vec<super::SimulationOutLegend>,
}

struct ReportBuilder {
    lines: Vec<SimulationOutLine>,
    fields: Vec<FieldBuilder>,
    offset: usize,
}

pub(super) fn project(
    inputs: Vec<SimulationOutLineInput>,
    revision: &SimulationOutRevision,
    limits: SimulationOutLimits,
) -> Result<(Vec<SimulationOutLine>, Vec<SimulationOutField>)> {
    let mut builder = ReportBuilder {
        lines: Vec::new(),
        fields: Vec::new(),
        offset: 4,
    };
    for input in inputs {
        builder.append(input, revision, limits.maximum_fields)?;
    }
    let fields = builder.fields.into_iter().map(publish_field).collect();
    Ok((builder.lines, fields))
}

impl ReportBuilder {
    fn append(
        &mut self,
        input: SimulationOutLineInput,
        revision: &SimulationOutRevision,
        maximum_fields: usize,
    ) -> Result<()> {
        let mut line = line(input, self.offset);
        if let Some(parsed) = syntax::field(&line.input.content)? {
            line.field_index = Some(self.fields.len());
            self.append_field(parsed, revision, maximum_fields)?;
        } else {
            attach_legend(&mut self.fields, &mut line, self.lines.len());
        }
        self.offset += line.input.content.len() + if line.input.crlf { 2 } else { 0 };
        self.lines.push(line);
        Ok(())
    }

    fn append_field(
        &mut self,
        parsed: syntax::FieldSyntax,
        revision: &SimulationOutRevision,
        maximum: usize,
    ) -> Result<()> {
        require(
            self.fields.len() < maximum,
            "OUT fields",
            format!(
                "at least {} occurrences; expected at most {maximum}",
                self.fields.len() + 1
            ),
        )?;
        let occurrence = field(
            parsed,
            revision,
            self.fields.len(),
            self.lines.len(),
            self.offset,
        );
        self.fields.push(FieldBuilder {
            occurrence,
            legends: Vec::new(),
        });
        Ok(())
    }
}

fn line(input: SimulationOutLineInput, stream_offset: usize) -> SimulationOutLine {
    SimulationOutLine {
        nul_positions: input
            .content
            .iter()
            .enumerate()
            .filter_map(|(index, byte)| (*byte == 0).then_some(index))
            .collect(),
        input,
        stream_offset,
        field_index: None,
        legend_field_index: None,
    }
}

fn field(
    parsed: syntax::FieldSyntax,
    revision: &SimulationOutRevision,
    index: usize,
    line_index: usize,
    stream_offset: usize,
) -> SimulationOutField {
    SimulationOutField {
        id: SimulationOutFieldId {
            revision: revision.clone(),
            index,
        },
        label: parsed.label,
        symbol: parsed.symbol,
        numeric_lexeme: parsed.lexeme,
        value: parsed.value,
        line_index,
        stream_offset,
        legends: [].into(),
        token_offset: stream_offset + parsed.token_offset,
    }
}

fn attach_legend(fields: &mut [FieldBuilder], line: &mut SimulationOutLine, line_index: usize) {
    let Some(index) = fields.len().checked_sub(1) else {
        return;
    };
    let Some(legend) = syntax::legend(&line.input.content, line_index, line.stream_offset) else {
        return;
    };
    fields[index].legends.push(legend);
    line.legend_field_index = Some(index);
}

fn publish_field(builder: FieldBuilder) -> SimulationOutField {
    let mut occurrence = builder.occurrence;
    occurrence.legends = builder.legends.into();
    occurrence
}
