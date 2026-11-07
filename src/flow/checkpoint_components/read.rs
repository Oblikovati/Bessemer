use super::*;
use budget::Counter;
mod grids;
struct Reader<'a> {
    cursor: ArchiveCursor<'a>,
    size: usize,
    options: FlowCheckpointOptions,
    counts: FlowCheckpointCounts,
}
pub(super) fn component(
    bytes: &[u8],
    context: FlowCheckpointContext,
    kind: FlowCheckpointKind,
    options: FlowCheckpointOptions,
) -> Result<FlowCheckpointComponent> {
    input_bound(bytes.len(), options.maximum_component_bytes)?;
    let mut reader = Reader {
        cursor: ArchiveCursor::new(bytes),
        size: bytes.len(),
        options,
        counts: FlowCheckpointCounts::default(),
    };
    let layout = reader.layout(kind)?;
    reader.cursor.finish()?;
    Ok(publish(
        bytes,
        context,
        kind,
        options,
        layout,
        reader.counts,
    ))
}
fn input_bound(length: usize, maximum: usize) -> Result<()> {
    if length <= maximum {
        return Ok(());
    }
    Err(invalid(
        "component bytes",
        format!("length {length}; expected at most configured bound {maximum} bytes"),
    ))
}

fn publish(
    bytes: &[u8],
    context: FlowCheckpointContext,
    kind: FlowCheckpointKind,
    options: FlowCheckpointOptions,
    layout: Layout,
    counts: FlowCheckpointCounts,
) -> FlowCheckpointComponent {
    let revision = CheckpointRevision {
        wire: Arc::from(bytes),
        context,
        kind,
        options,
        layout,
        counts,
        diagnostics: diagnostics(kind, counts),
    };
    FlowCheckpointComponent {
        revision: Arc::new(revision),
    }
}
impl Reader<'_> {
    fn position(&self) -> usize {
        self.size - self.cursor.remaining()
    }
    fn admit(&mut self, kind: Counter, count: usize) -> Result<()> {
        budget::admit(&mut self.counts, self.options, kind, count)
    }
    fn layout(&mut self, kind: FlowCheckpointKind) -> Result<Layout> {
        match kind {
            FlowCheckpointKind::Graph1 => {
                Ok(Layout::Graph1(self.patterns(7, ValueShape::Pattern64)?))
            }
            FlowCheckpointKind::Graph2 => Ok(Layout::Grid(self.grid(true)?)),
            FlowCheckpointKind::Auxiliary => Ok(Layout::Grid(self.grid(false)?)),
            FlowCheckpointKind::TrailingContainers => Ok(Layout::Trailing(
                (0..3)
                    .map(|_| self.container())
                    .collect::<Result<Vec<_>>>()?,
            )),
            FlowCheckpointKind::ChunkBuffer => Ok(Layout::Chunks(self.chunks()?)),
        }
    }
    fn patterns(&mut self, count: usize, shape: ValueShape) -> Result<PatternSpan> {
        let kind = pattern_counter(shape)?;
        let bytes = count.checked_mul(shape.width()).ok_or_else(|| {
            invalid(
                "numeric array",
                format!(
                    "count {count} width {}; expected a byte length fitting usize",
                    shape.width()
                ),
            )
        })?;
        self.admit(kind, count)?;
        let start = self.position();
        self.cursor.take(bytes)?;
        Ok(PatternSpan {
            start,
            count,
            shape,
        })
    }
    fn bytes(&mut self, count: usize) -> Result<PatternSpan> {
        let start = self.position();
        self.cursor.take(count)?;
        Ok(PatternSpan {
            start,
            count,
            shape: ValueShape::Byte,
        })
    }
    fn container(&mut self) -> Result<ContainerLayout> {
        self.admit(Counter::Containers, 1)?;
        let start = self.position();
        let count = self.cursor.nonnegative_count(28, "numeric container")?;
        let first = self.patterns(count, ValueShape::Pattern64)?;
        let indices = self.patterns(count, ValueShape::Pattern32)?;
        let second = self.patterns(count * 2, ValueShape::Pattern64)?;
        Ok(ContainerLayout {
            start,
            end: self.position(),
            first,
            indices,
            second,
        })
    }
    fn chunks(&mut self) -> Result<Vec<ChunkLayout>> {
        let version = self.cursor.i32()?;
        if version != 0 {
            return Err(BessemerError::new(
                OPERATION,
                "chunk version",
                FailureClass::Unsupported,
                format!("version {version}; expected signed version zero"),
            ));
        }
        let count = self.cursor.i32()?;
        self.cursor.u64()?;
        let count = self.chunk_count(count)?;
        self.admit(Counter::Chunks, count)?;
        (0..count).map(|_| self.chunk()).collect()
    }
    fn chunk_count(&self, count: i32) -> Result<usize> {
        let available = self.cursor.remaining() / 4;
        if count < 1 || count as usize > available {
            return Err(invalid(
                "chunk count",
                format!(
                    "count {count}; expected 1..={available} complete length headers in remaining bytes"
                ),
            ));
        }
        Ok(count as usize)
    }
    fn chunk(&mut self) -> Result<ChunkLayout> {
        let start = self.position();
        let count = self.cursor.nonnegative_count(1, "encoded chunk")?;
        self.admit(Counter::Encoded, count)?;
        let encoded = self.bytes(count)?;
        Ok(ChunkLayout {
            start,
            end: self.position(),
            encoded,
        })
    }
}
fn diagnostics(
    kind: FlowCheckpointKind,
    counts: FlowCheckpointCounts,
) -> Vec<FlowCheckpointDiagnostic> {
    use FlowCheckpointDiagnostic::*;
    let mut result = vec![
        LogicalContextAssumed,
        InstalledNativeComponentNotVerified,
        ParentRecordNotDecoded,
        EngineeringMeaningsNotAssigned,
        NativeRuntimeNotReproduced,
        NativeApplicationNotChecked,
    ];
    if counts.noncanonical_tags > 0 {
        result.push(NoncanonicalWriterTagsRetained);
    }
    if kind == FlowCheckpointKind::ChunkBuffer {
        result.extend([ChunkCodecNotDecoded, DeclaredRawSizeNotVerified]);
    }
    result
}

fn pattern_counter(shape: ValueShape) -> Result<Counter> {
    match shape {
        ValueShape::Pattern32 => Ok(Counter::Indices),
        ValueShape::Pattern64 => Ok(Counter::Wides),
        other => Err(invalid(
            "numeric array",
            format!("shape {other:?}; expected Pattern32 or Pattern64"),
        )),
    }
}
