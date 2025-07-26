//! Physical layer (or PHY), the first part of the networking stack.
//! This the most low-level and responsible of direct interaction with hardware.

use crate::time::Instant;

/// Metadata of a packet.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PacketMetadata {
    pub id: u32,
}

/// Describe a given device's capabilities.
pub struct DeviceCapabilities {
    /// The device's network medium type.
    ///
    /// Determines packet format and interface behavior. For example, ARP/NDISC is only
    /// performed for [`Medium::Ethernet`].
    pub medium: Medium,
    /// Maximum transmission unit (MTU) in octets.  
    ///  
    /// The device cannot send/receive frames larger than this.  
    ///  
    /// For Ethernet, this is the max frame size *including* the 14-octet header  
    /// but *excluding* the 4-octet FCS. Thus, Ethernet MTU = IP MTU + 14.  
    ///  
    /// Note: Linux/other OSes define "MTU" as the IP MTU, even for Ethernet.  
    pub max_transmission_unit: usize,
    /// Maximum burst size (in MTU units).  
    ///  
    /// The device cannot send/receive bursts larger than this.  
    pub max_burst_size: Option<usize>,
    /// Checksum behaviour.
    pub checksum: ChecksumCapabilities,
}

/// Describe the checksum behaviour for each protocol.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ChecksumCapabilities {
    pub ipv4: Checksum,
    pub udp: Checksum,
    pub tcp: Checksum,
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

/// Type of a medium device.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Medium {
    Ethernet,
    Ip,
}

/// Interface for sending and receiving raw frames.
pub trait Device {
    type RxToken<'t>: RxToken
    where
        Self: 't;
    type TxToken<'t>: TxToken
    where
        Self: 't;

    /// Creates a receiving token and a transmit token.
    fn receive(&mut self, timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)>;

    /// Creates a transmit token.
    fn transmit(&mut self, timestamp: Instant) -> Option<Self::TxToken<'_>>;

    /// Describe the device capabilities.
    fn capabilities(&self) -> Option<()>;
}

pub trait RxToken {}

pub trait TxToken {}

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
