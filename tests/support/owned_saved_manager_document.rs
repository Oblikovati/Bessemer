use bessemer::Document;

/// Match the saved-manager fixture generator context; e.g. `prepared_saved_manager_document(raw)`.
pub(crate) fn prepared_saved_manager_document(raw: &[u8]) -> Document {
    let mut document = crate::owned_resolved_document::prepared_owned_resolved_document(raw);
    document
        .replace_stream(
            "Contents/CMgr",
            include_bytes!("../fixtures/sheet-metal/manager-0.bin").to_vec(),
        )
        .unwrap();
    Document::from_bytes(&document.to_bytes().unwrap()).unwrap()
}
