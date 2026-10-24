//! Native container fixtures checked against the independent Python reader.

use bessemer::{BessemerError, Document, FailureClass, StreamInfo};
use flate2::Crc;

const FIXTURES: [(&str, &[u8]); 3] = [
    (
        "blank-part.SLDPRT",
        include_bytes!("fixtures/blank-part.SLDPRT"),
    ),
    (
        "blank-assembly.SLDASM",
        include_bytes!("fixtures/blank-assembly.SLDASM"),
    ),
    (
        "blank-drawing.SLDDRW",
        include_bytes!("fixtures/blank-drawing.SLDDRW"),
    ),
];

#[test]
fn native_streams_match_python_reference() {
    for (filename, bytes) in FIXTURES {
        let document = Document::from_bytes(bytes).unwrap();
        let actual: Vec<String> = document
            .streams()
            .map(|stream| {
                let expanded = document.read_stream(stream.name).unwrap();
                let mut crc = Crc::new();
                crc.update(&expanded);
                assert_eq!(expanded.len(), stream.uncompressed_size as usize);
                format!(
                    "{filename}\t{}\t{}\t{:08x}",
                    stream.name,
                    expanded.len(),
                    crc.sum()
                )
            })
            .collect();
        let expected: Vec<&str> = include_str!("fixtures/streams.tsv")
            .lines()
            .filter(|line| line.starts_with(filename))
            .collect();
        assert_eq!(actual, expected, "{filename}");
        assert_eq!(document.version(), 4);
        assert_eq!(document.key().to_be_bytes(), bytes[..4]);
        assert_eq!(document.to_bytes().unwrap(), bytes);
    }
}

#[test]
fn edits_are_deterministic_and_isolated() {
    for (_, bytes) in FIXTURES {
        let source = Document::from_bytes(bytes).unwrap();
        let mut edited = source.clone();
        let name = source.streams().next().unwrap().name;
        let replacement = b"container-level replacement".to_vec();
        edited.replace_stream(name, replacement.clone()).unwrap();
        let serialized = edited.to_bytes().unwrap();
        assert_eq!(serialized, edited.to_bytes().unwrap());
        let reopened = Document::from_bytes(&serialized).unwrap();
        assert_eq!(reopened.read_stream(name).unwrap(), replacement);
        for stream in source.streams().filter(|stream| stream.name != name) {
            assert_eq!(
                source.read_stream(stream.name).unwrap(),
                reopened.read_stream(stream.name).unwrap()
            );
        }
        assert_eq!(source.to_bytes().unwrap(), bytes);
    }
}

#[test]
fn missing_stream_and_limits_have_typed_errors() {
    let mut document = Document::from_bytes(FIXTURES[0].1).unwrap();
    assert_eq!(
        document.read_stream("missing").unwrap_err().class,
        FailureClass::Missing
    );
    assert!(document.replace_stream("missing", vec![]).is_err());
    assert_eq!(document.to_bytes().unwrap(), FIXTURES[0].1);
    let name = document
        .streams()
        .find(|stream| stream.uncompressed_size > 0)
        .unwrap()
        .name
        .to_owned();
    assert_eq!(
        document.read_stream_limited(&name, 0).unwrap_err().class,
        FailureClass::Unsupported
    );
    document.replace_stream(&name, vec![1, 2]).unwrap();
    assert!(document.read_stream_limited(&name, 1).is_err());
    assert_eq!(document.read_stream_limited(&name, 2).unwrap(), [1, 2]);
}

#[test]
fn malformed_inputs_do_not_panic() {
    for size in 0..128 {
        assert!(Document::from_bytes(&vec![0; size]).is_err());
    }
    let native = FIXTURES[0].1;
    for size in [8, 30, 46, 100, native.len() / 2] {
        assert!(Document::from_bytes(&native[..size]).is_err());
    }
    let mut wrong_version = native.to_vec();
    wrong_version[7] = 5;
    assert_eq!(
        Document::from_bytes(&wrong_version).unwrap_err().class,
        FailureClass::Unsupported
    );
}

#[test]
fn public_types_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<Document>();
    assert_traits::<StreamInfo<'static>>();
    assert_traits::<BessemerError>();
    assert_traits::<FailureClass>();
}

#[test]
fn concurrent_reads_and_edits_match_serial_results() {
    let source = Document::from_bytes(FIXTURES[0].1).unwrap();
    let expected = source.read_stream("PreviewPNG").unwrap();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    for _ in 0..16 {
                        assert_eq!(source.read_stream("PreviewPNG").unwrap(), expected);
                        let mut isolated = source.clone();
                        isolated.replace_stream("PreviewPNG", vec![]).unwrap();
                        let output = isolated.to_bytes().unwrap();
                        assert!(
                            Document::from_bytes(&output)
                                .unwrap()
                                .read_stream("PreviewPNG")
                                .unwrap()
                                .is_empty()
                        );
                    }
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    assert_eq!(source.to_bytes().unwrap(), FIXTURES[0].1);
}

#[test]
fn native_equivalent_replacements_preserve_every_file_byte() {
    for (_, bytes) in FIXTURES {
        let source = Document::from_bytes(bytes).unwrap();
        for stream in source.streams() {
            let mut edited = source.clone();
            edited
                .replace_stream(stream.name, source.read_stream(stream.name).unwrap())
                .unwrap();
            assert_eq!(edited.to_bytes().unwrap(), bytes, "{}", stream.name);
        }
    }
}
