use super::{ArchiveVersion, invalid, unsupported};
use crate::Result;

/// Whether facet bodies include the full archive's edge-point collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FacetArchiveMode {
    /// Direct full archive body.
    Full,
    /// Lightweight body, omitting edge points.
    Lightweight,
}

/// Explicit context carried outside the tessellation stream itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TessellationContext {
    /// Unsigned archive version controlling documented gates.
    pub archive_version: ArchiveVersion,
    /// Full or lightweight facet serialization.
    pub mode: FacetArchiveMode,
}

/// Allocation limits for complete tessellation decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TessellationLimits {
    /// Maximum component entries across one directory or block stream.
    pub maximum_components: usize,
    /// Maximum facet containers across one block stream.
    pub maximum_faces: usize,
    /// Maximum elements in one dynamic or direct array.
    pub maximum_array_elements: usize,
}

#[derive(Clone, Copy)]
pub(super) enum CollectionClass {
    Element,
    Component,
    Face,
}

pub(super) struct FacetState {
    pub version: u32,
    pub mode: FacetArchiveMode,
    pub curvature: bool,
    pub skip_edge_first: bool,
    limits: TessellationLimits,
    components: usize,
    faces: usize,
    element_budget: Option<(usize, usize)>,
}

impl FacetState {
    pub fn new(context: TessellationContext, limits: TessellationLimits) -> Self {
        Self {
            version: context.archive_version.value(),
            mode: context.mode,
            curvature: false,
            skip_edge_first: false,
            limits,
            components: 0,
            faces: 0,
            element_budget: None,
        }
    }

    pub fn writing(context: TessellationContext) -> Self {
        Self::new(
            context,
            TessellationLimits {
                maximum_components: usize::MAX,
                maximum_faces: usize::MAX,
                maximum_array_elements: usize::MAX,
            },
        )
    }

    pub fn available(&self, class: CollectionClass) -> usize {
        match class {
            CollectionClass::Element => self.limits.maximum_array_elements,
            CollectionClass::Component => self
                .limits
                .maximum_components
                .saturating_sub(self.components),
            CollectionClass::Face => self.limits.maximum_faces.saturating_sub(self.faces),
        }
    }

    pub fn begin(&mut self, class: CollectionClass) -> Result<()> {
        match class {
            CollectionClass::Element => return Ok(()),
            CollectionClass::Component => {
                self.components =
                    charged(self.components, self.limits.maximum_components, "component")?
            }
            CollectionClass::Face => self.begin_face()?,
        }
        Ok(())
    }

    fn begin_face(&mut self) -> Result<()> {
        if (0xa25..=0xc7b).contains(&self.version) {
            return Err(unsupported(format!(
                "facet archive version {:#x}; expected a documented layout outside unresolved object range 0xA25..=0xC7B",
                self.version
            )));
        }
        self.faces = charged(self.faces, self.limits.maximum_faces, "face")?;
        self.curvature = false;
        self.skip_edge_first = false;
        Ok(())
    }

    pub fn require_count(&mut self, count: usize, class: CollectionClass) -> Result<()> {
        let available = self.available(class);
        if count <= available {
            return self.charge_elements(count, class);
        }
        Err(invalid(format!(
            "collection count {count}; expected 0..={available} within allocation budget"
        )))
    }

    pub fn with_element_budget(
        context: TessellationContext,
        limits: TessellationLimits,
        maximum: usize,
    ) -> Self {
        Self {
            element_budget: Some((0, maximum)),
            ..Self::new(context, limits)
        }
    }

    pub fn element_count(&self) -> usize {
        self.element_budget.map_or(0, |(used, _)| used)
    }

    fn charge_elements(&mut self, count: usize, class: CollectionClass) -> Result<()> {
        if !matches!(class, CollectionClass::Element) {
            return Ok(());
        }
        let Some((used, maximum)) = self.element_budget else {
            return Ok(());
        };
        let total = used
            .checked_add(count)
            .ok_or_else(|| invalid(format!("facet elements {used} + {count}; expected usize")))?;
        if total > maximum {
            return Err(invalid(format!(
                "facet elements {total}; expected at most {maximum}"
            )));
        }
        self.element_budget = Some((total, maximum));
        Ok(())
    }
}

fn charged(current: usize, maximum: usize, label: &str) -> Result<usize> {
    let next = current
        .checked_add(1)
        .ok_or_else(|| invalid(format!("{label} count {current} + 1; expected usize")))?;
    if next <= maximum {
        return Ok(next);
    }
    Err(invalid(format!(
        "{label} count {next}; expected at most {maximum}"
    )))
}
