use super::*;
type ActiveMap = BTreeMap<u32, FlowIndexedSpan>;
type SavedMap = BTreeMap<u64, u64>;
struct ReplayBuilder {
    active: ActiveMap,
    saved: SavedMap,
    options: FlowIndexOptions,
}
pub(super) fn read(
    source: &FlowCarrier,
    policy: FlowIndexPolicy,
    options: FlowIndexOptions,
) -> Result<FlowLogicalIndex> {
    let terminal = match policy {
        FlowIndexPolicy::ForcePhysicalScan => None,
        FlowIndexPolicy::PreferTerminalIndex => recognize::terminal(source)?,
    };
    let mut builder = ReplayBuilder {
        active: ActiveMap::new(),
        saved: SavedMap::new(),
        options,
    };
    let controls = match terminal {
        Some(record) => builder.adopt(source, &record)?,
        None => {
            builder.scan(source)?;
            (None, None)
        }
    };
    Ok(builder.finish(source, policy, controls))
}
type SelectedControls = (Option<FlowTerminalIndex>, Option<FlowSavedSpans>);
impl ReplayBuilder {
    fn scan(&mut self, source: &FlowCarrier) -> Result<()> {
        for record in source.records() {
            let span = physical_span(&record);
            if record.identifier == DELETED {
                self.save(span.record_start, span.record_span)?;
                continue;
            }
            if let Some(old) = self.activate(record.identifier, span)? {
                self.save(old.record_start, old.record_span)?;
            }
        }
        Ok(())
    }
    fn adopt(&mut self, source: &FlowCarrier, record: &FlowRecord<'_>) -> Result<SelectedControls> {
        let terminal = source.terminal_index_writer(
            &record.id,
            FlowControlLengthWidth::EightBytes,
            self.options.controls,
        )?;
        self.index_entries(&terminal)?;
        self.activate(INDEX, physical_span(record))?;
        let saved_control = self.selected_saved(source)?;
        Ok((Some(terminal), saved_control))
    }
    fn index_entries(&mut self, terminal: &FlowTerminalIndex) -> Result<()> {
        for ordinal in 0..terminal.stored_count() {
            let entry = terminal.entry(ordinal)?;
            let span = FlowIndexedSpan {
                record_start: entry.record_start(),
                record_span: entry.record_span(),
            };
            self.activate(entry.identifier(), span)?;
        }
        Ok(())
    }
    fn selected_saved(&mut self, source: &FlowCarrier) -> Result<Option<FlowSavedSpans>> {
        let Some(span) = self.active.get(&SAVED).copied() else {
            return Ok(None);
        };
        let record = matching_saved(source, span)?;
        let control = source.saved_span_writer(&record.id, self.options.controls)?;
        for ordinal in 0..control.stored_count() {
            let entry = control.entry(ordinal)?;
            self.save(entry.key_bits(), entry.value_bits())?;
        }
        Ok(Some(control))
    }
    fn activate(
        &mut self,
        identifier: u32,
        span: FlowIndexedSpan,
    ) -> Result<Option<FlowIndexedSpan>> {
        insert(
            &mut self.active,
            identifier,
            span,
            self.options.maximum_active_entries,
            "active map",
        )
    }
    fn save(&mut self, key: u64, value: u64) -> Result<()> {
        insert(
            &mut self.saved,
            key,
            value,
            self.options.maximum_saved_entries,
            "saved map",
        )?;
        Ok(())
    }
    fn finish(
        self,
        source: &FlowCarrier,
        policy: FlowIndexPolicy,
        controls: SelectedControls,
    ) -> FlowLogicalIndex {
        FlowLogicalIndex {
            source: source.clone(),
            policy,
            options: self.options,
            active: Arc::new(self.active),
            saved: Arc::new(self.saved),
            terminal: controls.0,
            saved_control: controls.1,
        }
    }
}
fn matching_saved(source: &FlowCarrier, span: FlowIndexedSpan) -> Result<FlowRecord<'_>> {
    let record = recognize::at_start(source, span.record_start);
    if let Some(record) = record
        && record.identifier == SAVED
        && physical_span(&record) == span
    {
        return Ok(record);
    }
    Err(invalid(
        "saved-map directive",
        format!(
            "start {} span {}; expected an exact physical record with identifier {SAVED:#x}, matching start and complete span",
            span.record_start, span.record_span
        ),
    ))
}
fn insert<K: Ord + std::fmt::Debug, V>(
    map: &mut BTreeMap<K, V>,
    key: K,
    value: V,
    maximum: usize,
    entity: &str,
) -> Result<Option<V>> {
    if map.len() >= maximum && !map.contains_key(&key) {
        return Err(invalid(
            entity,
            format!(
                "new key {key:?} at distinct count {}; expected fewer than configured maximum {maximum} entries before insertion",
                map.len()
            ),
        ));
    }
    Ok(map.insert(key, value))
}
