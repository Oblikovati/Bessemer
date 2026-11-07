use super::layout::ParameterGates;
use super::*;

pub(super) struct ParameterEncoding {
    pub bytes: Vec<u8>,
    pub fields: Vec<SimulationParameterField>,
}

struct ParameterEmitter {
    encoding: ParameterEncoding,
    revision: SimulationParameterRevision,
}

pub(super) fn encode(
    parts: &SimulationParameterParts,
    gates: ParameterGates,
    limits: SimulationParameterLimits,
    revision: SimulationParameterRevision,
) -> Result<ParameterEncoding> {
    let shape = super::construction::validate(parts, gates, limits)?;
    let mut emitter = ParameterEmitter::new(shape.size, revision)?;
    emitter.prefix(parts, shape.record_count)?;
    for (index, record) in parts.records.iter().enumerate() {
        emitter.record(index, record)?;
    }
    emitter.tail(&parts.header)?;
    require(
        emitter.encoding.bytes.len() == shape.size,
        "ParameterSets complete construction",
        format!(
            "{} emitted bytes; expected exactly {} validated bytes",
            emitter.encoding.bytes.len(),
            shape.size
        ),
    )?;
    Ok(emitter.encoding)
}

impl ParameterEmitter {
    fn new(size: usize, revision: SimulationParameterRevision) -> Result<Self> {
        Ok(Self {
            encoding: ParameterEncoding {
                bytes: super::super::reserve_bytes(size)?,
                fields: Vec::new(),
            },
            revision,
        })
    }
    fn emit(
        &mut self,
        role: SimulationParameterRole,
        value: SimulationParameterValue,
    ) -> Result<()> {
        let ordinal = self.encoding.fields.len();
        self.encoding.fields.try_reserve(1).map_err(|error| {
            invalid(
                "ParameterSets field allocation",
                format!("field {ordinal}, {error}; expected available bounded field storage"),
            )
        })?;
        let stream_offset = self.encoding.bytes.len();
        value.append_to(&mut self.encoding.bytes);
        let id = SimulationParameterFieldId {
            revision: self.revision.clone(),
            ordinal,
        };
        self.encoding.fields.push(SimulationParameterField {
            id,
            role,
            stream_offset,
            value,
        });
        Ok(())
    }

    fn prefix(&mut self, parts: &SimulationParameterParts, count: i32) -> Result<()> {
        use SimulationParameterRole::*;
        for (index, value) in parts.header.words.iter().enumerate() {
            self.emit(HeaderWord(index), SimulationParameterValue::Signed(*value))?;
        }
        self.optional_word(Gate700013, parts.header.gate700013)?;
        self.optional_word(Gate700018, parts.header.gate700018)?;
        self.emit(RecordCount, SimulationParameterValue::Signed(count))?;
        for (index, value) in parts.table.iter().enumerate() {
            self.emit(TableWord(index), SimulationParameterValue::Signed(*value))?;
        }
        Ok(())
    }

    fn record(&mut self, index: usize, record: &SimulationParameterRecordParts) -> Result<()> {
        use SimulationParameterRole::*;
        self.record_prefix(index, record)?;
        self.record_options(index, &record.numbers)?;
        for (column, value) in record.numbers.values_bits.iter().enumerate() {
            self.emit(
                RecordValue {
                    record: index,
                    column,
                },
                SimulationParameterValue::DoubleBits(*value),
            )?;
        }
        Ok(())
    }

    fn record_prefix(
        &mut self,
        index: usize,
        record: &SimulationParameterRecordParts,
    ) -> Result<()> {
        use SimulationParameterRole::*;
        self.emit(
            RecordText(index),
            SimulationParameterValue::Text(record.text.clone()),
        )?;
        self.emit(
            RecordSignedWord(index),
            SimulationParameterValue::Signed(record.numbers.signed_word),
        )?;
        for (component, value) in record.numbers.unsigned_words.iter().enumerate() {
            self.emit(
                RecordUnsignedWord {
                    record: index,
                    component,
                },
                SimulationParameterValue::Unsigned(*value),
            )?;
        }
        Ok(())
    }

    fn record_options(
        &mut self,
        index: usize,
        numbers: &SimulationParameterRecordNumbers,
    ) -> Result<()> {
        use SimulationParameterRole::*;
        self.optional_word(RecordConditionalWord(index), numbers.conditional_word)?;
        if let Some(value) = numbers.conditional_double_bits {
            self.emit(
                RecordConditionalDouble(index),
                SimulationParameterValue::DoubleBits(value),
            )?;
        }
        Ok(())
    }

    fn tail(&mut self, header: &SimulationParameterHeader) -> Result<()> {
        use SimulationParameterRole::*;
        self.emit(Tail, SimulationParameterValue::Signed(header.tail))?;
        self.optional_word(Gate820015, header.gate820015)?;
        self.optional_word(Gate830009, header.gate830009)
    }

    fn optional_word(&mut self, role: SimulationParameterRole, value: Option<i32>) -> Result<()> {
        if let Some(value) = value {
            self.emit(role, SimulationParameterValue::Signed(value))?;
        }
        Ok(())
    }
}
