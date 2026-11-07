use super::super::thermal::{SimulationThermalField, SimulationThermalValue};
use super::{SimulationPostColorView, SimulationRgbaColor};
use crate::{Result, simulation::invalid};

type ColorComponent<'a> = (&'a str, (usize, usize), Option<u32>);

fn collect_colors<'a>(
    mut components: impl Iterator<Item = ColorComponent<'a>>,
) -> Result<Vec<SimulationPostColorView>> {
    let mut colors = Vec::new();
    while let Some(first) = components.next() {
        let Some(prefix) = first.0.strip_suffix(".component_0") else {
            continue;
        };
        colors.push(collect_group(first, prefix, &mut components)?);
    }
    Ok(colors)
}

fn collect_group<'a>(
    first: ColorComponent<'a>,
    prefix: &str,
    components: &mut impl Iterator<Item = ColorComponent<'a>>,
) -> Result<SimulationPostColorView> {
    let start = first.1.0;
    let mut bits = [require_component(first, first.0, start)?; 4];
    for (index, component) in bits.iter_mut().enumerate().skip(1) {
        let expected = format!("{prefix}.component_{index}");
        let next = next_component(components, &expected, start + 4 * index)?;
        *component = require_component(next, &expected, start + 4 * index)?;
    }
    Ok(SimulationPostColorView {
        path: prefix.into(),
        wire_range: (start, start + 16),
        color: SimulationRgbaColor::from_bits(bits),
    })
}

fn require_component(component: ColorComponent<'_>, expected: &str, start: usize) -> Result<u32> {
    if component.0 != expected || component.1 != (start, start + 4) || component.2.is_none() {
        return Err(invalid(
            "COGLColor component",
            format!(
                "{component:?}; expected {expected} float32 pattern at {start}..{}",
                start + 4
            ),
        ));
    }
    component
        .2
        .ok_or_else(|| invalid(expected, "missing pattern; expected float32 component"))
}

fn next_component<'a>(
    components: &mut impl Iterator<Item = ColorComponent<'a>>,
    expected: &str,
    start: usize,
) -> Result<ColorComponent<'a>> {
    components.next().ok_or_else(|| {
        invalid(
            expected,
            format!("end of fields; expected component at byte {start}"),
        )
    })
}

pub(in crate::simulation::persistence) fn collect_wire_colors(
    fields: &[SimulationThermalField],
    spans: &[(usize, usize)],
) -> Result<Vec<SimulationPostColorView>> {
    if fields.len() != spans.len() {
        return Err(invalid(
            "COGLColor spans",
            format!(
                "{} fields and {} spans; expected equal counts",
                fields.len(),
                spans.len()
            ),
        ));
    }
    let components = fields.iter().zip(spans).map(|(field, span)| {
        let bits = match field.value {
            SimulationThermalValue::Unsigned(bits) => Some(bits),
            _ => None,
        };
        (field.path.as_ref(), *span, bits)
    });
    collect_colors(components)
}

#[cfg(test)]
mod tests {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/simulation_rgba/collect.rs"
    ));
}
