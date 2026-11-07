//! C847 raw position factors remain distinct from unmodeled runtime pixel consumers (#166).
mod types;
mod views;
pub use types::*;

#[cfg(test)]
mod tests {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/simulation_legend_position/boundaries.rs"
    ));
}
