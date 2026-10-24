use super::{invalid, types::ImageShape};
use crate::{Result, archive_codec::ArchiveCursor};

struct RlePosition {
    x: usize,
    y: usize,
    shape: ImageShape,
    palette: usize,
}

pub(super) fn walk_rle(
    bytes: &[u8],
    shape: ImageShape,
    palette: usize,
    mut raster: Option<&mut [Option<u8>]>,
) -> Result<()> {
    let mut cursor = ArchiveCursor::new(bytes);
    let mut position = RlePosition::new(shape, palette);
    loop {
        let count = cursor.u8()?;
        let value = cursor.u8()?;
        if count != 0 {
            position.run(count as usize, value, &mut raster)?;
            continue;
        }
        match value {
            0 => position.next_row()?,
            1 => return Ok(()),
            2 => position.delta(cursor.u8()?, cursor.u8()?)?,
            count => absolute(&mut cursor, &mut position, count, &mut raster)?,
        }
    }
}

fn absolute(
    cursor: &mut ArchiveCursor<'_>,
    position: &mut RlePosition,
    count: u8,
    raster: &mut Option<&mut [Option<u8>]>,
) -> Result<()> {
    let indices = cursor.take(count as usize)?;
    for index in indices {
        position.run(1, *index, raster)?;
    }
    if !count.is_multiple_of(2) {
        cursor.u8()?;
    }
    Ok(())
}

impl RlePosition {
    fn new(shape: ImageShape, palette: usize) -> Self {
        Self {
            x: 0,
            y: 0,
            shape,
            palette,
        }
    }
    fn run(
        &mut self,
        count: usize,
        index: u8,
        raster: &mut Option<&mut [Option<u8>]>,
    ) -> Result<()> {
        if self.y >= self.shape.height || count > self.shape.width - self.x {
            return Err(invalid(format!(
                "RLE run ({},{}) length {count}; expected within {}x{}",
                self.x, self.y, self.shape.width, self.shape.height
            )));
        }
        if index as usize >= self.palette {
            return Err(invalid(format!(
                "RLE palette index {index}; expected below {}",
                self.palette
            )));
        }
        if let Some(pixels) = raster {
            let start = (self.shape.height - 1 - self.y) * self.shape.width + self.x;
            pixels[start..start + count].fill(Some(index));
        }
        self.x += count;
        Ok(())
    }

    fn next_row(&mut self) -> Result<()> {
        if self.y >= self.shape.height {
            return Err(invalid(format!(
                "RLE EOL at row {}; expected row below {}",
                self.y, self.shape.height
            )));
        }
        self.x = 0;
        self.y += 1;
        Ok(())
    }

    fn delta(&mut self, dx: u8, dy: u8) -> Result<()> {
        let x = self.x + dx as usize;
        let y = self.y + dy as usize;
        if x > self.shape.width || y >= self.shape.height {
            return Err(invalid(format!(
                "RLE delta target ({x},{y}); expected within {}x{}",
                self.shape.width, self.shape.height
            )));
        }
        self.x = x;
        self.y = y;
        Ok(())
    }
}

pub(super) fn encode_indices(indices: &[u8], shape: ImageShape) -> Vec<u8> {
    let mut output = Vec::new();
    for row in indices.chunks_exact(shape.width).rev() {
        encode_row(row, &mut output);
        output.extend_from_slice(&[0, 0]);
    }
    output.extend_from_slice(&[0, 1]);
    output
}

fn encode_row(mut row: &[u8], output: &mut Vec<u8>) {
    while let Some(index) = row.first() {
        let count = row
            .iter()
            .take(255)
            .take_while(|value| *value == index)
            .count();
        output.extend_from_slice(&[count as u8, *index]);
        row = &row[count..];
    }
}
