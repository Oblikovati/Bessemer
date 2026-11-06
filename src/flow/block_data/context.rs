use super::*;

/// Explicit assumption required by the documented fresh full-grid derivation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFreshContextAssumption {
    /// Caller assumes fresh runtime context; native construction has not been executed.
    AssumeFresh,
}

/// Immutable derivation with the original grid and tree checkpoints.
#[derive(Clone, Debug)]
pub struct FlowDerivedLeafCount {
    count: FlowLeafCount,
    grid: FlowBaseGrid,
    trees: FlowBlockTrees,
    assumption: FlowFreshContextAssumption,
}

/// Derives only a cardinality under explicit fresh full-grid assumptions; e.g. `let context = derive_fresh_flow_leaf_count(&grid, &trees, FlowFreshContextAssumption::AssumeFresh)?`.
pub fn derive_fresh_flow_leaf_count(
    grid: &FlowBaseGrid,
    trees: &FlowBlockTrees,
    assumption: FlowFreshContextAssumption,
) -> Result<FlowDerivedLeafCount> {
    validate_full_grid(grid, trees)?;
    let leaves = trees.coverage().checked_leaves;
    if leaves > i32::MAX as usize || trees.bounds().grid_count() > i32::MAX as u128 {
        return Err(invalid_block(format!(
            "wire leaves {leaves}, grid count {}; expected both in native nonnegative signed-i32 domain",
            trees.bounds().grid_count()
        )));
    }
    Ok(FlowDerivedLeafCount {
        count: FlowLeafCount::try_from(leaves as u32)?,
        grid: grid.clone(),
        trees: trees.clone(),
        assumption,
    })
}

fn validate_full_grid(grid: &FlowBaseGrid, trees: &FlowBlockTrees) -> Result<()> {
    let roots = legacy_roots(trees)?;
    if grid.default_tree_bounds() != trees.bounds()
        || grid.root_grid_count() != trees.bounds().grid_count()
        || roots.iter().any(|root| root.node_index().is_none())
    {
        return Err(invalid_block(format!(
            "grid bounds {:?}, tree bounds {:?}, {} roots; expected equal complete default-grid ranges and every root present",
            grid.default_tree_bounds(),
            trees.bounds(),
            roots.len()
        )));
    }
    Ok(())
}

impl FlowDerivedLeafCount {
    /// Returns validated derived cardinality; e.g. `let count = context.count()`.
    pub fn count(&self) -> FlowLeafCount {
        self.count
    }
    /// Borrows the complete grid checkpoint; e.g. `let grid = context.grid()`.
    pub fn grid(&self) -> &FlowBaseGrid {
        &self.grid
    }
    /// Borrows the complete tree checkpoint; e.g. `let trees = context.trees()`.
    pub fn trees(&self) -> &FlowBlockTrees {
        &self.trees
    }
    /// Returns the explicit assumption; e.g. `let assumption = context.assumption()`.
    pub fn assumption(&self) -> FlowFreshContextAssumption {
        self.assumption
    }
    /// Reports absence of native context execution; e.g. `assert!(!context.native_context_executed())`.
    pub fn native_context_executed(&self) -> bool {
        false
    }
    /// Reports absence of a block-to-tree association; e.g. `assert!(!context.block_to_tree_order_assigned())`.
    pub fn block_to_tree_order_assigned(&self) -> bool {
        false
    }
}

fn legacy_roots(trees: &FlowBlockTrees) -> Result<&[FlowTreeRoot]> {
    trees.roots().ok_or_else(|| {
        block_failure(
            FailureClass::Missing,
            format!(
                "tree version {} without active legacy roots; expected complete legacy forest",
                trees.version()
            ),
        )
    })
}
