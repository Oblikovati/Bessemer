use super::*;
use std::sync::Arc;

#[test]
fn reserved_sector_is_exclusively_owned_and_rejects_every_chain_role() {
    let mut allocation = boundary_allocation(END, 524288);
    allocation.reserve_range_lock().unwrap();
    allocation.finish().unwrap();
    assert_eq!(
        allocation.owners[524286],
        Some(SectorOwner::RangeLock(true))
    );
    reject_chain_roles(&mut allocation);
    let mut at_boundary = boundary_allocation(FREE, 524287);
    at_boundary.reserve_range_lock().unwrap();
    at_boundary.finish().unwrap();
    assert_eq!(
        at_boundary.owners[524286],
        Some(SectorOwner::RangeLock(false))
    );
    let mut missing = boundary_allocation(FREE, 524288);
    assert!(missing.reserve_range_lock().is_err());
}

fn boundary_allocation(marker: u32, sectors: usize) -> Allocation {
    let header = boundary_header(sectors);
    let mut fat = vec![FREE; sectors];
    fat[524286] = marker;
    Allocation {
        header,
        fat,
        owners: vec![None; sectors],
    }
}

fn reject_chain_roles(allocation: &mut Allocation) {
    for owner in [
        SectorOwner::Directory,
        SectorOwner::MiniFat,
        SectorOwner::MiniStream,
        SectorOwner::Stream(1),
        SectorOwner::Fat,
        SectorOwner::Difat,
    ] {
        assert!(allocation.claim(524286, owner).is_err());
    }
}

fn boundary_header(sectors: usize) -> Header {
    Header {
        raw: Arc::from([]),
        major: 4,
        sector_bytes: 4096,
        sectors,
        directory_start: END,
        directory_count: 0,
        fat_count: 1,
        mini_fat_start: END,
        mini_fat_count: 0,
        difat_start: END,
        difat_count: 0,
    }
}
