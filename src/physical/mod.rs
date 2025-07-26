//! Physical layer (or PHY), the first part of the networking stack.
//! This the most low-level and responsible of direct interaction with hardware.

/// Metadata of a packet.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PacketMetadata {
    pub id: u32,
}

/// Checksum behaviour of a given protocol.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Checksum {
    /// Verify the checksum when receiving and compute it when sending.
    #[default]
    Both,
    /// Ignore checksum.
    None,
    /// Verify the checksum when receiving.
    Rx,
    /// Compute the checksum when sending.
    Tx,
}

impl Checksum {
    /// Returns whether the checksum should be computed when sending.
    pub const fn tx(&self) -> bool {
        match self {
            Self::Both | Self::Tx => true,
            _ => false,
        }
    }

    /// Returns whether the checksum should be verified when receiving.
    pub const fn rx(&self) -> bool {
        match self {
            Self::Both | Self::Rx => true,
            _ => false,
        }
    }
}
