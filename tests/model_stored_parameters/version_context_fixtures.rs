pub(super) struct VersionParameterFixture {
    pub case: usize,
    pub version: u32,
    pub graphs: [[&'static [u8]; 5]; 6],
}

pub(super) const CONTEXTS: [VersionParameterFixture; 2] = [
    VersionParameterFixture {
        case: 0,
        version: 11000,
        graphs: [
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-0-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-0-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-0-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-0-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/11000-0-0-edited.bin"
                ),
            ],
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-1-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-1-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-1-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-1-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/11000-1-0-edited.bin"
                ),
            ],
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-2-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-2-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-2-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-2-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/11000-2-0-edited.bin"
                ),
            ],
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-3-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-3-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-3-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-3-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/11000-3-0-edited.bin"
                ),
            ],
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-4-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-4-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-4-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-4-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/11000-4-0-edited.bin"
                ),
            ],
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-5-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-5-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-5-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/11000-5-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/11000-5-0-edited.bin"
                ),
            ],
        ],
    },
    VersionParameterFixture {
        case: 210,
        version: 18000,
        graphs: [
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-0-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-0-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-0-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-0-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/18000-0-0-edited.bin"
                ),
            ],
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-1-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-1-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-1-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-1-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/18000-1-0-edited.bin"
                ),
            ],
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-2-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-2-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-2-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-2-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/18000-2-0-edited.bin"
                ),
            ],
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-3-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-3-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-3-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-3-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/18000-3-0-edited.bin"
                ),
            ],
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-4-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-4-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-4-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-4-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/18000-4-0-edited.bin"
                ),
            ],
            [
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-5-0.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-5-1.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-5-2.bin"),
                include_bytes!("../fixtures/stored-parameters/version-contexts/18000-5-3.bin"),
                include_bytes!(
                    "../fixtures/stored-parameters/version-contexts/18000-5-0-edited.bin"
                ),
            ],
        ],
    },
];
