use super::*;
pub fn cases() -> Vec<ConditionalCase> {
    let mut result = Vec::new();
    for compressed in [
        &include_bytes!("../fixtures/flow-conditional-items/record126.bin.gz")[..],
        &include_bytes!("../fixtures/flow-conditional-items/record315.bin.gz")[..],
        &include_bytes!("../fixtures/flow-conditional-items/record128.bin.gz")[..],
    ] {
        let mut cursor = cursor(compressed, *b"FCI1");
        result.extend(counted(&mut cursor, case_reader::case));
        finish(&cursor);
    }
    result
}
