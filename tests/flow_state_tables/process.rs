pub fn transcript(worker: &str, begin: &[u8], end: &[u8]) -> Vec<u8> {
    let output = execute(worker);
    assert!(output.stderr.is_empty());
    let start = marker(&output.stdout, begin) + begin.len();
    let finish = marker(&output.stdout, end);
    assert!(finish > start);
    output.stdout[start..finish].to_vec()
}
fn execute(worker: &str) -> std::process::Output {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", worker, "--ignored", "--nocapture"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    output
}
fn marker(raw: &[u8], pattern: &[u8]) -> usize {
    raw.windows(pattern.len())
        .position(|bytes| bytes == pattern)
        .unwrap()
}
