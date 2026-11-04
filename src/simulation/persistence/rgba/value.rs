use crate::{Result, simulation::invalid};
use std::sync::Arc;

/// Named component of a stored four-channel COGLColor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationColorChannel {
    /// Red component.
    Red,
    /// Green component.
    Green,
    /// Blue component.
    Blue,
    /// Alpha component.
    Alpha,
}

/// Bitwise RGBA value; no clamping, normalization or native comparison policy is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationRgbaColor([u32; 4]);

impl SimulationRgbaColor {
    /// Preserve four scalar patterns in RGBA order; e.g. `Self::from_bits([0, 0, 0, 0x3f800000])`.
    pub const fn from_bits(bits: [u32; 4]) -> Self {
        Self(bits)
    }

    /// Read exactly sixteen stored bytes; e.g. `Self::from_bytes(&[0; 16])?`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let raw: [u8; 16] = bytes.try_into().map_err(|_| {
            invalid(
                "COGLColor",
                format!("{} bytes; expected exactly 16 RGBA bytes", bytes.len()),
            )
        })?;
        let bits = std::array::from_fn(|index| {
            let start = index * 4;
            u32::from_le_bytes([raw[start], raw[start + 1], raw[start + 2], raw[start + 3]])
        });
        Ok(Self(bits))
    }

    /// Return all original patterns; e.g. `color.bits()`.
    pub const fn bits(self) -> [u32; 4] {
        self.0
    }

    /// Select one named raw component; e.g. `color.channel_bits(SimulationColorChannel::Alpha)`.
    pub fn channel_bits(self, channel: SimulationColorChannel) -> u32 {
        let index = match channel {
            SimulationColorChannel::Red => 0,
            SimulationColorChannel::Green => 1,
            SimulationColorChannel::Blue => 2,
            SimulationColorChannel::Alpha => 3,
        };
        self.0[index]
    }

    /// Return a numeric value only for finite patterns; e.g. `color.finite_channel(SimulationColorChannel::Red)`.
    pub fn finite_channel(self, channel: SimulationColorChannel) -> Option<f32> {
        let value = f32::from_bits(self.channel_bits(channel));
        value.is_finite().then_some(value)
    }

    /// Reconstruct every stored component bit; e.g. `color.to_bytes()`.
    pub fn to_bytes(self) -> [u8; 16] {
        let mut bytes = [0; 16];
        for (index, bits) in self.0.into_iter().enumerate() {
            bytes[index * 4..index * 4 + 4].copy_from_slice(&bits.to_le_bytes());
        }
        bytes
    }
}

/// One named physical color with its absolute body byte range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPostColorView {
    /// Structural color prefix before `.component_0`.
    pub path: Arc<str>,
    /// Complete sixteen-byte span in the supplied body.
    pub wire_range: (usize, usize),
    /// Exact RGBA channel patterns.
    pub color: SimulationRgbaColor,
}
