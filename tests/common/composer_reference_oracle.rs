use bessemer::composer::references::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Oracle {
    Null,
    Bool(bool),
    Integer(i64),
    Text(String),
    List(Vec<Self>),
    Map(BTreeMap<String, Self>),
}

impl Oracle {
    pub fn get(&self, key: &str) -> &Self {
        let Self::Map(entries) = self else {
            panic!("expected map: {self:?}")
        };
        entries.get(key).unwrap()
    }

    pub fn list(&self) -> &[Self] {
        let Self::List(entries) = self else {
            panic!("expected list: {self:?}")
        };
        entries
    }

    pub fn number(&self) -> i64 {
        let Self::Integer(number) = self else {
            panic!("expected integer: {self:?}")
        };
        *number
    }

    pub fn text(&self) -> &str {
        let Self::Text(text) = self else {
            panic!("expected text: {self:?}")
        };
        text
    }

    pub fn boolean(&self) -> bool {
        let Self::Bool(flag) = self else {
            panic!("expected bool: {self:?}")
        };
        *flag
    }

    pub fn units(&self) -> ComposerReferenceText {
        ComposerReferenceText::from_units(
            &self
                .list()
                .iter()
                .map(|unit| u16::try_from(unit.number()).unwrap())
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }
}

pub struct Cursor<'a>(pub &'a [u8]);

impl Cursor<'_> {
    pub fn take(&mut self, count: usize) -> &[u8] {
        let (head, tail) = self.0.split_at(count);
        self.0 = tail;
        head
    }

    pub fn count(&mut self) -> usize {
        u32::from_le_bytes(self.take(4).try_into().unwrap()) as usize
    }

    pub fn oracle(&mut self) -> Oracle {
        match self.take(1)[0] {
            0 => Oracle::Null,
            1 => Oracle::Bool(false),
            2 => Oracle::Bool(true),
            3 => Oracle::Integer(i64::from_le_bytes(self.take(8).try_into().unwrap())),
            4 => {
                let count = self.count();
                Oracle::Text(String::from_utf8(self.take(count).to_vec()).unwrap())
            }
            5 => Oracle::List((0..self.count()).map(|_| self.oracle()).collect()),
            6 => Oracle::Map(
                (0..self.count())
                    .map(|_| (self.oracle().text().to_owned(), self.oracle()))
                    .collect(),
            ),
            tag => panic!("tag {tag}; expected frozen test oracle tag 0..6"),
        }
    }
}

pub fn text(value: &str) -> ComposerReferenceText {
    ComposerReferenceText::from_text(value).unwrap()
}

pub fn limits() -> ComposerServerLimits {
    ComposerServerLimits {
        maximum_files: 100,
        maximum_entries: 10000,
        maximum_key_bytes: 1_000_000,
    }
}

pub fn expected_tier(expected: &Oracle) -> Option<ComposerGeodeTier> {
    match expected {
        Oracle::Null => None,
        Oracle::Text(tier) if tier == "primary" => Some(ComposerGeodeTier::Primary),
        Oracle::Text(tier) if tier == "override" => Some(ComposerGeodeTier::Override),
        _ => panic!("{expected:?}; expected primary, override or null"),
    }
}

pub fn assert_lookup(actual: ComposerGeodeLookup, expected: &Oracle) {
    assert_eq!(actual.found, expected.get("found").boolean());
    assert_eq!(actual.pointer, expected.get("pointer").number() as u64);
    assert_eq!(actual.tier, expected_tier(expected.get("tier")));
}

pub fn entries(expected: &Oracle) -> Vec<(ComposerReferenceText, u64)> {
    expected
        .list()
        .iter()
        .map(|pair| (text(pair.list()[0].text()), pair.list()[1].number() as u64))
        .collect()
}

pub fn assert_attempts(actual: &ComposerReferenceLookup, expected: &Oracle) {
    assert_eq!(actual.completion, ComposerReferenceCompletion::Completed);
    assert_eq!(
        actual.fidelity,
        ComposerReferenceFidelity::ExactSuppliedState
    );
    assert_eq!(actual.pointer, expected.get("pointer").number() as u64);
    let expected = expected.get("attempts").list();
    assert_eq!(actual.attempts.len(), expected.len());
    for (attempt, expected) in actual.attempts.iter().zip(expected) {
        assert_eq!(attempt.file_key, text(expected.get("file_key").text()));
        assert_lookup(attempt.result, expected.get("result"));
    }
}

pub fn encode_lookup(output: &mut Vec<u8>, lookup: &ComposerReferenceLookup) {
    output.extend_from_slice(&lookup.pointer.to_le_bytes());
    output.extend_from_slice(&(lookup.attempts.len() as u32).to_le_bytes());
    for attempt in &lookup.attempts {
        let key = attempt.file_key.code_units();
        output.extend_from_slice(&(key.len() as u32).to_le_bytes());
        for unit in key {
            output.extend_from_slice(&unit.to_le_bytes());
        }
        output.push(u8::from(attempt.result.found));
        output.extend_from_slice(&attempt.result.pointer.to_le_bytes());
        output.push(match attempt.result.tier {
            None => 0,
            Some(ComposerGeodeTier::Override) => 1,
            Some(ComposerGeodeTier::Primary) => 2,
        });
    }
}
