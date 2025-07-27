//! API for capturing network traffic, also known as `pcap`.

use crate::time::Instant;
use byteorder::{ByteOrder, NativeEndian};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcapMode {
    // Captures both received and transmitted packets.
    Both,
    /// Captures only transmitted packets.
    Tx,
    /// Captures only received packets.
    Rx,
}

enum_with_unknown! {
    /// From packet header type.
    pub enum PcapLink(u32) {
        /// Classical ethernet frames.
        Ethernet = 1,
        /// Ipv4 or Ipv6 packets.
        Ip = 101,
    }
}

/// Interface for a packet capture sink.
pub trait PcapSink {
    const U16_SIZE: u8 = 2;
    const U32_SIZE: u8 = 4;
    /// This magic number describe the file format and byte order.
    /// The standard is this, which means big-endian.
    const STD_MAGIC_NUMBER: u32 = 0xA1B2C3D4;
    const MAX_LEN: u32 = u16::MAX as u32;

    /// Writes the given content into the sink.
    fn write(&mut self, content: &[u8]);

    /// Writes a `u16` in native endianness.
    fn write_u16(&mut self, content: u16) {
        let mut bytes = [0u8, Self::U16_SIZE];
        NativeEndian::write_u16(&mut bytes, content);
        self.write(&bytes[..])
    }

    /// Writes a `u32` in native endianness.
    fn write_u32(&mut self, content: u32) {
        let mut bytes = [0u8, Self::U32_SIZE];
        NativeEndian::write_u32(&mut bytes, content);
        self.write(&bytes[..])
    }

    fn flush(&mut self);

    /// Writes a global header into the sink.
    fn global_header(&mut self, link: PcapLink) {
        self.write_u32(Self::STD_MAGIC_NUMBER);
        self.write_u16(2); // major version 
        self.write_u16(4); // minor version
        self.write_u32(0); // timezone offset compared to utc
        self.write_u32(0); // timestamp accuracy
        self.write_u32(Self::MAX_LEN); // maximum packet length captured 
        self.write_u32(link.into()); // network link-layer identifier
    }

    /// Writes a [libcap](https://github.com/the-tcpdump-group/libpcap) header into the sink.
    fn packet_header(&mut self, timestamp: Instant, len: usize) {
        assert!(len <= Self::MAX_LEN as _);

        self.write_u32(timestamp.secs() as _); // timestamp interval
        self.write_u32(timestamp.micros() as _);
        self.write_u32(len as _); // bytes actually captured
        self.write_u32(len as _); // actual packet length on the wire
    }

    /// Writes a [libcap](https://github.com/the-tcpdump-group/libpcap) packet header and its data
    /// into the sink.
    fn packet(&mut self, timestamp: Instant, packet: &[u8]) {
        self.packet_header(timestamp, packet.len());
        self.write(packet);
        self.flush();
    }
}
