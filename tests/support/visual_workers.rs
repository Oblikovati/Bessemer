use std::fmt::Debug;

/// Compares every worker with its serial snapshot oracle; e.g. `assert_eight_visual_workers(&serial, || project(&source));`.
pub(crate) fn assert_eight_visual_workers<Outcome, Project>(expected: &Outcome, project: Project)
where
    Outcome: Debug + PartialEq + Send,
    Project: Fn() -> Outcome + Sync,
{
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(&project)).collect();
        for worker in workers {
            assert_eq!(&worker.join().unwrap(), expected);
        }
    });
}
