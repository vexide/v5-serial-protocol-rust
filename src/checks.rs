/// Packet decoding checks
use bitflags::bitflags;

bitflags! {
    /// These flags determine what checks recieve_extended will perform
    /// on the recieved packet.
    pub struct VexExtPacketChecks: u8 {
        /// Bit 1 requires that we check the ACK value
        const ACK = 1 << 0;
        /// Bit 2 requires that we check the CRC
        const CRC = 1 << 1;
        /// Bit 3 requires that we check the Length of the packet
        const LENGTH = 1 << 2;
    }
}