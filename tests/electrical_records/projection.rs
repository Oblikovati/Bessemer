use crate::sqlite_wire::{
    write_sqlite_blob as blob, write_sqlite_count as word, write_sqlite_value as value,
};

pub(super) fn assert_matches(view: &ElectricalProjectView, expected: &[u8], name: &str) {
    let actual = render(view);
    let difference = actual
        .iter()
        .zip(expected)
        .position(|(left, right)| left != right);
    assert!(
        actual == expected,
        "{name}: {} versus {} bytes, first difference {difference:?}",
        actual.len(),
        expected.len()
    );
}
use bessemer::electrical::*;
use bessemer::sqlite_image::{SqliteRows, SqliteValue};

pub(super) fn render(view: &ElectricalProjectView) -> Vec<u8> {
    let ids: Vec<_> = view.source().members().map(|member| member.id).collect();
    let mut encoded = Vec::new();
    word(&mut encoded, view.projects().len());
    for project in view.projects() {
        member(&mut encoded, &project.catalog, &ids);
        record(&mut encoded, &project.record);
        table(&mut encoded, &project.catalog_versions);
        word(&mut encoded, project.databases.len());
        for database in project.databases.iter() {
            project_database(&mut encoded, database, &ids);
        }
    }
    encoded
}

fn project_database(
    encoded: &mut Vec<u8>,
    database: &ElectricalProjectDatabase,
    ids: &[ElectricalMemberId],
) {
    member(encoded, &database.member, ids);
    word(encoded, database.files.len());
    for link in database.files.iter() {
        file_link(encoded, link, ids);
    }
    for rows in [
        &database.locations,
        &database.functions,
        &database.bundles,
        &database.userdata,
        &database.versions,
    ] {
        table(encoded, rows);
    }
}

fn file_link(encoded: &mut Vec<u8>, link: &ElectricalFileLink, ids: &[ElectricalMemberId]) {
    record(encoded, &link.record);
    optional_text(encoded, link.member_name.as_deref());
    word(encoded, link.members.len());
    for id in link.members.iter() {
        member(encoded, id, ids);
    }
    optional_text(encoded, link.file_type_constant());
}

fn record(encoded: &mut Vec<u8>, row: &ElectricalRecord) {
    word(encoded, row.columns().len());
    for (name, cell) in row.columns().iter().zip(row.values()) {
        blob(encoded, name.as_bytes());
        value(encoded, cell);
    }
}

fn table(encoded: &mut Vec<u8>, rows: &SqliteRows) {
    word(encoded, rows.rows().len());
    for row in rows.rows() {
        word(encoded, rows.columns().len());
        for (name, cell) in rows.columns().iter().zip(row) {
            blob(encoded, name.as_bytes());
            value(encoded, cell);
        }
    }
}

fn member(encoded: &mut Vec<u8>, id: &ElectricalMemberId, ids: &[ElectricalMemberId]) {
    word(
        encoded,
        ids.iter().position(|candidate| candidate == id).unwrap(),
    );
}

fn optional_text(encoded: &mut Vec<u8>, text: Option<&str>) {
    value(
        encoded,
        &text
            .map(|text| SqliteValue::Text(text.as_bytes().into()))
            .unwrap_or(SqliteValue::Null),
    );
}
