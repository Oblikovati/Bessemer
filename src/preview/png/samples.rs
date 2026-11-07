use super::{
    PngContents, PngHeader, PngRaster, filters,
    header::sample_count,
    passes::{Pass, image_passes, row_bytes},
};
use crate::{Result, archive_codec::ArchiveCursor};

pub(super) fn decode_samples(bytes: &[u8], header: PngHeader) -> Result<PngRaster> {
    let mut samples = vec![0; sample_count(header)?];
    let mut cursor = ArchiveCursor::new(bytes);
    for pass in image_passes(header) {
        decode_pass(&mut cursor, header, pass, &mut samples)?;
    }
    cursor.finish()?;
    Ok(PngRaster { header, samples })
}

fn decode_pass(
    cursor: &mut ArchiveCursor<'_>,
    header: PngHeader,
    pass: Pass,
    samples: &mut [u16],
) -> Result<()> {
    let size = row_bytes(header, pass.width)?;
    let mut previous = vec![0; size];
    let stride = (header.color.channels() * header.bit_depth as usize).div_ceil(8);
    for y in 0..pass.height {
        let filter = cursor.u8()?;
        let mut row = cursor.take(size)?.to_vec();
        filters::unfilter(filter, &mut row, &previous, stride)?;
        copy_row(&row, header, pass, y, samples);
        previous = row;
    }
    Ok(())
}

fn copy_row(row: &[u8], header: PngHeader, pass: Pass, y: usize, samples: &mut [u16]) {
    let channels = header.color.channels();
    for x in 0..pass.width {
        let target =
            ((pass.y + y * pass.dy) * header.width as usize + pass.x + x * pass.dx) * channels;
        for channel in 0..channels {
            samples[target + channel] = read_sample(row, x * channels + channel, header.bit_depth);
        }
    }
}

fn read_sample(row: &[u8], index: usize, depth: u8) -> u16 {
    if depth == 16 {
        return u16::from_be_bytes([row[index * 2], row[index * 2 + 1]]);
    }
    if depth == 8 {
        return u16::from(row[index]);
    }
    let bit = index * depth as usize;
    u16::from((row[bit / 8] >> (8 - depth - bit as u8 % 8)) & ((1 << depth) - 1))
}

pub(super) fn encode_samples(raster: &PngRaster) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    for pass in image_passes(raster.header) {
        for y in 0..pass.height {
            output.push(0);
            output.extend(encode_row(raster, pass, y)?);
        }
    }
    Ok(output)
}

fn encode_row(raster: &PngRaster, pass: Pass, y: usize) -> Result<Vec<u8>> {
    let header = raster.header;
    let mut row = vec![0; row_bytes(header, pass.width)?];
    let channels = header.color.channels();
    for x in 0..pass.width {
        let source =
            ((pass.y + y * pass.dy) * header.width as usize + pass.x + x * pass.dx) * channels;
        for channel in 0..channels {
            write_sample(
                &mut row,
                x * channels + channel,
                header.bit_depth,
                raster.samples[source + channel],
            );
        }
    }
    Ok(row)
}

fn write_sample(row: &mut [u8], index: usize, depth: u8, value: u16) {
    if depth == 16 {
        row[index * 2..index * 2 + 2].copy_from_slice(&value.to_be_bytes());
        return;
    }
    if depth == 8 {
        row[index] = value as u8;
        return;
    }
    let bit = index * depth as usize;
    row[bit / 8] |= (value as u8) << (8 - depth - bit as u8 % 8);
}

pub(super) fn payload(contents: &PngContents, name: [u8; 4]) -> Option<&[u8]> {
    contents
        .chunks
        .iter()
        .find(|chunk| chunk.kind.bytes() == name)
        .map(|chunk| chunk.payload.as_slice())
}
