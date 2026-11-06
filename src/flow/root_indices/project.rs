use super::*;

struct ProjectionBuilder<'a> {
    context: &'a FlowDerivedLeafCount,
    limits: FlowRootIndexLimits,
    roots: Vec<RootSpan>,
    leaves: Vec<LeafSpan>,
    paths: Vec<u8>,
    maximum_depth: u16,
}

enum Task {
    Node(usize),
    Enter(usize, u8),
    Pop,
}

pub(super) fn read(
    context: &FlowDerivedLeafCount,
    limits: FlowRootIndexLimits,
) -> Result<FlowRootIndexProjection> {
    let roots = context.trees().roots().ok_or_else(|| {
        invalid_projection(format!(
            "tree version {} without roots; expected validated complete legacy context",
            context.trees().version()
        ))
    })?;
    check_limits(context, roots.len(), limits)?;
    let mut builder = ProjectionBuilder::new(context, limits, roots.len());
    for root in roots {
        builder.project_root(root)?;
    }
    builder.publish()
}

fn check_limits(
    context: &FlowDerivedLeafCount,
    roots: usize,
    limits: FlowRootIndexLimits,
) -> Result<()> {
    let leaves = context.count().value() as usize;
    if limits.maximum_roots == 0
        || limits.maximum_leaves == 0
        || limits.maximum_path_bytes == 0
        || roots > limits.maximum_roots
        || leaves > limits.maximum_leaves
    {
        return Err(invalid_projection(format!(
            "limits {limits:?}, roots {roots}, leaves {leaves}; expected positive bounds covering all roots/leaves and complete path bytes"
        )));
    }
    Ok(())
}

impl<'a> ProjectionBuilder<'a> {
    fn new(context: &'a FlowDerivedLeafCount, limits: FlowRootIndexLimits, roots: usize) -> Self {
        Self {
            context,
            limits,
            roots: Vec::with_capacity(roots),
            leaves: Vec::with_capacity(context.count().value() as usize),
            paths: Vec::new(),
            maximum_depth: 0,
        }
    }

    fn project_root(&mut self, root: &FlowTreeRoot) -> Result<()> {
        let node = root.node_index().ok_or_else(|| {
            invalid_projection(format!(
                "root {:?} without a node; expected every fresh global root present",
                root.indices()
            ))
        })?;
        let index = self.checked_root_index(root)?;
        let first_leaf = self.leaves.len();
        self.walk(node)?;
        self.roots.push(RootSpan {
            indices: root.indices(),
            storage_index: index,
            first_leaf,
            leaf_end: self.leaves.len(),
        });
        Ok(())
    }

    fn checked_root_index(&self, root: &FlowTreeRoot) -> Result<FlowRootStorageIndex> {
        let index = storage_index(root.indices(), self.context.grid().dimensions())?;
        if index.value() as usize != self.roots.len() {
            return Err(projection_failure(
                FailureClass::Integrity,
                format!(
                    "root storage index {}, position {}; expected unique first-index-fast complete root order",
                    index.value(),
                    self.roots.len()
                ),
            ));
        }
        Ok(index)
    }

    fn walk(&mut self, root: usize) -> Result<()> {
        let mut pending = vec![Task::Node(root)];
        let mut path = Vec::new();
        while let Some(task) = pending.pop() {
            self.execute_task(task, &mut path, &mut pending)?;
        }
        if path.is_empty() {
            return Ok(());
        }
        Err(projection_failure(
            FailureClass::Integrity,
            format!("root {root} residual path {path:?}; expected balanced empty path stack"),
        ))
    }

    fn execute_task(
        &mut self,
        task: Task,
        path: &mut Vec<u8>,
        pending: &mut Vec<Task>,
    ) -> Result<()> {
        match task {
            Task::Node(index) => self.visit(index, path, pending),
            Task::Enter(index, slot) => {
                path.push(slot);
                pending.push(Task::Pop);
                pending.push(Task::Node(index));
                Ok(())
            }
            Task::Pop => pop_path(path),
        }
    }

    fn visit(&mut self, index: usize, path: &[u8], pending: &mut Vec<Task>) -> Result<()> {
        let node = self.context.trees().nodes().get(index).ok_or_else(|| {
            invalid_projection(format!(
                "tree node {index}; expected index below {}",
                self.context.trees().nodes().len()
            ))
        })?;
        if node.children().is_empty() {
            return self.append_leaf(index, path);
        }
        for (slot, child) in node.children().iter().enumerate().rev() {
            pending.push(Task::Enter(child.node_index(), slot as u8));
        }
        Ok(())
    }

    fn append_leaf(&mut self, node: usize, path: &[u8]) -> Result<()> {
        let expected = self.check_leaf_capacity(node, path)?;
        let path_start = self.paths.len();
        self.paths.extend_from_slice(path);
        self.leaves.push(LeafSpan {
            node,
            path_start,
            path_end: self.paths.len(),
        });
        self.maximum_depth = self.maximum_depth.max(expected);
        Ok(())
    }

    fn check_leaf_capacity(&self, node: usize, path: &[u8]) -> Result<u16> {
        let available = self.limits.maximum_path_bytes - self.paths.len();
        let expected = self.context.trees().nodes()[node].wire_depth();
        if path.len() > available
            || path.len() != usize::from(expected)
            || self.leaves.len() >= self.context.count().value() as usize
        {
            return Err(invalid_projection(format!(
                "node {node}, path {} bytes, {} leaves; expected depth {expected}, <= {available} additional path bytes and < {} leaves",
                path.len(),
                self.leaves.len(),
                self.context.count().value()
            )));
        }
        Ok(expected)
    }

    fn publish(self) -> Result<FlowRootIndexProjection> {
        if self.leaves.len() != self.context.count().value() as usize {
            return Err(projection_failure(
                FailureClass::Integrity,
                format!(
                    "projected {} leaves; expected complete {}",
                    self.leaves.len(),
                    self.context.count().value()
                ),
            ));
        }
        let coverage = self.coverage();
        Ok(FlowRootIndexProjection {
            context: self.context.clone(),
            roots: self.roots.into(),
            leaves: self.leaves.into(),
            paths: self.paths.into(),
            limits: self.limits,
            coverage,
        })
    }

    fn coverage(&self) -> FlowRootIndexCoverage {
        FlowRootIndexCoverage {
            mode: FlowRootIndexCheckMode::ExactRootStorageAndLocalPaths,
            checked_roots: self.roots.len(),
            checked_leaves: self.leaves.len(),
            child_path_bytes: self.paths.len(),
            maximum_path_depth: self.maximum_depth,
            achieved_error: 0,
        }
    }
}

fn storage_index(indices: [i32; 3], dimensions: [i32; 3]) -> Result<FlowRootStorageIndex> {
    if indices
        .into_iter()
        .zip(dimensions)
        .any(|(index, size)| index < 0 || index >= size)
    {
        return Err(invalid_projection(format!(
            "root indices {indices:?}, dimensions {dimensions:?}; expected every index in its nonnegative global dimension"
        )));
    }
    let [first, second, third] = indices.map(i128::from);
    let [n0, n1, _] = dimensions.map(i128::from);
    let index = (third * n1 + second) * n0 + first;
    if index > i128::from(i32::MAX) {
        return Err(invalid_projection(format!(
            "root index {index}; expected native nonnegative signed-i32 storage domain"
        )));
    }
    Ok(FlowRootStorageIndex(index as i32))
}

fn pop_path(path: &mut Vec<u8>) -> Result<()> {
    path.pop().ok_or_else(|| {
        projection_failure(
            FailureClass::Integrity,
            "path stack length 0; expected a pending child path to pop".to_owned(),
        )
    })?;
    Ok(())
}
