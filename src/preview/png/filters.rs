use super::invalid;
use crate::Result;

pub(super) fn unfilter(kind: u8, row: &mut [u8], previous: &[u8], stride: usize) -> Result<()> {
    if kind > 4 {
        return Err(invalid(format!("PNG filter {kind}; expected 0..=4")));
    }
    for index in 0..row.len() {
        let left = if index >= stride {
            row[index - stride]
        } else {
            0
        };
        let up = previous[index];
        let corner = if index >= stride {
            previous[index - stride]
        } else {
            0
        };
        let predictor = predict(kind, left, up, corner);
        row[index] = row[index].wrapping_add(predictor);
    }
    Ok(())
}

fn predict(kind: u8, left: u8, up: u8, corner: u8) -> u8 {
    match kind {
        0 => 0,
        1 => left,
        2 => up,
        3 => ((u16::from(left) + u16::from(up)) / 2) as u8,
        _ => paeth(left, up, corner),
    }
}

fn paeth(left: u8, up: u8, corner: u8) -> u8 {
    let (a, b, c) = (i32::from(left), i32::from(up), i32::from(corner));
    let prediction = a + b - c;
    let distances = (
        (prediction - a).abs(),
        (prediction - b).abs(),
        (prediction - c).abs(),
    );
    if distances.0 <= distances.1 && distances.0 <= distances.2 {
        return left;
    }
    if distances.1 <= distances.2 {
        return up;
    }
    corner
}
