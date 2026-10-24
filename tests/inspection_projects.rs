//! Complete native Inspection XML oracles and non-destructive drawing edits.
#[path = "support/xml.rs"]
mod support;
use bessemer::{Document, metadata::*};
use support::*;
const STREAM: &str = "ThirdPty/InspectionXpert";
const BLANK: &[u8] = include_bytes!("fixtures/inspection-drawing.SLDDRW");
const JOB: &[u8] = include_bytes!("fixtures/inspection-job.SLDDRW");

#[test]
fn both_complete_native_trees_match_independent_elementtree_oracles() {
    for (bytes, oracle, expected_job, count) in [
        (
            BLANK,
            include_str!("fixtures/inspection/inspection-drawing.tsv"),
            "",
            353,
        ),
        (
            JOB,
            include_str!("fixtures/inspection/inspection-job.tsv"),
            "SCHEMA-JOB-001",
            305,
        ),
    ] {
        let document = Document::from_bytes(bytes).unwrap();
        let raw = document.read_stream(STREAM).unwrap();
        let metadata = parse(STREAM, &raw);
        assert_eq!(
            metadata.stream().family(),
            MetadataFamily::InspectionProject
        );
        assert_eq!(metadata.elements().len(), count);
        assert_eq!(job_number(&metadata).direct_text(), expected_job);
        assert_eq!(&raw[..3], &[0xef, 0xbb, 0xbf]);
        for (element, row) in metadata.elements().zip(oracle.lines()) {
            assert_eq!(element_trace(element), row);
        }
        assert_eq!(oracle.lines().count(), count);
        assert_eq!(
            XmlMetadata::new(stream(STREAM), metadata.source().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            raw
        );
    }
}

fn element_trace(element: XmlElementRef<'_>) -> String {
    let parent = element
        .parent()
        .map_or(-1, |parent| parent.ordinal() as i64);
    let attributes = element
        .attributes()
        .map(|attribute| {
            format!(
                "{}:{}:{}",
                optional_hex(attribute.name.namespace.as_deref()),
                hex(&attribute.name.local),
                hex(&attribute.value)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let children = element
        .children()
        .map(|child| child.ordinal().to_string())
        .collect::<Vec<_>>()
        .join(",");
    let attributes = nonempty(attributes);
    let children = nonempty(children);
    format!(
        "{parent}\t{}\t{}\t{}\t{attributes}\t{children}",
        optional_hex(element.name().namespace.as_deref()),
        element.name().local,
        hex(element.direct_text())
    )
}

fn optional_hex(value: Option<&str>) -> String {
    value.map_or_else(|| "-".into(), hex)
}
fn hex(value: &str) -> String {
    value.bytes().map(|byte| format!("{byte:02x}")).collect()
}

fn child<'a>(parent: XmlElementRef<'a>, local: &str) -> XmlElementRef<'a> {
    parent
        .children()
        .find(|element| {
            element.name().namespace.is_none() && element.name().local.as_ref() == local
        })
        .unwrap()
}

fn job_number(metadata: &XmlMetadata) -> XmlElementRef<'_> {
    child(child(metadata.root().unwrap(), "Settings"), "JobNumber")
}

#[test]
fn native_cache_changes_are_preserved_without_invented_guid_or_current_path_rules() {
    let blank = parse(
        STREAM,
        &Document::from_bytes(BLANK)
            .unwrap()
            .read_stream(STREAM)
            .unwrap(),
    );
    let job = parse(
        STREAM,
        &Document::from_bytes(JOB)
            .unwrap()
            .read_stream(STREAM)
            .unwrap(),
    );
    for metadata in [&blank, &job] {
        let root = metadata.root().unwrap();
        assert_eq!(child(root, "GUID").direct_text(), "SOLIDWORKS Inspection");
        assert_eq!(child(root, "Version").direct_text(), "2026.2.1.2");
        assert_eq!(child(root, "MajorVersion").direct_text(), "2026");
        assert_eq!(metadata.property_sections().count(), 0);
    }
    let blank_drawing = child(blank.root().unwrap(), "ExtCADDrawing");
    let job_drawing = child(job.root().unwrap(), "ExtCADDrawing");
    assert_eq!(
        child(blank_drawing, "CustomProperties").children().len(),
        10
    );
    assert_eq!(child(job_drawing, "CustomProperties").children().len(), 0);
    assert_eq!(
        child(blank_drawing, "Path").direct_text(),
        child(job_drawing, "Path").direct_text()
    );
    assert_eq!(
        child(blank_drawing, "Name").direct_text(),
        child(job_drawing, "Name").direct_text()
    );
}

#[test]
fn job_edits_reopen_with_only_selected_leaf_changed_and_authentic_reverts_restore_drawing() {
    for bytes in [BLANK, JOB] {
        let mut document = Document::from_bytes(bytes).unwrap();
        let raw = document.read_stream(STREAM).unwrap();
        let original = parse(STREAM, &raw);
        let edited = original
            .with_leaf_text(&job_number(&original).id(), "JOB λ <&>\r\n", LIMITS)
            .unwrap();
        document
            .replace_stream(STREAM, edited.to_bytes().unwrap())
            .unwrap();
        let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
        let metadata = parse(STREAM, &reopened.read_stream(STREAM).unwrap());
        assert_eq!(job_number(&metadata).direct_text(), "JOB λ <&>\r\n");
        compare_untouched_elements(&original, &metadata);
        for entry in document.streams().filter(|entry| entry.name != STREAM) {
            assert_eq!(
                document.read_stream(entry.name).unwrap(),
                reopened.read_stream(entry.name).unwrap()
            );
        }
        document.replace_stream(STREAM, raw.clone()).unwrap();
        assert_eq!(document.to_bytes().unwrap(), bytes);
        assert_eq!(original.to_bytes().unwrap(), raw);
    }
}

fn compare_untouched_elements(original: &XmlMetadata, edited: &XmlMetadata) {
    let job = job_number(original).ordinal();
    for (before, after) in original.elements().zip(edited.elements()) {
        assert_eq!(before.name(), after.name());
        assert_eq!(
            before.attributes().collect::<Vec<_>>(),
            after.attributes().collect::<Vec<_>>()
        );
        if before.ordinal() != job {
            assert_eq!(before.direct_text(), after.direct_text());
        }
    }
    assert_eq!(original.elements().len(), edited.elements().len());
}

#[test]
fn inspection_roots_empty_payloads_namespaces_and_invalid_leaf_edits_fail_before_publication() {
    for text in [
        "",
        "<Keywords/>",
        "<ExtInspectionProject xmlns='urn:wrong'/>",
        "<ExtInspectionProject><broken></ExtInspectionProject>",
    ] {
        assert!(
            XmlMetadata::from_bytes(stream(STREAM), text.as_bytes(), LIMITS).is_err(),
            "{text}"
        );
    }
    let original = parse(
        STREAM,
        &Document::from_bytes(JOB)
            .unwrap()
            .read_stream(STREAM)
            .unwrap(),
    );
    let raw = original.to_bytes().unwrap();
    assert!(
        original
            .with_leaf_text(&job_number(&original).id(), "\0", LIMITS)
            .is_err()
    );
    assert!(
        original
            .with_leaf_text(&original.root().unwrap().id(), "drop children", LIMITS)
            .is_err()
    );
    assert_eq!(original.to_bytes().unwrap(), raw);
    assert!(MetadataStream::parse("ThirdPty/inspectionxpert").is_err());
}

#[test]
fn parallel_inspection_edits_equal_serial_bytes_and_leave_the_native_source_isolated() {
    let original = parse(
        STREAM,
        &Document::from_bytes(JOB)
            .unwrap()
            .read_stream(STREAM)
            .unwrap(),
    );
    let raw = original.to_bytes().unwrap();
    let expected: Vec<Vec<u8>> = (0..8).map(|index| edit_job(&original, index)).collect();
    for _ in 0..8 {
        std::thread::scope(|scope| {
            let tasks: Vec<_> = (0..8)
                .map(|index| {
                    let original = &original;
                    scope.spawn(move || edit_job(original, index))
                })
                .collect();
            for (task, expected) in tasks.into_iter().zip(&expected) {
                assert_eq!(task.join().unwrap(), *expected);
            }
        });
    }
    assert_eq!(original.to_bytes().unwrap(), raw);
}

fn edit_job(original: &XmlMetadata, index: usize) -> Vec<u8> {
    original
        .with_leaf_text(
            &job_number(original).id(),
            &format!("JOB-{index} λ"),
            LIMITS,
        )
        .unwrap()
        .to_bytes()
        .unwrap()
}

#[test]
fn construction_and_unknown_populated_extensions_do_not_turn_observed_counts_into_rules() {
    let empty =
        XmlMetadata::new(stream(STREAM), source("<ExtInspectionProject/>"), LIMITS).unwrap();
    assert_eq!(empty.elements().len(), 1);
    let text = "<ExtInspectionProject xmlns:f='urn:future' f:option='not-an-enum'><pObjID_Array><f:unknown id='not-a-uuid'>+001,5</f:unknown></pObjID_Array><Settings><JobNumber>anything</JobNumber></Settings></ExtInspectionProject>";
    let project = XmlMetadata::new(stream(STREAM), source(text), LIMITS).unwrap();
    let unknown = project.element_at(2).unwrap();
    assert_eq!(unknown.name().namespace.as_deref(), Some("urn:future"));
    assert_eq!(unknown.direct_text(), "+001,5");
    assert_eq!(project.to_bytes().unwrap(), text.as_bytes());
    let changed = project
        .with_leaf_text(&job_number(&project).id(), "new job", LIMITS)
        .unwrap();
    assert_eq!(
        changed.element_at(2).unwrap().raw_source(),
        unknown.raw_source()
    );
    assert_eq!(
        project
            .root()
            .unwrap()
            .attribute(Some("urn:future"), "option"),
        Some("not-an-enum")
    );
}

#[test]
fn forced_source_reconstruction_retains_the_complete_native_drawing_bytes() {
    for bytes in [BLANK, JOB] {
        let mut document = Document::from_bytes(bytes).unwrap();
        let metadata = parse(STREAM, &document.read_stream(STREAM).unwrap());
        let constructed =
            XmlMetadata::new(stream(STREAM), metadata.source().clone(), LIMITS).unwrap();
        document
            .replace_stream(STREAM, constructed.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), bytes);
    }
}

fn nonempty(value: String) -> String {
    if value.is_empty() {
        return "-".into();
    }
    value
}
